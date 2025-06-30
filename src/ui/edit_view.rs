// src/ui/edit_view.rs

use crate::app::{App, Mode};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let title = match app.mode {
        Mode::AddTitle    => "New Title",
        Mode::AddContent  => "New Content",
        Mode::EditTitle   => "Edit Title",
        Mode::EditContent => "Edit Content",
        _ => unreachable!(),
    };

    let p = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(title));

    f.render_widget(p, area);
}
