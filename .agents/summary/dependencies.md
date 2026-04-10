# Dependencies

## Rust (Cargo.toml)

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratatui` | 0.29 | TUI framework (model picker + main UI) |
| `crossterm` | 0.28 | Terminal manipulation (raw mode, events) |
| `cpal` | 0.15 | Audio capture (microphone) |
| `serde` | 1 | JSON serialization |
| `serde_json` | 1 | JSON parsing |
| `base64` | 0.22 | PCM audio encoding |
| `anyhow` | 1 | Error handling |

## Python (requirements.txt)

| Package | Version | Purpose |
|---------|---------|---------|
| `transformers` | ≥4.52.0 | Model loading and inference |
| `torch` | ≥2.6.0 | Tensor computation, GPU |
| `torchvision` | ≥0.20.0 | Image processing for vision |
| `accelerate` | ≥1.6.0 | Model loading optimization |
| `librosa` | ≥0.10.0 | Audio processing |
| `huggingface_hub` | ≥0.30.0 | Model downloading |
| `pillow` | ≥10.0.0 | Image loading |
| `soundfile` | ≥0.13.0 | Audio file I/O |

## AI Models

| Model | Source | Size |
|-------|--------|------|
| Gemma 4 E2B | `google/gemma-4-E2B-it` | ~5 GB |
| Gemma 4 E4B | `google/gemma-4-E4B-it` | ~9 GB |
| Gemma 4 26B-A4B | `google/gemma-4-26B-A4B-it` | ~16 GB |
| Gemma 4 31B | `google/gemma-4-31B-it` | ~20 GB |
| MMS-TTS | `facebook/mms-tts-{lang}` | ~145 MB/lang |

## System Dependencies

| Dependency | Purpose |
|-----------|---------|
| macOS + Apple Silicon | Metal GPU acceleration, `afplay` for TTS |
| Python 3.11+ | Inference subprocess |
| Rust 1.75+ | Build the binary |
