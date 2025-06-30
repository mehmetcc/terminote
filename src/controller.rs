// src/controller.rs
use crate::{
    app::{App, Mode},
    db::NoteClient,
    input::{Action, poll_action},
    ui,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::error::Error;
use std::io::Stdout;

pub fn run(
    app: &mut App,
    client: NoteClient,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    loop {
        // keep selection in sync
        app.state.select(Some(app.selected));

        terminal.draw(|f| {
            let area = f.area();
            if matches!(app.mode, Mode::List) {
                ui::list_view::render(f, area, app);
            } else {
                ui::edit_view::render(f, area, app);
            }
        })?;

        if let Some(action) = poll_action() {
            match action {
                Action::Quit => break,
                Action::Up => app.selected = app.selected.saturating_sub(1),
                Action::Down => {
                    if app.selected + 1 < app.notes.len() {
                        app.selected += 1
                    }
                }
                Action::Add => app.mode = Mode::AddTitle,
                Action::Edit => {
                    if let Some(n) = app.notes.get(app.selected) {
                        app.edit_id = Some(n.id);
                        app.input = n.title.clone();
                        app.mode = Mode::EditTitle;
                    }
                }
                Action::Confirm => match app.mode {
                    Mode::AddTitle => {
                        app.buffer = app.input.clone();
                        app.input.clear();
                        app.mode = Mode::AddContent;
                    }
                    Mode::AddContent => {
                        let note = crate::note::Note::new(&app.buffer, &app.input);
                        client.add_note(&note)?;
                        app.notes = client.get_all_notes()?;
                        app.mode = Mode::List;
                    }
                    Mode::EditTitle => {
                        app.buffer = app.input.clone();
                        app.input.clear();
                        app.mode = Mode::EditContent;
                    }
                    Mode::EditContent => {
                        if let Some(id) = app.edit_id {
                            if let Some(mut note) = app.notes.iter_mut().find(|n| n.id == id) {
                                note.title = app.buffer.clone();
                                note.content = app.input.clone();
                                client.update_note(&mut note)?;
                                app.notes = client.get_all_notes()?;
                            }
                        }
                        app.mode = Mode::List;
                    }
                    _ => {}
                },
                Action::Cancel => app.mode = Mode::List,
                Action::InsertChar(c) => app.input.push(c),
                Action::Backspace => {
                    app.input.pop();
                }
            }
        }
    }
    Ok(())
}
