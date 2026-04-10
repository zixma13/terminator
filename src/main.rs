mod app;
mod audio;
mod bridge;
mod theme;
mod ui;

use anyhow::Result;
use app::{App, State};
use bridge::MODELS;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;
use std::time::Duration;

/// Check which models are downloaded locally.
fn is_downloaded(model_id: &str) -> bool {
    let cwd = std::env::current_dir().unwrap_or_default();
    let variants = [
        ("E2B", "gemma-4-E2B-it"),
        ("E4B", "gemma-4-E4B-it"),
        ("26B-A4B", "gemma-4-26B-A4B-it"),
        ("31B", "gemma-4-31B-it"),
    ];
    for (id, dir) in variants {
        if id == model_id {
            return cwd.join("models").join(dir).is_dir();
        }
    }
    false
}

/// Show model selection menu before TUI starts.
fn select_model() -> Result<String> {
    // Use green ANSI for the retro feel
    let green = "\x1b[32m";
    let dim = "\x1b[2m";
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";
    let amber = "\x1b[33m";

    println!("{green}╔══════════════════════════════════════════════════╗");
    println!("║  {bold}TERMINATOR — MODEL SELECT{reset}{green}                       ║");
    println!("╠══════════════════════════════════════════════════╣{reset}");

    for (i, m) in MODELS.iter().enumerate() {
        let downloaded = if is_downloaded(m.id) {
            format!("{green}[✓ READY]{reset}")
        } else {
            format!("{dim}[download]{reset}")
        };
        println!(
            "{green}║{reset}  {bold}{}{reset}. {:<20} RAM: {:<8} Disk: {:<8} {}  {green}║{reset}",
            i + 1, m.name, m.ram, m.size, downloaded
        );
    }

    println!("{green}╠══════════════════════════════════════════════════╣");
    println!("║{reset}  {amber}Select model [1-4] or Enter for default (1):{reset}    {green}║");
    println!("╚══════════════════════════════════════════════════╝{reset}");

    print!("{green}> {reset}");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let choice = input.trim();

    let idx = if choice.is_empty() {
        0
    } else {
        match choice.parse::<usize>() {
            Ok(n) if n >= 1 && n <= MODELS.len() => n - 1,
            _ => {
                println!("{amber}Invalid choice, using default (E2B){reset}");
                0
            }
        }
    };

    let selected = MODELS[idx].id.to_string();
    println!(
        "\n{green}▶ Loading {bold}{}{reset}{green} ...{reset}\n",
        MODELS[idx].name
    );

    Ok(selected)
}

fn main() -> Result<()> {
    let model_id = select_model()?;

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let bridge = bridge::Bridge::spawn(&model_id)?;
    let (audio, audio_level) = audio::AudioCapture::new()?;
    let mut app = App::new(bridge, audio, audio_level, model_id);

    let result = run(&mut terminal, &mut app);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    // Kill any lingering TTS audio
    let _ = std::process::Command::new("killall").arg("afplay").output();

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        match app.state {
            State::Booting => app.tick_boot(),
            State::Loading => app.check_ready(),
            State::Recording => app.tick_recording(),
            State::Processing | State::Streaming => app.poll_tokens(),
            _ => {}
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        app.should_quit = true;
                    }
                    // Approval popup
                    KeyCode::Char('y') if app.state == State::AwaitingApproval => {
                        app.approve_tool();
                    }
                    KeyCode::Char('n') if app.state == State::AwaitingApproval => {
                        app.reject_tool();
                    }
                    KeyCode::Tab if app.state == State::Idle => {
                        app.toggle_voice();
                    }
                    // Voice mode: Space toggles recording on/off
                    KeyCode::Char(' ') if app.voice_mode && app.state == State::Idle => {
                        app.start_recording();
                    }
                    KeyCode::Char(' ') if app.voice_mode && app.state == State::Recording => {
                        app.stop_recording();
                    }
                    // Text mode
                    KeyCode::Enter if !app.voice_mode && app.state == State::Idle => {
                        app.submit_text();
                    }
                    KeyCode::Backspace if app.state == State::Idle && !app.voice_mode => {
                        app.input.pop();
                    }
                    KeyCode::Char(c) if app.state == State::Idle && !app.voice_mode => {
                        app.input.push(c);
                    }
                    _ => {}
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
