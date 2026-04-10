use crate::audio::{AudioCapture, AudioLevel};
use crate::bridge::{Bridge, Request, Response};
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Booting,
    Loading,
    Idle,
    Recording,
    Processing,
    Streaming,
    AwaitingApproval,
}

#[derive(Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Pending tool call awaiting user approval.
#[derive(Clone)]
pub struct PendingTool {
    pub tool: String,
    pub args: serde_json::Value,
    pub display: String, // human-readable description
}

pub struct App {
    pub state: State,
    pub input: String,
    pub messages: Vec<ChatMessage>,
    pub current_response: String,
    pub voice_mode: bool,
    pub boot_step: usize,
    pub loading_pct: u8,
    pub should_quit: bool,
    pub status: String,
    pub recording_start: Option<Instant>,
    pub audio_level: AudioLevel,
    pub pending_tool: Option<PendingTool>,
    pub model_id: String,
    pub bridge: Bridge,
    pub audio: AudioCapture,
}

const MAX_RECORD_SECS: u64 = 28;

impl App {
    pub fn new(bridge: Bridge, audio: AudioCapture, audio_level: AudioLevel, model_id: String) -> Self {
        Self {
            state: State::Booting,
            input: String::new(),
            messages: Vec::new(),
            current_response: String::new(),
            voice_mode: false,
            boot_step: 0,
            loading_pct: 0,
            should_quit: false,
            status: "BOOTING...".into(),
            recording_start: None,
            audio_level,
            pending_tool: None,
            model_id,
            bridge,
            audio,
        }
    }

    /// Model display name for UI (e.g. "GEMMA-4-E4B")
    pub fn model_display(&self) -> String {
        format!("GEMMA-4-{}", self.model_id.to_uppercase())
    }

    pub fn tick_boot(&mut self) {
        self.boot_step += 1;
        if self.boot_step >= crate::theme::BOOT_LINES.len() {
            self.state = State::Loading;
            self.status = "LOADING NEURAL CORE...".into();
        }
    }

    pub fn check_ready(&mut self) {
        if let Ok(Response::Ready) = self.bridge.rx.try_recv() {
            self.loading_pct = 100;
            self.state = State::Idle;
            self.status = "ONLINE — Type or press [SPACE] to speak".into();
        } else {
            // Fake progress: fast to 60%, slow to 92%, stall there
            if self.loading_pct < 60 {
                self.loading_pct = self.loading_pct.saturating_add(3);
            } else if self.loading_pct < 92 {
                self.loading_pct = self.loading_pct.saturating_add(1);
            }
            self.status = format!("LOADING NEURAL CORE... {}%", self.loading_pct);
        }
    }

    pub fn submit_text(&mut self) {
        let content = self.input.trim().to_string();
        if content.is_empty() {
            return;
        }
        self.messages.push(ChatMessage {
            role: "user".into(),
            content: content.clone(),
        });
        self.input.clear();
        self.current_response.clear();
        self.state = State::Processing;
        self.status = "PROCESSING...".into();
        let _ = self.bridge.send(&Request::Text { content });
    }

    pub fn start_recording(&mut self) {
        if self.audio.start(&self.audio_level).is_ok() {
            self.state = State::Recording;
            self.recording_start = Some(Instant::now());
            self.status = "🎤 RECORDING 0s / 28s — [SPACE] to send".into();
        }
    }

    pub fn stop_recording(&mut self) {
        self.recording_start = None;
        self.audio_level.lock().unwrap().iter_mut().for_each(|v| *v = 0.0);
        let pcm = self.audio.stop();
        if pcm.is_empty() {
            self.state = State::Idle;
            self.status = "ONLINE — No audio captured".into();
            return;
        }
        self.messages.push(ChatMessage {
            role: "user".into(),
            content: "🎤 [voice input]".into(),
        });
        self.current_response.clear();
        self.state = State::Processing;
        self.status = "PROCESSING AUDIO...".into();
        let data = AudioCapture::encode_base64(&pcm);
        let _ = self.bridge.send(&Request::Audio { data });
    }

    pub fn tick_recording(&mut self) {
        if let Some(start) = self.recording_start {
            let elapsed = start.elapsed().as_secs();
            self.status = format!("🎤 RECORDING {elapsed}s / {MAX_RECORD_SECS}s — [SPACE] to send");
            if elapsed >= MAX_RECORD_SECS {
                self.stop_recording();
            }
        }
    }

    pub fn poll_tokens(&mut self) {
        loop {
            match self.bridge.rx.try_recv() {
                Ok(Response::Transcript { content }) => {
                    if let Some(msg) = self.messages.last_mut() {
                        if msg.role == "user" && msg.content.contains("[voice input]") {
                            msg.content = format!("🎤 {content}");
                        }
                    }
                    self.status = "TRANSCRIBED — GENERATING RESPONSE...".into();
                }
                Ok(Response::ToolCall { tool, args }) => {
                    let display = match tool.as_str() {
                        "open_file" => format!("OPEN: {}", args["path"].as_str().unwrap_or("?")),
                        "read_file" => format!("READ: {}", args["path"].as_str().unwrap_or("?")),
                        "list_directory" => format!("LIST: {}", args["path"].as_str().unwrap_or("?")),
                        "run_command" => format!("EXEC: {}", args["command"].as_str().unwrap_or("?")),
                        "analyze_image" => format!("VISION: {}", args["path"].as_str().unwrap_or("?")),
                        _ => format!("{tool}: {args}"),
                    };
                    self.pending_tool = Some(PendingTool {
                        tool: tool.clone(),
                        args: args.clone(),
                        display,
                    });
                    self.state = State::AwaitingApproval;
                    self.status = "⚠ AWAITING APPROVAL — [Y] Approve  [N] Reject".into();
                }
                Ok(Response::Token { content }) => {
                    self.state = State::Streaming;
                    self.status = "TRANSMITTING...".into();
                    self.current_response.push_str(&content);
                }
                Ok(Response::Done) => {
                    if !self.current_response.is_empty() {
                        self.messages.push(ChatMessage {
                            role: "ai".into(),
                            content: self.current_response.clone(),
                        });
                    }
                    self.current_response.clear();
                    self.state = State::Idle;
                    self.status = "ONLINE — Type or press [SPACE] to speak".into();
                    break;
                }
                Ok(Response::Error { message }) => {
                    self.messages.push(ChatMessage {
                        role: "ai".into(),
                        content: format!("[ERROR] {message}"),
                    });
                    self.current_response.clear();
                    self.state = State::Idle;
                    self.status = "ERROR — See above".into();
                    break;
                }
                _ => break,
            }
        }
    }

    /// User approved the pending tool call — execute it for real.
    pub fn approve_tool(&mut self) {
        let Some(tool) = self.pending_tool.take() else { return };

        self.messages.push(ChatMessage {
            role: "ai".into(),
            content: format!("⚡ [APPROVED] {}", tool.display),
        });

        // Execute the tool
        let result = execute_tool(&tool.tool, &tool.args);

        self.state = State::Processing;
        self.status = "EXECUTING...".into();
        let _ = self.bridge.send(&Request::ToolResult {
            tool: tool.tool,
            result,
            approved: true,
        });
    }

    /// User rejected the pending tool call.
    pub fn reject_tool(&mut self) {
        let Some(tool) = self.pending_tool.take() else { return };

        self.messages.push(ChatMessage {
            role: "ai".into(),
            content: format!("🚫 [REJECTED] {}", tool.display),
        });

        self.state = State::Processing;
        self.status = "PROCESSING REJECTION...".into();
        let _ = self.bridge.send(&Request::ToolResult {
            tool: tool.tool,
            result: String::new(),
            approved: false,
        });
    }

    pub fn toggle_voice(&mut self) {
        self.voice_mode = !self.voice_mode;
        self.status = if self.voice_mode {
            "VOICE MODE — Tap [SPACE] to speak".into()
        } else {
            "TEXT MODE — Type and press [ENTER]".into()
        };
    }
}

/// Execute a tool and return the result string.
fn execute_tool(tool: &str, args: &serde_json::Value) -> String {
    match tool {
        "open_file" => {
            let path = args["path"].as_str().unwrap_or("");
            let path = shellexpand(path);
            match Command::new("open").arg(&path).spawn() {
                Ok(_) => format!("Opened: {path}"),
                Err(e) => format!("Failed to open: {e}"),
            }
        }
        "read_file" => {
            let path = args["path"].as_str().unwrap_or("");
            let path = shellexpand(path);
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    // Truncate to avoid huge payloads
                    if content.len() > 2000 {
                        format!("{}...\n[truncated, {} bytes total]", &content[..2000], content.len())
                    } else {
                        content
                    }
                }
                Err(e) => format!("Failed to read: {e}"),
            }
        }
        "list_directory" => {
            let path = args["path"].as_str().unwrap_or("");
            let path = shellexpand(path);
            match std::fs::read_dir(&path) {
                Ok(entries) => {
                    let items: Vec<String> = entries
                        .filter_map(|e| e.ok())
                        .take(50)
                        .map(|e| {
                            let name = e.file_name().to_string_lossy().to_string();
                            if e.path().is_dir() { format!("{name}/") } else { name }
                        })
                        .collect();
                    items.join("\n")
                }
                Err(e) => format!("Failed to list: {e}"),
            }
        }
        "run_command" => {
            let cmd = args["command"].as_str().unwrap_or("");
            // Expand ~ (even inside quotes) and ensure proper shell execution
            let cmd = cmd.replace("\"~/", &format!("\"{}/", std::env::var("HOME").unwrap_or_default()));
            let cmd = cmd.replace("'~/", &format!("'{}/", std::env::var("HOME").unwrap_or_default()));
            let cmd = cmd.replace("~/", &format!("{}/", std::env::var("HOME").unwrap_or_default()));
            match Command::new("sh").arg("-c").arg(&cmd).output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let mut result = stdout.to_string();
                    if !stderr.is_empty() {
                        result.push_str(&format!("\nSTDERR: {stderr}"));
                    }
                    if !output.status.success() && result.trim().is_empty() {
                        result = format!("Command failed (exit {}). HINT: If file paths contain spaces, wrap them in quotes.", output.status.code().unwrap_or(-1));
                    }
                    if result.len() > 2000 {
                        result.truncate(2000);
                        result.push_str("...[truncated]");
                    }
                    result
                }
                Err(e) => format!("Failed to execute: {e}"),
            }
        }
        "analyze_image" => {
            // Pass args as JSON — Python handles vision inference
            serde_json::to_string(args).unwrap_or_default()
        }
        _ => format!("Unknown tool: {tool}"),
    }
}

/// Expand ~ to home directory.
fn shellexpand(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return format!("{}{}", home.to_string_lossy(), &path[1..]);
        }
    }
    path.to_string()
}
