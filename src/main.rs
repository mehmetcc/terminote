// src/main.rs

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{
    cursor::Show,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rusqlite::Connection;
use std::{error::Error, io};

mod app;
mod config;
mod controller;
mod db;
mod input;
mod note;
mod ui;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let settings = config::Settings::new("Settings.toml")?;
    let conn = Connection::open(settings.db_path())?;
    let client = db::NoteClient::new(conn)?;
    let mut app = app::App::new();
    app.notes = client.get_all_notes()?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let _cleanup = TerminalCleanupGuard::new()?;
    controller::run(&mut app, client, &mut terminal)?;

    Ok(())
}

pub struct TerminalCleanupGuard {
    stdout: io::Stdout,
}

impl TerminalCleanupGuard {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();
        // enter raw mode + alt screen + enable mouse
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        Ok(Self { stdout })
    }
}

impl Drop for TerminalCleanupGuard {
    fn drop(&mut self) {
        // best‐effort cleanup
        let _ = disable_raw_mode();
        let _ = execute!(self.stdout, LeaveAlternateScreen, DisableMouseCapture, Show);
    }
}
