// src/controller.rs

use crate::models::note::Note;
use crate::{
    app::{App, Mode},
    components::{
        component::Component, confirm_dialog::ConfirmDialog, edit_view::EditView,
        list_view::ListView, markdown_view::MarkdownView,
    },
    input::{poll_action, Action},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, io::Stdout};

pub fn run(
    app: &mut App,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let mut list = ListView::new();
    let mut edit = EditView::new();
    let mut preview = MarkdownView::new();
    let mut confirm_discarding_changes = ConfirmDialog::new("Discard changes?");
    let mut confirm_deleting_changes = ConfirmDialog::new("Delete note?");

    list.set_focus(true);

    loop {
        terminal.draw(|f| {
            let area = f.area();
            if list.focused() {
                list.render(f, area, app);
            } else if edit.focused() {
                edit.render(f, area, app);
            } else if preview.focused() {
                preview.render(f, area, app);
            } else if confirm_deleting_changes.focused() {
                confirm_deleting_changes.render(f, area, app);
            } else {
                confirm_discarding_changes.render(f, area, app);
            }
        })?;

        if let Some(action) = poll_action() {
            if list.focused() {
                match action {
                    Action::Char('q') => break,
                    Action::Char('a') => {
                        app.input.clear();
                        app.buffer.clear();
                        app.mode = Mode::AddTitle;
                        list.set_focus(false);
                        edit.set_focus(true);
                    }
                    Action::Char('e') => {
                        let notes = app.note_client.get_all_notes().unwrap_or_default();
                        if let Some(n) = notes.get(app.selected) {
                            app.edit_id = Some(n.id);
                            app.input = n.title.clone();
                            app.mode = Mode::EditTitle;
                            list.set_focus(false);
                            edit.set_focus(true);
                        }
                    }
                    Action::Char('d') => {
                        let notes = app.note_client.get_all_notes().unwrap_or_default();
                        if let Some(n) = notes.get(app.selected) {
                            app.delete_id = Some(n.id);
                            list.set_focus(false);
                            confirm_deleting_changes.set_focus(true);
                        }
                    }
                    Action::Char('p') => {
                        let notes = app.note_client.get_all_notes().unwrap_or_default();
                        if let Some(n) = notes.get(app.selected) {
                            app.buffer = n.title.clone();
                            app.input = n.content.clone();
                            app.mode = Mode::EditContent;
                            list.set_focus(false);
                            preview.set_focus(true);
                        }
                    }
                    other => list.handle(&other, app),
                }
                continue;
            }

            if edit.focused() {
                if let Action::Save = action {
                    match app.mode {
                        Mode::AddContent => {
                            let note = Note::new(&app.buffer, &app.input);
                            app.note_client.add_note(&note)?;
                        }
                        Mode::EditContent => {
                            if let Some(id) = app.edit_id {
                                let mut n = app.note_client.get_note_by_id(id)?.unwrap();
                                n.title = app.buffer.clone();
                                n.content = app.input.clone();
                                app.note_client.update_note(&mut n)?;
                            }
                        }
                        _ => {}
                    }
                    app.mode = Mode::List;
                    edit.set_focus(false);
                    list.set_focus(true);
                    continue;
                }
                if let Action::Esc = action {
                    edit.set_focus(false);
                    confirm_discarding_changes.set_focus(true);
                    continue;
                }
                edit.handle(&action, app);
                if matches!(app.mode, Mode::List) {
                    edit.set_focus(false);
                    list.set_focus(true);
                }
                continue;
            }

            if preview.focused() {
                preview.handle(&action, app);
                if let Action::Esc = action {
                    preview.set_focus(false);
                    app.mode = Mode::List;
                    list.set_focus(true);
                }
                continue;
            }

            if confirm_deleting_changes.focused() {
                confirm_deleting_changes.handle(&action, app);
                if let Some(ok) = confirm_deleting_changes.take_result() {
                    confirm_deleting_changes.set_focus(false);
                    if ok {
                        if let Some(id) = app.delete_id.take() {
                            let _ = app.note_client.delete_note(id);
                            let len = app.note_client.get_all_notes().unwrap_or_default().len();
                            if app.selected >= len && len > 0 {
                                app.selected = len - 1;
                            }
                        }
                        app.mode = Mode::List;
                        list.set_focus(true);
                    } else {
                        app.mode = Mode::List;
                        list.set_focus(true);
                    }
                }
                continue;
            }

            if confirm_discarding_changes.focused() {
                confirm_discarding_changes.handle(&action, app);
                if let Some(ok) = confirm_discarding_changes.take_result() {
                    confirm_discarding_changes.set_focus(false);
                    if ok {
                        app.input.clear();
                        app.buffer.clear();
                        app.mode = Mode::List;
                        list.set_focus(true);
                    } else {
                        app.mode = Mode::EditContent;
                        edit.set_focus(true);
                    }
                }
                continue;
            }
        }
    }

    Ok(())
}
