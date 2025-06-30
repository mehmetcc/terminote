// src/components/edit_view.rs

use crate::{
    app::{App, Mode},
    components::component::Component,
    input::Action,
    models::note::Note,
};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct EditView {
    focus: bool,
}

impl EditView {
    pub fn new() -> Self {
        Self { focus: false }
    }
}

impl Component for EditView {
    fn render(&mut self, f: &mut Frame, area: Rect, app: &App) {
        let title = match app.mode {
            Mode::AddTitle => "New Title",
            Mode::AddContent => "New Content",
            Mode::EditTitle => "Edit Title",
            Mode::EditContent => "Edit Content",
            _ => unreachable!(),
        };

        let p = Paragraph::new(app.input.as_str())
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(p, area);
    }

    fn handle(&mut self, action: &Action, app: &mut App) {
        if !self.focus {
            return;
        }

        match action {
            Action::Char(c) => {
                app.input.push(*c);
            }
            Action::Backspace => {
                app.input.pop();
            }
            Action::Enter => match app.mode {
                Mode::AddTitle => {
                    app.buffer = app.input.clone();
                    app.input.clear();
                    app.mode = Mode::AddContent;
                }
                Mode::AddContent => {
                    let note = Note::new(&app.buffer, &app.input);
                    app.note_client.add_note(&note).unwrap();
                    app.mode = Mode::List;
                }
                Mode::EditTitle => {
                    // Save edited title to buffer
                    app.buffer = app.input.clone();
                    // Seed existing content into input
                    if let Some(id) = app.edit_id {
                        if let Ok(Some(n)) = app.note_client.get_note_by_id(id) {
                            app.input = n.content.clone();
                        } else {
                            app.input.clear();
                        }
                    } else {
                        app.input.clear();
                    }
                    app.mode = Mode::EditContent;
                }
                Mode::EditContent => {
                    if let Some(id) = app.edit_id {
                        if let Ok(Some(mut n)) = app.note_client.get_note_by_id(id) {
                            n.title = app.buffer.clone();
                            n.content = app.input.clone();
                            app.note_client.update_note(&mut n).unwrap();
                        }
                    }
                    app.mode = Mode::List;
                }
                _ => {}
            },
            Action::Esc => {
                // go straight back to list
                app.mode = Mode::List;
            }
            _ => {}
        }
    }

    fn focused(&self) -> bool {
        self.focus
    }
    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}
