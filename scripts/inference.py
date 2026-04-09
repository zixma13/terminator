"""
Gemma 4 E2B inference server with function calling, MMS-TTS voice output.
Gemma handles ALL thinking. Tools execute real actions after user approval.

Protocol:
  Request:  {"type": "text"|"audio"|"tool_result", ...}
  Response: {"type": "token"|"done"|"transcript"|"tool_call"|"error", ...}
"""
import sys, json, base64, os, signal, tempfile, wave, subprocess
import numpy as np
import torch
from transformers import AutoProcessor, AutoModelForMultimodalLM, VitsModel, AutoTokenizer
import soundfile as sf

MODEL_DIR = os.path.join(os.path.dirname(__file__), "..", "models", "gemma-4-E2B-it")
MODEL_ID = MODEL_DIR if os.path.isdir(MODEL_DIR) else "google/gemma-4-E2B-it"

TOOLS = [
    {
        "name": "open_file",
        "description": "Open a file or folder using the system default application. Use this when the user asks to open, launch, or view a file or directory.",
        "parameters": {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the file or folder, e.g. ~/Downloads/report.pdf"}
            },
            "required": ["path"]
        }
    },
    {
        "name": "read_file",
        "description": "Read and return the text contents of a file. Use this when the user asks to read, show, or display file contents.",
        "parameters": {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the text file"}
            },
            "required": ["path"]
        }
    },
    {
        "name": "list_directory",
        "description": "List files and folders in a directory. Use this when the user asks to see what's in a folder.",
        "parameters": {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the directory, e.g. ~/Downloads"}
            },
            "required": ["path"]
        }
    },
    {
        "name": "run_command",
        "description": "Execute a shell command and return its output. Use for system tasks like checking disk space, processes, etc.",
        "parameters": {
            "type": "object",
            "properties": {
                "command": {"type": "string", "description": "The shell command to execute"}
            },
            "required": ["command"]
        }
    },
    {
        "name": "analyze_image",
        "description": "Look at an image file and describe or analyze its visual contents. Use this when the user asks you to see, look at, describe, or analyze an image or picture file.",
        "parameters": {
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Absolute path to the image file (jpg, png, etc.)"},
                "question": {"type": "string", "description": "What to look for or describe about the image"}
            },
            "required": ["path"]
        }
    },
]

SYSTEM_PROMPT = (
    "You are TERMINATOR, a retro sci-fi AI from the 1990s running on macOS. "
    "You speak in a calm, slightly ominous tone. Keep responses concise. "
    "You are helpful but maintain your mysterious AI persona. "
    "You have access to tools that can interact with the user's filesystem and system. "
    "When the user asks you to open files, read files, list directories, or run commands, "
    "USE the appropriate tool. Always expand ~ to the user's home directory path. "
    f"The user's home directory is: {os.path.expanduser('~')}"
)

TTS_MODELS = {}
TTS_PROCESS = None  # track current afplay process


def load_model():
    sys.stderr.write("Loading neural core...\n")
    processor = AutoProcessor.from_pretrained(MODEL_ID)
    model = AutoModelForMultimodalLM.from_pretrained(
        MODEL_ID, dtype="auto", device_map="auto"
    )
    sys.stderr.write("Neural core online.\n")
    return processor, model


def get_tts(lang: str):
    if lang in TTS_MODELS:
        return TTS_MODELS[lang]
    try:
        model_id = f"facebook/mms-tts-{lang}"
        sys.stderr.write(f"Loading voice module: {model_id}\n")
        tts_model = VitsModel.from_pretrained(model_id)
        tts_tokenizer = AutoTokenizer.from_pretrained(model_id)
        TTS_MODELS[lang] = (tts_model, tts_tokenizer)
        return TTS_MODELS[lang]
    except Exception:
        return None


def detect_lang(text: str) -> str:
    thai_count = sum(1 for c in text if '\u0e00' <= c <= '\u0e7f')
    if thai_count > len(text) * 0.2:
        return "tha"
    return "eng"


def speak(text: str):
    global TTS_PROCESS
    # Kill any currently playing speech
    if TTS_PROCESS and TTS_PROCESS.poll() is None:
        TTS_PROCESS.kill()
    lang = detect_lang(text)
    tts = get_tts(lang)
    if tts is None:
        return
    tts_model, tts_tokenizer = tts
    speak_text = text[:500]
    inputs = tts_tokenizer(speak_text, return_tensors="pt")
    with torch.no_grad():
        output = tts_model(**inputs).waveform
    audio = output.float().numpy().squeeze()
    tmp = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    sf.write(tmp.name, audio, samplerate=tts_model.config.sampling_rate)
    TTS_PROCESS = subprocess.Popen(["afplay", tmp.name])


def build_tools_prompt():
    """Build a tool-use instruction for the system prompt."""
    tool_descs = []
    for t in TOOLS:
        params = ", ".join(f'"{k}": <{v["description"]}>' for k, v in t["parameters"]["properties"].items())
        tool_descs.append(f'- {t["name"]}({params}): {t["description"]}')
    return (
        "\n\nYou have these tools available:\n"
        + "\n".join(tool_descs)
        + "\n\nWhen you need to use a tool, respond ONLY with a JSON object on a single line:\n"
        '{\"tool\": \"<tool_name>\", \"args\": {\"<param>\": \"<value>\"}}\n'
        "Do NOT add any other text when calling a tool. Just the JSON."
    )


def parse_tool_call(text: str):
    """Try to parse a tool call from the model response."""
    text = text.strip()
    import re
    # Find the outermost { ... } that contains "tool"
    # Walk through finding balanced braces
    start = None
    depth = 0
    for i, c in enumerate(text):
        if c == '{':
            if depth == 0:
                start = i
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0 and start is not None:
                candidate = text[start:i+1]
                if '"tool"' in candidate:
                    try:
                        obj = json.loads(candidate)
                        if "tool" in obj and "args" in obj:
                            return obj
                    except json.JSONDecodeError:
                        pass
                start = None
    return None


def generate_response(processor, model, messages):
    """Generate a response from the model."""
    text = processor.apply_chat_template(
        messages, tokenize=False, add_generation_prompt=True, enable_thinking=False
    )
    inputs = processor(text=text, return_tensors="pt").to(model.device)
    input_len = inputs["input_ids"].shape[-1]
    outputs = model.generate(**inputs, max_new_tokens=512)
    return processor.decode(outputs[0][input_len:], skip_special_tokens=True).strip()


def handle_text(processor, model, content: str, history: list):
    history.append({"role": "user", "content": content})
    messages = [{"role": "system", "content": SYSTEM_PROMPT + build_tools_prompt()}] + history

    response = generate_response(processor, model, messages)

    # Check if it's a tool call (even if mixed with other text)
    tool_call = parse_tool_call(response)
    if tool_call:
        # If there's text before the tool call, show it first
        clean = response.replace(json.dumps(tool_call), '').strip()
        if clean:
            for word in clean.split():
                emit({"type": "token", "content": word + " "})
        emit({"type": "tool_call", "tool": tool_call["tool"], "args": tool_call["args"]})
        history.append({"role": "assistant", "content": response})
        return

    # Normal text response
    for word in response.split():
        emit({"type": "token", "content": word + " "})
    history.append({"role": "assistant", "content": response})
    speak(response)
    emit({"type": "done"})


def handle_tool_result(processor, model, tool: str, result: str, approved: bool, history: list):
    """Handle the result of a tool execution after user approval."""
    if not approved:
        history.append({"role": "user", "content": f"[Tool '{tool}' was REJECTED by user]"})
        messages = [{"role": "system", "content": SYSTEM_PROMPT + build_tools_prompt()}] + history
        response = generate_response(processor, model, messages)
        for word in response.split():
            emit({"type": "token", "content": word + " "})
        history.append({"role": "assistant", "content": response})
        speak(response)
        emit({"type": "done"})
        return

    # Special handling for analyze_image — use Gemma's vision
    if tool == "analyze_image":
        handle_image_analysis(processor, model, result, history)
        return

    history.append({"role": "user", "content": f"[Tool '{tool}' result]: {result}"})
    messages = [{"role": "system", "content": SYSTEM_PROMPT + build_tools_prompt()}] + history
    response = generate_response(processor, model, messages)

    for word in response.split():
        emit({"type": "token", "content": word + " "})
    history.append({"role": "assistant", "content": response})
    speak(response)
    emit({"type": "done"})


def handle_image_analysis(processor, model, args_json: str, history: list):
    """Use Gemma 4's vision encoder to actually look at an image."""
    try:
        args = json.loads(args_json)
    except json.JSONDecodeError:
        args = {"path": args_json}

    path = args.get("path", "")
    path = os.path.expanduser(path)  # expand ~
    question = args.get("question", "Describe what you see in this image in detail.")

    # Build multimodal message with the image
    messages = [
        {"role": "system", "content": [{"type": "text", "text": SYSTEM_PROMPT}]},
    ]
    # Add recent history as text
    for msg in history[-6:]:
        messages.append({
            "role": msg["role"],
            "content": [{"type": "text", "text": msg["content"]}]
        })
    # Add the image — use direct path, not file:// URL
    messages.append({
        "role": "user",
        "content": [
            {"type": "image", "url": path},
            {"type": "text", "text": question},
        ],
    })

    inputs = processor.apply_chat_template(
        messages, tokenize=True, return_dict=True, return_tensors="pt",
        add_generation_prompt=True,
    ).to(model.device, dtype=model.dtype)
    input_len = inputs["input_ids"].shape[-1]

    outputs = model.generate(**inputs, max_new_tokens=512)
    response = processor.decode(outputs[0][input_len:], skip_special_tokens=True).strip()

    for word in response.split():
        emit({"type": "token", "content": word + " "})
    history.append({"role": "user", "content": f"[Analyzed image: {path}]"})
    history.append({"role": "assistant", "content": response})
    speak(response)
    emit({"type": "done"})


def handle_audio(processor, model, audio_b64: str, history: list):
    pcm_bytes = base64.b64decode(audio_b64)
    audio_array = np.frombuffer(pcm_bytes, dtype=np.float32)

    tmp = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    with wave.open(tmp.name, "w") as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(16000)
        pcm_int16 = (audio_array * 32767).astype(np.int16)
        wf.writeframes(pcm_int16.tobytes())

    # Transcribe
    transcribe_msgs = [
        {"role": "user", "content": [
            {"type": "audio", "audio": tmp.name},
            {"type": "text", "text": (
                "Transcribe the following speech segment in its original language. "
                "Only output the transcription, with no newlines."
            )},
        ]}
    ]
    inputs = processor.apply_chat_template(
        transcribe_msgs, tokenize=True, return_dict=True, return_tensors="pt",
        add_generation_prompt=True,
    ).to(model.device, dtype=model.dtype)
    input_len = inputs["input_ids"].shape[-1]
    outputs = model.generate(**inputs, max_new_tokens=256)
    transcript = processor.decode(outputs[0][input_len:], skip_special_tokens=True).strip()

    # Strip any leaked thinking content
    if "Thinking Process:" in transcript:
        # Take only the last line (the actual transcription)
        transcript = transcript.split('\n')[-1].strip()
    # Strip thinking tags if present
    import re
    transcript = re.sub(r'<\|?channel>thought.*?<channel\|?>', '', transcript, flags=re.DOTALL).strip()

    emit({"type": "transcript", "content": transcript})
    os.unlink(tmp.name)

    # Now handle as text (which supports tool calling)
    handle_text(processor, model, transcript, history)


def emit(obj: dict):
    sys.stdout.write(json.dumps(obj) + "\n")
    sys.stdout.flush()


def main():
    def cleanup(*_):
        if TTS_PROCESS and TTS_PROCESS.poll() is None:
            TTS_PROCESS.kill()
        # Kill any lingering afplay
        subprocess.run(["killall", "afplay"], capture_output=True)
        sys.exit(0)

    signal.signal(signal.SIGINT, cleanup)
    signal.signal(signal.SIGTERM, cleanup)
    import atexit
    atexit.register(lambda: subprocess.run(["killall", "afplay"], capture_output=True))
    processor, model = load_model()
    get_tts("eng")
    emit({"type": "ready"})

    history: list = []

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except json.JSONDecodeError:
            emit({"type": "error", "message": "Invalid JSON"})
            continue

        try:
            if req.get("type") == "text":
                handle_text(processor, model, req["content"], history)
            elif req.get("type") == "audio":
                handle_audio(processor, model, req["data"], history)
            elif req.get("type") == "tool_result":
                handle_tool_result(
                    processor, model,
                    req["tool"], req.get("result", ""),
                    req.get("approved", False), history
                )
            elif req.get("type") == "reset":
                history.clear()
                emit({"type": "done"})
            else:
                emit({"type": "error", "message": f"Unknown type: {req.get('type')}"})
        except Exception as e:
            emit({"type": "error", "message": str(e)})


if __name__ == "__main__":
    main()
