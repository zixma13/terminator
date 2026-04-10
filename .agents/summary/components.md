# Components

## Rust Components

### main.rs — Entry Point, Model Picker & Event Loop
- `select_model()`: Ratatui TUI screen for choosing Gemma 4 variant (E2B/E4B/26B-A4B/31B). Arrow keys + Enter, number keys for quick select, Esc to quit.
- `model_status()`: Returns `(is_ready, status_text)` — checks for `config.json` + `.safetensors` weight files. Shows `[✓ READY]`, `[↓ 4.9/9 GB]` (partial download), or `[download]`.
- `dir_size()`: Recursive directory size calculation for download progress display.
- `run()`: Main event loop — render UI → tick state → poll keyboard events.
- Cleans up on exit (kills `afplay` TTS processes).

### app.rs — State Machine & Tool Execution
- **`State` enum**: `Booting → Loading → Idle → Recording → Processing → Streaming → AwaitingApproval`
- **`App` struct**: All application state including `model_id`, `loading_pct` (fake progress bar)
- **`model_display()`**: Returns formatted model name for UI (e.g. `"GEMMA-4-E4B"`)
- **`check_ready()`**: Polls for Python `ready` signal, ticks fake progress (fast to 60%, slow to 92%, stalls until ready)
- **Tool execution**: `execute_tool()` handles 5 tools with `~` expansion (even inside quotes) and quoting hints on failure
- **Truncation**: `read_file`/`run_command` capped at 2000 chars; `list_directory` at 50 entries

### ui.rs — TUI Rendering
- `draw_header()`: Shows model name dynamically (e.g. `NEURAL CORE: GEMMA-4-E4B`)
- Boot lines use `{}` placeholder replaced with `app.model_display()`
- Retro progress bar: `[████████████████░░░░░░░░░░░░░░] 53%`
- `draw_approval_popup()`: Red WARNING popup for tool approval
- `render_waveform()`: Live oscilloscope during voice recording

### audio.rs — Microphone Capture
- `AudioCapture`: Wraps cpal for PCM recording at 16kHz mono
- `encode_base64()`: PCM f32 → 16-bit LE → base64

### bridge.rs — Python Subprocess Bridge
- **`MODELS`**: Static array of `ModelInfo` (id, name, ram, size) for all 4 variants
- **`Bridge::spawn(model_id)`**: Passes `--model` arg to `inference.py`
- Background reader thread parses JSON from Python stdout into `mpsc::channel`

### theme.rs — Visual Theme
- CRT neon color palette constants
- `BOOT_LINES` / `BOOT_READY`: Boot text templates with `{}` placeholder for model name

## Python Components

### inference.py — AI Inference Server
- **`resolve_model()`**: Reads `--model` arg, resolves local `models/` path or HuggingFace ID
- Supports all 4 Gemma 4 variants (E2B, E4B, 26B-A4B, 31B)
- Tool descriptions include quoting hints for file paths with spaces
- `handle_text()`, `handle_audio()`, `handle_tool_result()`, `handle_image_analysis()`
- `speak()` + `get_tts()`: MMS-TTS synthesis, plays via `afplay`

### download_model.py — Multi-Model Downloader
- `MODELS` dict with all 4 variants (repo ID, dirname, expected size)
- Interactive prompt or CLI arg: `python3 download_model.py E4B`
- Supports `ALL` to download everything
- Skips already-downloaded models
