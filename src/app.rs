// src/app.rs

use ratatui::widgets::ListState;
use crate::note::Note;
use uuid::Uuid;

pub enum Mode {
    List,
    AddTitle,
    AddContent,
    EditTitle,
    EditContent,
}

pub struct App {
    pub notes: Vec<Note>,
    pub selected: usize,
    pub state: ListState,
    pub mode: Mode,
    pub input: String,
    pub buffer: String,
    pub edit_id: Option<Uuid>,
}

impl App {
    pub fn new() -> Self {
        let mut ls = ListState::default();
        ls.select(Some(0));
        App {
            notes: Vec::new(),
            selected: 0,
            state: ls,
            mode: Mode::List,
            input: String::new(),
            buffer: String::new(),
            edit_id: None,
        }
    }
}
