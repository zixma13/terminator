# Terminator — Documentation Index

> This file is the primary entry point for AI assistants working with the Terminator codebase.
> Start here to understand the system and find detailed information in linked documents.

## How to Use This Index

1. **Read this file first** — it contains summaries of every documentation file
2. **Follow links** to specific files only when you need deeper detail
3. **For architecture questions** → `architecture.md`
4. **For "how does X work"** → `workflows.md`
5. **For data structures** → `data_models.md`
6. **For API/protocol details** → `interfaces.md`
7. **For "what does file X do"** → `components.md`
8. **For dependency info** → `dependencies.md`

## Quick System Summary

Terminator is a **two-process system**: a Rust TUI binary communicates with a Python AI inference subprocess via JSON-lines over stdin/stdout. The Rust side handles UI, audio capture, keyboard events, and tool execution. The Python side runs Gemma 4 E2B for text/audio/vision inference and MMS-TTS for speech output. All tool actions require explicit user approval via a popup.

## Documentation Files

| File | Purpose | Key Contents |
|------|---------|-------------|
| [codebase_info.md](codebase_info.md) | Project metadata | Tech stack, languages, repo layout |
| [architecture.md](architecture.md) | System design | Two-process architecture, state machine diagram, bridge protocol overview, design patterns |
| [components.md](components.md) | Component details | What each file does — `main.rs`, `app.rs`, `ui.rs`, `audio.rs`, `bridge.rs`, `theme.rs`, `inference.py`, `download_model.py` |
| [interfaces.md](interfaces.md) | APIs & protocols | Full JSON protocol schema (Request/Response), tool calling interface, audio format spec, tool call sequence diagram |
| [data_models.md](data_models.md) | Data structures | Rust enums/structs (`State`, `App`, `ChatMessage`, `PendingTool`, `Bridge`), Python conversation history format, tool definition schema |
| [workflows.md](workflows.md) | Process flows | Startup sequence, text input flow, voice input flow, tool approval flow, TTS output flow — all with sequence diagrams |
| [dependencies.md](dependencies.md) | External deps | Rust crates, Python packages, AI models, system requirements |
| [review_notes.md](review_notes.md) | Quality review | Consistency check results, completeness gaps, improvement recommendations |

## Key Entry Points

| Task | Start At |
|------|----------|
| Understand the overall system | `architecture.md` → System Overview |
| Modify the UI | `components.md` → ui.rs section |
| Add a new tool | `interfaces.md` → Tool Calling Interface, then `app.rs` `execute_tool()` + `inference.py` `TOOLS` list |
| Change the AI model | `components.md` → inference.py, `dependencies.md` → AI Models |
| Debug bridge communication | `interfaces.md` → Bridge JSON Protocol |
| Understand state transitions | `architecture.md` → State Machine |
