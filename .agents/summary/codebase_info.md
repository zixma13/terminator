# Codebase Information

## Project

- **Name**: Terminator
- **Version**: 1.0.0
- **License**: Apache-2.0
- **Description**: A retro 90s sci-fi terminal AI — talk to a real AI through a retro terminal interface

## Technology Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Frontend / TUI | Rust + Ratatui + Crossterm | Retro terminal UI, keyboard event loop, model picker |
| Audio capture | cpal | Microphone PCM capture (16kHz mono) |
| Serialization | serde + serde_json | JSON protocol between Rust ↔ Python |
| AI inference | Python + HuggingFace Transformers | Gemma 4 multimodal models (E2B/E4B/26B-A4B/31B) |
| TTS | MMS-TTS (VitsModel) | Multilingual speech synthesis |
| Audio encoding | base64 | PCM audio transport over JSON |

## Languages

| Language | Files | Role |
|----------|-------|------|
| Rust | 6 source + 2 test | Application binary, TUI, audio, bridge, model picker |
| Python | 2 scripts | AI inference server, model downloader |
| Markdown | 14 docs | README translations (12 languages), SDLC |

## Repository Layout

```
terminator/
├── src/
│   ├── main.rs             # Entry point, model picker, event loop
│   ├── app.rs              # State machine, tool execution, loading progress
│   ├── ui.rs               # Ratatui TUI rendering
│   ├── audio.rs            # Mic capture via cpal
│   ├── bridge.rs           # Python subprocess bridge, model definitions
│   └── theme.rs            # CRT/neon color palette, boot text templates
├── scripts/
│   ├── inference.py        # Gemma 4 inference + tool calling + TTS
│   └── download_model.py   # Multi-model weight downloader
├── tests/
│   ├── test_bridge.rs      # Bridge protocol tests
│   └── test_audio.rs       # Audio pipeline tests
├── docs/
│   ├── screenshots/        # Demo GIF
│   └── README.*.md         # Translations (12 languages)
├── models/                 # Downloaded model weights (gitignored)
├── Cargo.toml
└── requirements.txt
```
