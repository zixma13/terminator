use crate::app::{App, State};
use crate::theme::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(10),   // chat
            Constraint::Length(3), // input
            Constraint::Length(1), // status bar
        ])
        .split(f.area());

    draw_header(f, chunks[0], app);
    draw_chat(f, chunks[1], app);
    draw_input(f, chunks[2], app);
    draw_status(f, chunks[3], app);

    // Approval popup overlay
    if app.state == State::AwaitingApproval {
        if let Some(ref tool) = app.pending_tool {
            draw_approval_popup(f, f.area(), tool);
        }
    }
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" TERMINATOR ", Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)),
        Span::styled("OS v1.0.0 ", Style::default().fg(DIM)),
        Span::styled("│ ", Style::default().fg(BORDER)),
        Span::styled("NEURAL CORE: ", Style::default().fg(DIM)),
        Span::styled(app.model_display(), Style::default().fg(ACCENT)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER))
            .title_style(Style::default().fg(PRIMARY)),
    );
    f.render_widget(header, area);
}

fn draw_chat(f: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();

    // Boot sequence
    if app.state == State::Booting || app.state == State::Loading {
        for (i, line) in BOOT_LINES.iter().enumerate() {
            if i < app.boot_step {
                let text = line.replace("{}", &app.model_display());
                lines.push(Line::from(Span::styled(text, Style::default().fg(PRIMARY))));
            }
        }
        if app.state == State::Loading {
            for line in BOOT_READY {
                let text = line.replace("{}", &app.model_display());
                lines.push(Line::from(Span::styled(text, Style::default().fg(PRIMARY))));
            }
            // Retro progress bar
            let pct = app.loading_pct as usize;
            let filled = pct * 30 / 100;
            let empty = 30 - filled;
            let bar = format!(
                "  [{}{}] {}%",
                "█".repeat(filled),
                "░".repeat(empty),
                pct
            );
            lines.push(Line::from(Span::raw("")));
            lines.push(Line::from(Span::styled(bar, Style::default().fg(ACCENT))));
            lines.push(Line::from(Span::styled(
                "Loading model... please wait.",
                Style::default().fg(ACCENT).add_modifier(Modifier::SLOW_BLINK),
            )));
        }
    }

    // Chat messages
    let content_width = area.width.saturating_sub(2) as usize; // minus borders
    for msg in &app.messages {
        let (prefix, color) = if msg.role == "user" {
            ("> USER: ", USER_COLOR)
        } else {
            ("> AI: ", PRIMARY)
        };
        lines.push(Line::from(Span::raw("")));
        // First line with prefix
        let first_line_width = content_width.saturating_sub(prefix.len());
        let content = &msg.content;
        if content.len() <= first_line_width {
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled(content.as_str(), Style::default().fg(color)),
            ]));
        } else {
            // Split content into wrapped lines
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled(&content[..first_line_width], Style::default().fg(color)),
            ]));
            let rest = &content[first_line_width..];
            for chunk in rest.as_bytes().chunks(content_width) {
                let s = String::from_utf8_lossy(chunk);
                lines.push(Line::from(Span::styled(s.to_string(), Style::default().fg(color))));
            }
        }
    }

    // Current streaming response
    if !app.current_response.is_empty() {
        lines.push(Line::from(Span::raw("")));
        let prefix = "> AI: ";
        let first_w = content_width.saturating_sub(prefix.len());
        let content = &app.current_response;
        if content.len() <= first_w {
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)),
                Span::styled(content.as_str(), Style::default().fg(PRIMARY)),
                Span::styled("▌", Style::default().fg(PRIMARY).add_modifier(Modifier::RAPID_BLINK)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)),
                Span::styled(&content[..first_w], Style::default().fg(PRIMARY)),
            ]));
            let rest = &content[first_w..];
            let chunks: Vec<&[u8]> = rest.as_bytes().chunks(content_width).collect();
            for (i, chunk) in chunks.iter().enumerate() {
                let s = String::from_utf8_lossy(chunk);
                let mut spans = vec![Span::styled(s.to_string(), Style::default().fg(PRIMARY))];
                if i == chunks.len() - 1 {
                    spans.push(Span::styled("▌", Style::default().fg(PRIMARY).add_modifier(Modifier::RAPID_BLINK)));
                }
                lines.push(Line::from(spans));
            }
        }
    }

    let total_lines = lines.len() as u16;
    let visible = area.height.saturating_sub(2); // minus borders
    let scroll = total_lines.saturating_sub(visible);

    let chat = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(BORDER))
                .title(" TRANSMISSION LOG "),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(chat, area);
}

fn draw_input(f: &mut Frame, area: Rect, app: &App) {
    let (label, content) = match app.state {
        State::Recording => {
            let secs = app.recording_start
                .map(|s| s.elapsed().as_secs())
                .unwrap_or(0);

            // Build oscilloscope-style waveform from live audio levels
            let levels = app.audio_level.lock().unwrap();
            let width = (area.width as usize).saturating_sub(20); // space for label+timer
            let wave = render_waveform(&levels, width);

            return draw_recording_input(f, area, secs, &wave);
        }
        State::Processing => ("⏳", "[ processing... ]"),
        _ if app.voice_mode => ("🎤 VOICE", "Tap [SPACE] to speak"),
        _ => ("⌨ INPUT", app.input.as_str()),
    };

    let display = if matches!(app.state, State::Recording | State::Processing) {
        content.to_string()
    } else if app.voice_mode {
        content.to_string()
    } else {
        format!("{content}▌")
    };

    let input = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {label} "),
            Style::default().fg(BG).bg(PRIMARY).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ", Style::default().fg(PRIMARY)),
        Span::styled(display, Style::default().fg(PRIMARY)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER)),
    );
    f.render_widget(input, area);
}

fn draw_status(f: &mut Frame, area: Rect, app: &App) {
    let mode = if app.voice_mode { "VOICE" } else { "TEXT" };
    let status = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" [{mode}] "),
            Style::default().fg(ACCENT),
        ),
        Span::styled(&app.status, Style::default().fg(DIM)),
        Span::styled(
            "  [TAB] mode  [ESC] quit",
            Style::default().fg(DIM),
        ),
    ]));
    f.render_widget(status, area);
}

/// Render a 90s oscilloscope-style waveform from raw audio samples.
fn render_waveform(samples: &[f32], width: usize) -> String {
    if samples.is_empty() || width == 0 {
        return "·".repeat(width);
    }

    // Wave characters: maps vertical position to a character
    // Index 0 = top, 4 = center, 8 = bottom
    let wave_top    = ['⠈', '⠁', '⠂', '⠄', '⡀', '⠠', '⠐', '⠈'];
    let wave_smooth = ['˙', '·', '⁻', '⁻', '─', '₋', '·', '˙'];
    let _ = wave_top; // unused, keeping for reference

    // Use box-drawing + braille for that oscilloscope look
    //  top:    ⠉  ╱  
    //  center: ─  
    //  bottom: ⣀  ╲
    let chars = ['⠉', '⠑', '⠒', '─', '⠤', '⢄', '⣀'];

    let n = width;
    let step = samples.len() as f64 / n as f64;

    (0..n)
        .map(|i| {
            let idx = (i as f64 * step) as usize;
            let s = samples[idx.min(samples.len() - 1)];

            // Amplify and clamp to -1..1
            let v = (s * 8.0).clamp(-1.0, 1.0);

            if v.abs() < 0.05 {
                return '─'; // silence = flat center line
            }

            // Map -1..1 to char index 0..6
            let ci = ((v + 1.0) * 3.0) as usize;
            chars[ci.min(6)]
        })
        .collect()
}

fn draw_recording_input(f: &mut Frame, area: Rect, secs: u64, wave: &str) {
    let input = Paragraph::new(Line::from(vec![
        Span::styled(
            " 🎤 REC ",
            Style::default().fg(BG).bg(ERROR).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" {secs}s/28s "),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            wave,
            Style::default().fg(PRIMARY),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ERROR)),
    );
    f.render_widget(input, area);
}

fn draw_approval_popup(f: &mut Frame, area: Rect, tool: &crate::app::PendingTool) {
    // Center a popup box
    let w = 56.min(area.width.saturating_sub(4));
    let h = 11.min(area.height.saturating_sub(4));
    let x = (area.width.saturating_sub(w)) / 2;
    let y = (area.height.saturating_sub(h)) / 2;
    let popup_area = Rect::new(x, y, w, h);

    // Clear background
    f.render_widget(Clear, popup_area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ⚠  TERMINATOR REQUESTS ACCESS",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ACTION: ", Style::default().fg(DIM)),
            Span::styled(&tool.tool, Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  TARGET: ", Style::default().fg(DIM)),
            Span::styled(&tool.display, Style::default().fg(PRIMARY)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled("     [Y] ", Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)),
            Span::styled("APPROVE     ", Style::default().fg(PRIMARY)),
            Span::styled("    [N] ", Style::default().fg(ERROR).add_modifier(Modifier::BOLD)),
            Span::styled("REJECT", Style::default().fg(ERROR)),
        ]),
    ];

    let popup = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ERROR).add_modifier(Modifier::BOLD))
                .title(" ⚠ WARNING ")
                .title_style(Style::default().fg(ERROR).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)),
        )
        .style(Style::default().bg(ratatui::style::Color::Rgb(30, 0, 0)));

    f.render_widget(popup, popup_area);
}
