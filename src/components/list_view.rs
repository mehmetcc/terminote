// src/components/list_view.rs

use crate::{app::App, components::component::Component, input::Action};
use ratatui::{
    layout::Rect,
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

pub struct ListView {
    state: ListState,
    focus: bool,
}

impl ListView {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self { state, focus: false }
    }
}

impl Component for ListView {
    fn render(&mut self, f: &mut Frame, area: Rect, app: &App) {
        let notes = app.note_client.get_all_notes().unwrap_or_default();
        let items: Vec<ListItem> = notes
            .iter()
            .map(|n| ListItem::new(Span::raw(n.title.clone())))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Notes (a:add, e:edit, p:preview, d: delete, q:quit)"),
            )
            .highlight_symbol("➤ ");

        self.state.select(Some(app.selected));
        f.render_stateful_widget(list, area, &mut self.state);
    }

    fn handle(&mut self, action: &Action, app: &mut App) {
        if !self.focus {
            return;
        }
        let notes = app.note_client.get_all_notes().unwrap_or_default();
        match action {
            Action::Up if app.selected > 0 => {
                app.selected -= 1;
            }
            Action::Down if app.selected + 1 < notes.len() => {
                app.selected += 1;
            }
            _ => {} // a/e/p/q will be handled in controller
        }
    }

    fn focused(&self) -> bool {
        self.focus
    }
    fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}
