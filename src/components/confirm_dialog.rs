// src/components/confirm_dialog.rs

use crate::{app::App, components::component::Component, input::Action};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub struct ConfirmDialog {
    focus: bool,
    prompt: String,
    pub result: Option<bool>,
}

impl ConfirmDialog {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            focus: false,
            prompt: prompt.into(),
            result: None,
        }
    }

    pub fn take_result(&mut self) -> Option<bool> {
        let r = self.result;
        self.result = None;
        r
    }
}

impl Component for ConfirmDialog {
    fn render(&mut self, f: &mut Frame, area: Rect, _app: &App) {
        // Build a two‐line message: your prompt, then the options
        let text = format!("{}\n\n(o = yes, x = no)", self.prompt);

        let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Confirm"));
        f.render_widget(p, area);
    }

    fn handle(&mut self, action: &Action, _app: &mut App) {
        if !self.focus {
            return;
        }
        match action {
            Action::Char('o') => self.result = Some(true),
            Action::Char('x') => self.result = Some(false),
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
