// src/components/edit_view.rs

use crate::{
    app::{App, Mode},
    components::component::Component,
    input::Action,
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
            Mode::AddContent => "New Content (↵=newline, Ctrl+X=save, Esc=cancel)",
            Mode::EditTitle => "Edit Title",
            Mode::EditContent => "Edit Content (↵=newline, Ctrl+X=save, Esc=cancel)",
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
            Action::Save => {}
            Action::Enter => match app.mode {
                Mode::AddTitle => {
                    app.buffer = app.input.clone();
                    app.input.clear();
                    app.mode = Mode::AddContent;
                }
                Mode::AddContent | Mode::EditContent => {
                    app.input.push('\n');
                }
                Mode::EditTitle => {
                    app.buffer = app.input.clone();
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
                _ => {}
            },
            Action::Esc => {
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
