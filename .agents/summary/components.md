# Components

## Rust Components

### main.rs — Entry Point & Event Loop
- Initializes terminal (raw mode, alternate screen)
- Spawns `Bridge` (Python subprocess) and `AudioCapture`
- Runs the main loop: render UI → tick state → poll keyboard events
- Cleans up on exit (kills `afplay` TTS processes)

### app.rs — State Machine & Tool Execution
- **`State` enum**: `Booting → Loading → Idle → Recording → Processing → Streaming → AwaitingApproval`
- **`App` struct**: Holds all application state (messages, input buffer, bridge, audio, pending tool)
- **Tool execution**: `execute_tool()` handles `open_file`, `read_file`, `list_directory`, `run_command`, `analyze_image`
- **Security**: Tool calls pause in `AwaitingApproval` state until user presses Y/N
- **Truncation**: `read_file` and `run_command` results capped at 2000 chars; `list_directory` capped at 50 entries

### ui.rs — TUI Rendering
- `draw()` dispatches to sub-renderers based on state
- `draw_header()`: ASCII-style header with system status
- `draw_chat()`: Scrollable message history with role-based coloring
- `draw_input()`: Text input field or voice mode indicator
- `draw_status()`: Bottom status bar
- `draw_recording_input()` + `render_waveform()`: Live oscilloscope visualization during recording
- `draw_approval_popup()`: Centered warning popup for tool approval

### audio.rs — Microphone Capture
- **`AudioCapture`**: Wraps cpal for PCM recording at 16kHz mono
- `start()`: Begins capturing audio samples into a shared buffer
- `stop()`: Returns captured PCM samples as `Vec<f32>`
- `encode_base64()`: Converts PCM f32 samples to 16-bit LE bytes, then base64
- Shared `AudioLevel` (`Arc<Mutex<Vec<f32>>>`) feeds the oscilloscope waveform display

### bridge.rs — Python Subprocess Bridge
- **`Bridge`**: Spawns `scripts/inference.py` via `.venv/bin/python3` (falls back to system `python3`)
- Communicates via JSON-lines over stdin (requests) / stdout (responses)
- Background reader thread parses JSON from Python stdout into `mpsc::channel`
- `wait_ready()`: Blocks until Python sends `{"type": "ready"}`
- `Drop` impl kills the child process on cleanup

### theme.rs — Visual Theme
- CRT neon color palette: `PRIMARY` (neon green), `DIM`, `ACCENT` (amber), `ERROR` (red), `USER_COLOR` (cyan)
- `BOOT_LINES` and `BOOT_READY`: Boot sequence text displayed during startup animation

## Python Components

### inference.py — AI Inference Server
- Loads Gemma 4 E2B model via HuggingFace Transformers
- Reads JSON requests from stdin, writes JSON responses to stdout
- **`handle_text()`**: Processes text input with tool-calling system prompt
- **`handle_audio()`**: Processes base64 PCM audio through Gemma's audio encoder
- **`handle_tool_result()`**: Feeds tool execution results back into conversation
- **`handle_image_analysis()`**: Loads image from disk, runs through Gemma's vision encoder
- **`parse_tool_call()`**: Extracts structured tool calls from model output
- **`speak()`** + **`get_tts()`**: MMS-TTS speech synthesis, plays via `afplay`
- **`detect_lang()`**: Simple language detection for TTS voice selection

### download_model.py — Model Downloader
- Downloads Gemma 4 E2B weights from HuggingFace Hub to `models/` directory
