use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

/// Test that the Python bridge starts and responds to a basic protocol exchange.
/// Requires: Python 3 + transformers installed (skipped in CI if unavailable).
#[test]
#[ignore] // requires model downloaded — run with `cargo test -- --ignored`
fn test_bridge_text_roundtrip() {
    let mut child = Command::new("python3")
        .arg("scripts/inference.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn inference.py");

    let stdin = child.stdin.as_mut().unwrap();
    let stdout = BufReader::new(child.stdout.as_mut().unwrap());

    // Wait for ready
    let mut lines = stdout.lines();
    let ready = lines.next().unwrap().unwrap();
    assert!(ready.contains("\"type\":\"ready\"") || ready.contains("\"type\": \"ready\""));

    // Send text request
    writeln!(stdin, r#"{{"type":"text","content":"Say hello"}}"#).unwrap();
    stdin.flush().unwrap();

    // Should receive at least one token then done
    let mut got_token = false;
    let mut got_done = false;
    for line in lines {
        let line = line.unwrap();
        if line.contains("\"type\":\"token\"") || line.contains("\"type\": \"token\"") {
            got_token = true;
        }
        if line.contains("\"type\":\"done\"") || line.contains("\"type\": \"done\"") {
            got_done = true;
            break;
        }
    }
    assert!(got_token, "Expected at least one token");
    assert!(got_done, "Expected done signal");

    child.kill().ok();
}

/// Test that the bridge handles invalid JSON gracefully.
#[test]
#[ignore]
fn test_bridge_invalid_json() {
    let mut child = Command::new("python3")
        .arg("scripts/inference.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn inference.py");

    let stdin = child.stdin.as_mut().unwrap();
    let stdout = BufReader::new(child.stdout.as_mut().unwrap());
    let mut lines = stdout.lines();

    // Wait for ready
    let _ = lines.next();

    // Send garbage
    writeln!(stdin, "not json at all").unwrap();
    stdin.flush().unwrap();

    let resp = lines.next().unwrap().unwrap();
    assert!(resp.contains("error"), "Expected error response for invalid JSON");

    child.kill().ok();
}
