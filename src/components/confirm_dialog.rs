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
        let p = Paragraph::new(self.prompt.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Confirm (o/x)"),
        );
        f.render_widget(p, area);
    }

    fn handle(&mut self, action: &Action, _app: &mut App) {
        if !self.focus {
            return;
        }
        if let Action::Char('o') = action {
            self.result = Some(true);
        } else if let Action::Char('x') = action {
            self.result = Some(false);
        }
    }

    fn focused(&self) -> bool {
        self.focus
    }
    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}
