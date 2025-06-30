// src/components/markdown_view.rs

use crate::{app::App, components::component::Component, input::Action};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct MarkdownView {
    focus: bool,
    scroll: u16,
}

impl MarkdownView {
    pub fn new() -> Self {
        Self {
            focus: false,
            scroll: 0,
        }
    }
}

impl Component for MarkdownView {
    fn render(&mut self, f: &mut Frame, area: Rect, app: &App) {
        // use app.buffer as the title, and app.input as the body
        let header = if app.buffer.is_empty() {
            "Preview (Esc to list)".to_string()
        } else {
            format!("{} — Preview (Esc to list)", app.buffer)
        };

        let p = Paragraph::new(app.input.as_str())
            .block(Block::default().borders(Borders::ALL).title(header))
            .scroll((self.scroll, 0));

        f.render_widget(p, area);
    }

    fn handle(&mut self, action: &Action, _app: &mut App) {
        if !self.focus {
            return;
        }
        match action {
            Action::Up => self.scroll = self.scroll.saturating_sub(1),
            Action::Down => self.scroll = self.scroll.saturating_add(1),
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
