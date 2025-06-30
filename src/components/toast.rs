// src/components/toast.rs

use crate::input::Action;
use crate::{app::App, components::component::Component};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Toast {
    focus: bool,
    message: String,
    ttl: usize,
}

impl Toast {
    pub fn new(message: impl Into<String>, ttl_frames: usize) -> Self {
        Self {
            focus: false,
            message: message.into(),
            ttl: ttl_frames,
        }
    }
}

impl Component for Toast {
    fn render(&mut self, f: &mut Frame, area: Rect, _app: &App) {
        if self.ttl == 0 {
            return;
        }
        let p = Paragraph::new(self.message.as_str())
            .block(Block::default().borders(Borders::ALL).title("Message"));
        // render in the bottom half (you’ll choose a proper area later)
        f.render_widget(p, area);
        self.ttl = self.ttl.saturating_sub(1);
    }

    fn handle(&mut self, _action: &Action, _app: &mut App) {
        // toasts don’t handle input
    }

    fn focused(&self) -> bool {
        self.focus
    }
    fn set_focus(&mut self, _focus: bool) {
        // toasts never take focus
    }
}
