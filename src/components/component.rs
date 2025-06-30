// src/components/component.rs

use crate::app::App;
use crate::input::Action;
use ratatui::{Frame, layout::Rect};

/// A reusable UI widget
pub trait Component {
    fn render(&mut self, f: &mut Frame, area: Rect, app: &App);
    fn handle(&mut self, action: &Action, app: &mut App);
    fn focused(&self) -> bool;
    fn set_focus(&mut self, focus: bool);
}
