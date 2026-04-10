# Interfaces

## Bridge JSON Protocol

The primary interface is the JSON-lines protocol between Rust and Python over stdin/stdout.

### Request Schema (Rust → Python)

```mermaid
classDiagram
    class Request {
        <<enum>>
    }
    class Text {
        +type: "text"
        +content: String
    }
    class Audio {
        +type: "audio"
        +data: String (base64)
    }
    class ToolResult {
        +type: "tool_result"
        +tool: String
        +result: String
        +approved: bool
    }
    class Reset {
        +type: "reset"
    }
    Request <|-- Text
    Request <|-- Audio
    Request <|-- ToolResult
    Request <|-- Reset
```

### Response Schema (Python → Rust)

```mermaid
classDiagram
    class Response {
        <<enum>>
    }
    class Ready {
        +type: "ready"
    }
    class Transcript {
        +type: "transcript"
        +content: String
    }
    class Token {
        +type: "token"
        +content: String
    }
    class ToolCallResp {
        +type: "tool_call"
        +tool: String
        +args: Object
    }
    class Done {
        +type: "done"
    }
    class ErrorResp {
        +type: "error"
        +message: String
    }
    Response <|-- Ready
    Response <|-- Transcript
    Response <|-- Token
    Response <|-- ToolCallResp
    Response <|-- Done
    Response <|-- ErrorResp
```

## Tool Calling Interface

### Available Tools

| Tool | Parameters | Returns |
|------|-----------|---------|
| `open_file` | `path: String` | Success/failure message |
| `read_file` | `path: String` | File contents (truncated at 2000 chars) |
| `list_directory` | `path: String` | Newline-separated entries (max 50) |
| `run_command` | `command: String` | stdout + stderr (truncated at 2000 chars) |
| `analyze_image` | `path: String`, `question: String` | JSON args passed to Python vision |

### Tool Call Flow

```mermaid
sequenceDiagram
    participant U as User
    participant R as Rust App
    participant P as Python/Gemma

    U->>R: "List files in ~/Downloads"
    R->>P: {"type":"text","content":"List files in ~/Downloads"}
    P->>R: {"type":"tool_call","tool":"list_directory","args":{"path":"~/Downloads"}}
    R->>U: ⚠ Approval popup
    U->>R: Press Y
    R->>R: execute_tool("list_directory", {"path":"~/Downloads"})
    R->>P: {"type":"tool_result","tool":"list_directory","result":"file1.txt\nfile2.pdf","approved":true}
    P->>R: {"type":"token","content":"Here are..."} (streaming)
    P->>R: {"type":"done"}
```

## Audio Interface

- **Format**: PCM 16kHz mono float32
- **Transport**: Base64-encoded 16-bit LE over JSON
- **Max duration**: 28 seconds (enforced by Rust)
- **TTS output**: MMS-TTS generates WAV, played via macOS `afplay`
