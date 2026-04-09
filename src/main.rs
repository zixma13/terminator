mod app;
mod audio;
mod bridge;
mod theme;
mod ui;

use anyhow::Result;
use app::{App, State};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;
use std::time::Duration;

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let bridge = bridge::Bridge::spawn()?;
    let (audio, audio_level) = audio::AudioCapture::new()?;
    let mut app = App::new(bridge, audio, audio_level);

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
