// src/input.rs

use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read};
use std::time::Duration;

#[derive(Debug)]
pub enum Action {
    Up,
    Down,
    Enter,
    Esc,
    Backspace,
    Save,
    Char(char),
}

pub fn poll_action() -> Option<Action> {
    if poll(Duration::from_millis(100)).ok()? {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = read().ok()?
        {
            if modifiers.contains(KeyModifiers::CONTROL) {
                return match code {
                    KeyCode::Char('x') => Some(Action::Save),
                    _ => None,
                };
            }
            return match code {
                KeyCode::Up => Some(Action::Up),
                KeyCode::Down => Some(Action::Down),
                KeyCode::Enter => Some(Action::Enter),
                KeyCode::Esc => Some(Action::Esc),
                KeyCode::Backspace => Some(Action::Backspace),
                KeyCode::Char(c) => Some(Action::Char(c)),
                _ => None,
            };
        }
    }
    None
}
