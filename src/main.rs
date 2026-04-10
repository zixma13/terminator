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
use ratatui::layout::{Constraint, Layout, Rect, Alignment};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io::stdout;
use std::time::Duration;
use theme::{PRIMARY, DIM, ACCENT, BG, BORDER, ERROR};

/// Check download status of a model. Returns (is_ready, status_text).
fn model_status(model_id: &str) -> (bool, String) {
    let cwd = std::env::current_dir().unwrap_or_default();
    let variants = [
        ("E2B", "gemma-4-E2B-it", 5_000_000_000u64),
        ("E4B", "gemma-4-E4B-it", 9_000_000_000u64),
        ("26B-A4B", "gemma-4-26B-A4B-it", 16_000_000_000u64),
        ("31B", "gemma-4-31B-it", 20_000_000_000u64),
    ];
    for (id, dir, expected) in variants {
        if id == model_id {
            let model_dir = cwd.join("models").join(dir);
            if !model_dir.exists() {
                return (false, "[download]".into());
            }
            // Check for weight files
            let has_weights = model_dir.read_dir().ok().map_or(false, |entries| {
                entries.filter_map(|e| e.ok()).any(|e| {
                    let n = e.file_name().to_string_lossy().to_string();
                    n.ends_with(".safetensors") || n.ends_with(".bin")
                })
            });
            if has_weights && model_dir.join("config.json").exists() {
                return (true, "[✓ READY]".into());
            }
            // Partial download — calculate size
            let size = dir_size(&model_dir);
            if size > 0 {
                let gb = size as f64 / 1_000_000_000.0;
                let exp_gb = expected as f64 / 1_000_000_000.0;
                return (false, format!("[↓ {:.1}/{:.0} GB]", gb, exp_gb));
            }
            return (false, "[download]".into());
        }
    }
    (false, "[download]".into())
}

fn dir_size(path: &std::path::Path) -> u64 {
    std::fs::read_dir(path).ok().map_or(0, |entries| {
        entries.filter_map(|e| e.ok()).map(|e| {
            let meta = e.metadata().unwrap_or_else(|_| std::fs::metadata(e.path()).unwrap());
            if meta.is_dir() { dir_size(&e.path()) } else { meta.len() }
        }).sum()
    })
}

/// Ratatui-based model selection screen.
fn select_model(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<Option<String>> {
    let mut selected: usize = 0;

    loop {
        terminal.draw(|f| {
            let area = f.area();

            // Center the picker vertically
            let vert = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(MODELS.len() as u16 + 8),
                    Constraint::Min(0),
                ])
                .split(area);

            // Center horizontally
            let horiz = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(68),
                    Constraint::Min(0),
                ])
                .split(vert[1]);

            let picker_area = horiz[1];

            let mut lines: Vec<Line> = Vec::new();
            lines.push(Line::from(""));

            for (i, m) in MODELS.iter().enumerate() {
                let is_sel = i == selected;
                let (ready, status) = model_status(m.id);

                let marker = if is_sel { "▶ " } else { "  " };

                let style = if is_sel {
                    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(PRIMARY)
                };
                let status_style = if ready {
                    Style::default().fg(PRIMARY)
                } else if status.starts_with("[↓") {
                    Style::default().fg(ACCENT)
                } else {
                    Style::default().fg(DIM)
                };

                lines.push(Line::from(vec![
                    Span::styled(format!(" {marker}{}. ", i + 1), style),
                    Span::styled(format!("{:<18}", m.name), style),
                    Span::styled(format!("RAM: {:<7} Disk: {:<7}", m.ram, m.size), Style::default().fg(DIM)),
                    Span::styled(format!(" {status}"), status_style),
                ]));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                " [↑↓] select  [Enter] confirm  [Esc] quit",
                Style::default().fg(DIM),
            )));

            let picker = Paragraph::new(lines)
                .block(
                    Block::default()
                        .title(Span::styled(
                            " TERMINATOR — MODEL SELECT ",
                            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(BORDER)),
                )
                .style(Style::default().bg(BG));

            f.render_widget(picker, picker_area);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected + 1 < MODELS.len() {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        return Ok(Some(MODELS[selected].id.to_string()));
                    }
                    KeyCode::Esc => {
                        return Ok(None);
                    }
                    KeyCode::Char('1') => return Ok(Some(MODELS[0].id.to_string())),
                    KeyCode::Char('2') => return Ok(Some(MODELS[1].id.to_string())),
                    KeyCode::Char('3') => return Ok(Some(MODELS[2].id.to_string())),
                    KeyCode::Char('4') => return Ok(Some(MODELS[3].id.to_string())),
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Model picker
    let model_id = match select_model(&mut terminal)? {
        Some(id) => id,
        None => {
            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen)?;
            return Ok(());
        }
    };

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
