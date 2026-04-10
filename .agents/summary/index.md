# Terminator — Documentation Index

> Primary entry point for AI assistants. Start here.

## Quick Summary

Two-process system: Rust TUI ↔ Python AI (Gemma 4). Supports 4 model variants (E2B/E4B/26B-A4B/31B) selected at startup via Ratatui picker. Communication via JSON-lines over stdin/stdout. All tool actions require user approval.

## Documentation Files

| File | When to Read |
|------|-------------|
| [architecture.md](architecture.md) | System design, state machine, protocol overview, model variants |
| [components.md](components.md) | What each file does, function-level detail |
| [interfaces.md](interfaces.md) | JSON protocol schemas, tool API, path handling, model selection interface |
| [data_models.md](data_models.md) | Rust structs/enums, Python data structures |
| [workflows.md](workflows.md) | Sequence diagrams: startup, text, voice, tool approval, TTS |
| [dependencies.md](dependencies.md) | Rust crates, Python packages, AI models, system deps |
| [review_notes.md](review_notes.md) | Known gaps and recommendations |

## Key Entry Points

| Task | Where to Look |
|------|--------------|
| Add a new model variant | `bridge.rs` MODELS + `download_model.py` MODELS + `inference.py` resolve_model() + `main.rs` model_status() |
| Add a new tool | `inference.py` TOOLS list + `app.rs` execute_tool() + poll_tokens() ToolCall match |
| Change the UI | `ui.rs` draw functions |
| Fix path handling | `app.rs` execute_tool() — `~` expansion + quoting |
| Modify boot sequence | `theme.rs` BOOT_LINES/BOOT_READY (use `{}` for model name) |
