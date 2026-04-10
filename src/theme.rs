use ratatui::style::Color;

// ── Neon CRT palette ──
pub const BG: Color = Color::Black;
pub const PRIMARY: Color = Color::Rgb(0, 255, 136);    // neon green
pub const DIM: Color = Color::Rgb(0, 120, 60);         // dim green
pub const ACCENT: Color = Color::Rgb(255, 204, 0);     // amber
pub const ERROR: Color = Color::Rgb(255, 50, 50);      // red alert
pub const USER_COLOR: Color = Color::Rgb(0, 200, 255); // cyan for user
pub const BORDER: Color = Color::Rgb(0, 180, 100);     // border green

pub const BOOT_LINES: &[&str] = &[
    "TERMINATOR OS v1.0.0",
    "Copyright (c) 2026 Neural Systems Corp.",
    "",
    "Initializing subsystems...",
    "  MEMORY BANKS .............. OK",
    "  NEURAL CORE: {}.. LOADING",
];

pub const BOOT_READY: &[&str] = &[
    "  NEURAL CORE: {}.. ONLINE",
    "  AUDIO SENSOR .............. ACTIVE",
    "  LANGUAGES: 140+ .......... READY",
    "",
    "System ready. Awaiting input.",
];
