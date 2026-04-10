use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;

/// Available Gemma 4 model variants.
pub struct ModelInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub ram: &'static str,
    pub size: &'static str,
}

pub const MODELS: &[ModelInfo] = &[
    ModelInfo { id: "E2B",     name: "Gemma 4 E2B",     ram: "~4 GB", size: "~5 GB" },
    ModelInfo { id: "E4B",     name: "Gemma 4 E4B",     ram: "~8 GB", size: "~9 GB" },
    ModelInfo { id: "26B-A4B", name: "Gemma 4 26B-A4B",  ram: "~18 GB", size: "~16 GB" },
    ModelInfo { id: "31B",     name: "Gemma 4 31B",     ram: "~20 GB", size: "~20 GB" },
];

/// Messages sent to the Python inference process.
#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Request {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "audio")]
    Audio { data: String },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool: String,
        result: String,
        approved: bool,
    },
    #[serde(rename = "reset")]
    Reset,
}

/// Messages received from the Python inference process.
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Response {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "transcript")]
    Transcript { content: String },
    #[serde(rename = "token")]
    Token { content: String },
    #[serde(rename = "tool_call")]
    ToolCall {
        tool: String,
        args: serde_json::Value,
    },
    #[serde(rename = "done")]
    Done,
    #[serde(rename = "error")]
    Error { message: String },
}

/// Bridge to the Python inference subprocess.
pub struct Bridge {
    child: Child,
    stdin: std::process::ChildStdin,
    pub rx: mpsc::Receiver<Response>,
}

impl Bridge {
    /// Spawn the Python inference process and start reading its output.
    pub fn spawn(model_id: &str) -> Result<Self> {
        // Resolve paths relative to the working directory
        let cwd = std::env::current_dir().unwrap_or_default();
        let script = cwd.join("scripts/inference.py");
        let venv_python = cwd.join(".venv/bin/python3");
        let python = if venv_python.exists() {
            venv_python
        } else {
            std::path::PathBuf::from("python3")
        };

        let mut child = Command::new(&python)
            .arg(&script)
            .arg("--model")
            .arg(model_id)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn Python inference process. Is Python 3 installed?")?;

        let stdin = child.stdin.take().context("Failed to open stdin")?;
        let stdout = child.stdout.take().context("Failed to open stdout")?;

        let (tx, rx) = mpsc::channel();

        // Reader thread: parse JSON lines from Python stdout
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                let Ok(line) = line else { break };
                if line.trim().is_empty() {
                    continue;
                }
                if let Ok(resp) = serde_json::from_str::<Response>(&line) {
                    if tx.send(resp).is_err() {
                        break;
                    }
                }
            }
        });

        Ok(Self { child, stdin, rx })
    }

    /// Send a request to the Python process.
    pub fn send(&mut self, req: &Request) -> Result<()> {
        let json = serde_json::to_string(req)?;
        writeln!(self.stdin, "{json}")?;
        self.stdin.flush()?;
        Ok(())
    }

    /// Wait for the Ready signal (model loaded).
    pub fn wait_ready(&self) -> Result<()> {
        loop {
            match self.rx.recv() {
                Ok(Response::Ready) => return Ok(()),
                Ok(Response::Error { message }) => {
                    anyhow::bail!("Inference error during init: {message}")
                }
                Err(_) => anyhow::bail!("Inference process died during init"),
                _ => continue,
            }
        }
    }
}

impl Drop for Bridge {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
