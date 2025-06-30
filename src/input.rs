// src/input.rs

use ratatui::crossterm::event::{Event, KeyCode, poll, read};
use std::time::Duration;

pub enum Action {
    Up,
    Down,
    Add,
    Edit,
    Confirm,
    Cancel,
    Quit,
    InsertChar(char),
    Backspace,
}

pub fn poll_action() -> Option<Action> {
    if poll(Duration::from_millis(100)).ok()? {
        if let Event::Key(key) = read().ok()? {
            return Some(match key.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Char('a') => Action::Add,
                KeyCode::Char('e') => Action::Edit,
                KeyCode::Up => Action::Up,
                KeyCode::Down => Action::Down,
                KeyCode::Enter => Action::Confirm,
                KeyCode::Esc => Action::Cancel,
                KeyCode::Backspace => Action::Backspace,
                KeyCode::Char(c) => Action::InsertChar(c),
                _ => return None,
            });
        }
    }
    None
}
