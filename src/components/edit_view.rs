// src/components/edit_view.rs

use crate::{
    app::{App, Mode},
    components::component::Component,
    input::Action,
};
use ratatui::widgets::Wrap;
use ratatui::{
    backend::Backend,
    layout::{Position, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::cmp::{max, min};

pub struct EditView {
    focus: bool,
    scroll: usize,
}

impl EditView {
    pub fn new() -> Self {
        Self {
            focus: false,
            scroll: 0,
        }
    }

    fn compute_cursor(&self, area: Rect, input: &str) -> Position {
        let width = area.width.saturating_sub(2) as usize;
        let mut row = 0usize;
        let mut col = 0usize;

        for ch in input.chars() {
            if ch == '\n' {
                row += 1;
                col = 0;
            } else {
                // wrap if we'd exceed the inner width
                if col >= width {
                    row += 1;
                    col = 0;
                }
                col += 1;
            }
        }

        // clamp to the inner box
        let max_rows = area.height.saturating_sub(2) as usize;
        if row > max_rows {
            row = max_rows;
        }
        let max_cols = width;
        if col > max_cols {
            col = max_cols;
        }

        let x = area.x + 1 + col as u16;
        let y = area.y + 1 + row as u16;
        Position::new(x, y)
    }

    fn handle_enter(&mut self, app: &mut App) {
        match app.mode {
            Mode::AddTitle => self.handle_add_title(app),
            Mode::AddContent | Mode::EditContent => app.input.push('\n'),
            Mode::EditTitle => self.handle_edit_title(app),
            _ => {}
        }
    }

    fn handle_add_title(&mut self, app: &mut App) {
        app.buffer = app.input.clone();
        app.input.clear();
        app.mode = Mode::AddContent;
    }

    fn handle_edit_title(&mut self, app: &mut App) {
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

    // TODO: I can't make these work with the current setup
    // TODO: So, maybe think of cleaning this up
    fn scroll_down(&mut self, app: &App, area: Rect) {
        let max_scroll = max(
            0,
            app.input.lines().count() as usize - area.height as usize + 2,
        );
        self.scroll = min(self.scroll + 1, max_scroll);
    }

    fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }
}

impl Component for EditView {
    fn render(&mut self, f: &mut Frame<'_>, area: Rect, app: &App) {
        let title = match app.mode {
            Mode::AddTitle => "New Title",
            Mode::AddContent => "New Content (↵=newline, Ctrl+X=save, Esc=cancel)",
            Mode::EditTitle => "Edit Title",
            Mode::EditContent => "Edit Content (↵=newline, Ctrl+X=save, Esc=cancel)",
            _ => unreachable!(),
        };

        let block = Block::default().borders(Borders::ALL).title(title);

        let paragraph = Paragraph::new(app.input.as_str())
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll as u16, 0));

        f.render_widget(paragraph, area);

        if self.focus {
            let position = self.compute_cursor(area, &app.input);
            f.set_cursor_position(position);
        }
    }

    fn handle(&mut self, action: &Action, app: &mut App) {
        if !self.focus {
            return;
        }

        match action {
            Action::Char(c) => app.input.push(*c),
            Action::Backspace => {
                app.input.pop();
            }
            Action::Save => {}
            Action::Enter => self.handle_enter(app),
            Action::Esc => app.mode = Mode::List,
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
