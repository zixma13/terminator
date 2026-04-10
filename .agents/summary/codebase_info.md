# Codebase Information

## Project

- **Name**: Terminator
- **Version**: 1.0.0
- **License**: Apache-2.0
- **Description**: A retro 90s sci-fi terminal AI — talk to a real AI through a neon CRT interface

## Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Frontend / TUI | Rust + Ratatui + Crossterm | CRT-style terminal UI, keyboard event loop |
| Audio capture | cpal | Microphone PCM capture (16kHz mono) |
| Serialization | serde + serde_json | JSON protocol between Rust ↔ Python |
| AI inference | Python + HuggingFace Transformers | Gemma 4 E2B multimodal model |
| TTS | MMS-TTS (VitsModel) | Multilingual speech synthesis |
| Audio encoding | base64 | PCM audio transport over JSON |

## Languages

| Language | Files | Role |
|----------|-------|------|
| Rust | 6 source + 2 test | Application binary, TUI, audio, bridge |
| Python | 2 scripts | AI inference server, model downloader |
| Markdown | 14 docs | README translations, SDLC |

## Repository Layout

```
terminator/
├── src/                    # Rust source
│   ├── main.rs             # Entry point, event loop
│   ├── app.rs              # State machine, tool execution
│   ├── ui.rs               # Ratatui TUI rendering
│   ├── audio.rs            # Mic capture via cpal
│   ├── bridge.rs           # Python subprocess bridge (JSON)
│   └── theme.rs            # CRT/neon color palette
├── scripts/
│   ├── inference.py        # Gemma 4 inference + tool calling + TTS
│   └── download_model.py   # Model weight downloader
├── tests/
│   ├── test_bridge.rs      # Bridge protocol tests
│   └── test_audio.rs       # Audio pipeline tests
├── docs/                   # README translations (12 languages)
├── models/                 # Downloaded model weights (gitignored)
├── Cargo.toml              # Rust dependencies
└── requirements.txt        # Python dependencies
```
