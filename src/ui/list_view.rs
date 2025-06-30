// src/ui/list_view.rs

use crate::app::App;
use ratatui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let items = app
        .notes
        .iter()
        .map(|n| ListItem::new(Span::raw(n.title.clone())))
        .collect::<Vec<_>>();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Notes"))
        .highlight_symbol("➤ ");

    f.render_stateful_widget(list, area, &mut app.state);
}
