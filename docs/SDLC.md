# TERMINATOR — SDLC Documentation

## 1. Planning & Requirements

### 1.1 Product Vision
A retro 90s sci-fi terminal AI that runs 100% offline on Mac.
Users speak or type to an AI that responds through a neon CRT interface.
One model (Gemma 4 E2B) handles text, audio, and vision.
MMS-TTS provides multilingual voice output.

### 1.2 Functional Requirements
| ID    | Requirement                                      | Priority | Status |
|-------|--------------------------------------------------|----------|--------|
| FR-01 | Text chat with AI via keyboard                   | P0       | ✅     |
| FR-02 | Voice input via push-to-talk (Space key)          | P0       | ✅     |
| FR-03 | Retro CRT terminal UI with neon theme             | P0       | ✅     |
| FR-04 | 100% offline operation                            | P0       | ✅     |
| FR-05 | Multilingual support (140+ languages)             | P1       | ✅     |
| FR-06 | Streaming token-by-token response display         | P1       | ✅     |
| FR-07 | Boot sequence animation                           | P2       | ✅     |
| FR-08 | Conversation history within session               | P1       | ✅     |
| FR-09 | Toggle voice/text mode (Tab key)                  | P1       | ✅     |
| FR-10 | Voice output (MMS-TTS, multilingual)              | P1       | ✅     |
| FR-11 | Live oscilloscope waveform during recording       | P2       | ✅     |
| FR-12 | Agentic tool calling (open/read/list/run/vision)  | P1       | ✅     |
| FR-13 | Security approval popup for tool actions           | P0       | ✅     |
| FR-14 | Image analysis via Gemma 4 vision encoder          | P1       | ✅     |
| FR-15 | Voice transcription display                        | P1       | ✅     |
| FR-16 | Recording timer with auto-stop at 28s              | P1       | ✅     |

### 1.3 Non-Functional Requirements
| ID     | Requirement                                     | Target   | Actual  |
|--------|-------------------------------------------------|----------|---------|
| NFR-01 | First token latency                             | < 2s     | ~1.5s   |
| NFR-02 | RAM usage                                       | < 5GB    | ~4.3GB  |
| NFR-03 | Binary size (excl. model)                       | < 20MB   | 1.2MB   |
| NFR-04 | Startup time (excl. model load)                 | < 1s     | < 0.1s  |
| NFR-05 | Model load time                                 | < 10s    | ~2s     |
| NFR-06 | Platform                                        | macOS M1+| ✅      |

### 1.4 Technical Architecture
```
┌─────────────────────────────────────────────────┐
│                 terminator (Rust)                │
│                                                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐  │
│  │ Ratatui  │  │  cpal    │  │ Python Bridge│  │
│  │ (TUI)    │  │ (audio)  │  │ (subprocess) │  │
│  └────┬─────┘  └────┬─────┘  └──────┬───────┘  │
│       │              │               │          │
│       └──────────────┴───────────────┘          │
│                      │                          │
│              ┌───────┴───────┐                  │
│              │   App State   │                  │
│              │   Machine     │                  │
│              └───────────────┘                  │
└─────────────────────────────────────────────────┘
                       │
                       ▼ (subprocess stdin/stdout JSON)
┌─────────────────────────────────────────────────┐
│            inference.py (Python)                 │
│                                                 │
│  HuggingFace Transformers                       │
│  ├── Gemma 4 E2B (text + audio + vision)        │
│  │   ├── Text encoder (140+ languages)          │
│  │   ├── Audio encoder (~300M params, 16kHz)    │
│  │   ├── Vision encoder (~150M params)          │
│  │   └── Function calling (5 tools)             │
│  └── MMS-TTS (voice output per language)        │
│      ├── facebook/mms-tts-eng (~145MB)          │
│      └── facebook/mms-tts-tha (~145MB)          │
└─────────────────────────────────────────────────┘
```

### 1.5 Data Flow
```
Voice path:
  🎤 → cpal (48kHz) → resample 16kHz → base64 → Python bridge
  → Gemma 4 audio encoder → transcribe → respond/tool call
  → MMS-TTS → afplay 🔊

Text path:
  ⌨️ → Ratatui input → Python bridge
  → Gemma 4 text → respond/tool call → MMS-TTS → afplay 🔊

Tool path:
  Gemma 4 → tool_call JSON → Rust shows WARNING popup
  → [Y] approve → execute (open/read/list/run/vision)
  → result → Gemma 4 → response
  → [N] reject → Gemma 4 acknowledges
```

### 1.6 State Machine
```
Boot → Loading → Idle ⇄ Recording
                  ↓         ↓
              Processing ← (stop)
                  ↓
              Streaming → Idle
                  ↓
          AwaitingApproval
            ↓           ↓
        [Y] Approve  [N] Reject
            ↓           ↓
         Execute     Notify AI
            ↓           ↓
        Processing → Streaming → Idle
```

---

## 2. Design

### 2.1 Module Design

| Module      | File          | Responsibility                          |
|-------------|---------------|-----------------------------------------|
| main        | main.rs       | Entry point, event loop, terminal setup |
| app         | app.rs        | State machine, tool execution, approval |
| ui          | ui.rs         | TUI rendering, waveform, popup          |
| audio       | audio.rs      | Mic capture, resample, level tracking   |
| bridge      | bridge.rs     | Python subprocess, JSON protocol        |
| theme       | theme.rs      | Colors, borders, boot text              |

### 2.2 Python Bridge Protocol (JSON over stdin/stdout)
```json
// Requests (Rust → Python)
{"type": "text", "content": "Hello"}
{"type": "audio", "data": "<base64 PCM 16kHz mono f32>"}
{"type": "tool_result", "tool": "read_file", "result": "...", "approved": true}
{"type": "reset"}

// Responses (Python → Rust)
{"type": "ready"}
{"type": "transcript", "content": "what user said"}
{"type": "token", "content": "word "}
{"type": "tool_call", "tool": "open_file", "args": {"path": "~/Downloads/x.pdf"}}
{"type": "done"}
{"type": "error", "message": "..."}
```

### 2.3 Tool Definitions
| Tool             | Args                    | Executed by |
|------------------|-------------------------|-------------|
| `open_file`      | `path`                  | Rust (macOS `open`) |
| `read_file`      | `path`                  | Rust (`fs::read_to_string`) |
| `list_directory` | `path`                  | Rust (`fs::read_dir`) |
| `run_command`    | `command`               | Rust (`sh -c`) |
| `analyze_image`  | `path`, `question`      | Python (Gemma 4 vision) |

---

## 3. Testing

### 3.1 Test Strategy
| Level       | Scope                          | Tool          |
|-------------|--------------------------------|---------------|
| Unit        | State machine, theme, parsing  | cargo test    |
| Integration | Bridge protocol, audio pipeline| cargo test    |
| Manual/E2E  | Full app with model            | Human tester  |

### 3.2 Test Cases
| ID   | Test                                    | Type        | Status |
|------|-----------------------------------------|-------------|--------|
| T-01 | Bridge sends text, receives tokens      | Integration | ✅     |
| T-02 | Bridge handles malformed JSON           | Integration | ✅     |
| T-03 | Audio capture produces valid PCM buffer | Integration | ✅     |
| T-04 | PCM base64 roundtrip                    | Unit        | ✅     |
| T-05 | Tool approval/rejection flow            | Manual      | ✅     |
| T-06 | Voice input → transcription → response  | Manual      | ✅     |
| T-07 | Image analysis via vision encoder       | Manual      | ✅     |
| T-08 | TTS plays and stops on exit             | Manual      | ✅     |

---

## 4. Deployment & Release

### 4.1 Build
```bash
cargo build --release
```

### 4.2 Release Checklist
- [x] All tests pass (`cargo test`)
- [x] Clippy clean (`cargo clippy`)
- [x] README with architecture, setup, controls
- [x] Model download script works
- [x] Voice input/output working
- [x] Tool calling with approval popup
- [x] Image analysis working
- [x] TTS cleanup on exit
- [x] Binary size < 20MB (actual: 1.2MB)
- [x] RAM < 5GB (actual: ~4.3GB)

### 4.3 Distribution
```
terminator/
├── target/release/terminator   # Rust binary (~1.2MB)
├── scripts/inference.py        # Python inference + TTS
├── scripts/download_model.py   # Model downloader
└── requirements.txt            # Python deps
```
User downloads model on first run (~5GB).
MMS-TTS models download on first use (~145MB per language).

---

## 5. Maintenance

### 5.1 Update Path
- Model upgrades: update `download_model.py` MODEL_ID
- New tools: add to TOOLS list in `inference.py` + `execute_tool()` in `app.rs`
- New TTS languages: auto-downloaded on first use via MMS-TTS
- UI changes: modify `ui.rs`, rebuild Rust binary

### 5.2 Known Limitations
- Audio max 30 seconds per utterance (Gemma 4 limit)
- Requires Python 3.11+ installed
- macOS only (Apple Silicon)
- No conversation persistence across sessions
- MMS-TTS voice quality is functional but not human-like
- Tool calling depends on prompt engineering (may occasionally misfire)
