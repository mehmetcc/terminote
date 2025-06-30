// src/app.rs

use crate::db::NoteClient;
use ratatui::widgets::ListState;
use uuid::Uuid;

pub enum Mode {
    List,
    AddTitle,
    AddContent,
    EditTitle,
    EditContent,
}

pub struct App {
    pub selected: usize,
    pub state: ListState,
    pub mode: Mode,
    pub input: String,
    pub buffer: String,
    pub edit_id: Option<Uuid>,
    pub delete_id: Option<Uuid>,
    pub note_client: NoteClient,
}

impl App {
    pub fn new(note_client: NoteClient) -> Self {
        let mut ls = ListState::default();
        ls.select(Some(0));
        App {
            selected: 0,
            state: ls,
            mode: Mode::List,
            input: String::new(),
            buffer: String::new(),
            edit_id: None,
            delete_id: None,
            note_client,
        }
    }
}
