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

// Define the states of the application
#[derive(Debug, PartialEq)]
enum State {
    List,
    Edit,
    Preview,
    ConfirmDelete,
    ConfirmDiscard,
}

pub fn run(
    app: &mut App,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let mut list = ListView::new();
    let mut edit = EditView::new();
    let mut preview = MarkdownView::new();
    let mut confirm_discarding_changes = ConfirmDialog::new("Discard changes?");
    let mut confirm_deleting_changes = ConfirmDialog::new("Delete note?");

    let mut state = State::List;
    list.set_focus(true);

    let result = loop {
        terminal.draw(|f| {
            let area = f.area();
            match state {
                State::List => list.render(f, area, app),
                State::Edit => edit.render(f, area, app),
                State::Preview => preview.render(f, area, app),
                State::ConfirmDelete => confirm_deleting_changes.render(f, area, app),
                State::ConfirmDiscard => confirm_discarding_changes.render(f, area, app),
            };
        })?;

        if let Some(action) = poll_action() {
            match state {
                State::List => {
                    if let Err(e) = handle_list_state(
                        app,
                        &mut list,
                        &mut edit,
                        &mut preview,
                        &mut confirm_deleting_changes,
                        action,
                        &mut state,
                    ) {
                        break Err(e);
                    }
                }
                State::Edit => {
                    if let Err(e) = handle_edit_state(
                        app,
                        &mut edit,
                        &mut list,
                        &mut confirm_discarding_changes,
                        action,
                        &mut state,
                    ) {
                        break Err(e);
                    }
                }
                State::Preview => {
                    handle_preview_state(app, &mut preview, &mut list, action, &mut state);
                }
                State::ConfirmDelete => {
                    if let Err(e) = handle_confirm_delete_state(
                        app,
                        &mut confirm_deleting_changes,
                        &mut list,
                        action,
                        &mut state,
                    ) {
                        break Err(e);
                    }
                }
                State::ConfirmDiscard => {
                    handle_confirm_discard_state(
                        app,
                        &mut confirm_discarding_changes,
                        &mut edit,
                        &mut list,
                        action,
                        &mut state,
                    );
                }
            }
        }
    };
    result?;
    Ok(())
}

fn handle_list_state(
    app: &mut App,
    list: &mut ListView,
    edit: &mut EditView,
    preview: &mut MarkdownView,
    confirm_deleting_changes: &mut ConfirmDialog,
    action: Action,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    match action {
        Action::Char('q') => {
            return Err("User requested quit".into());
        }
        Action::Char('a') => {
            start_add_note(app, list, edit, state);
        }
        Action::Char('e') => {
            start_edit_note(app, list, edit, state)?;
        }
        Action::Char('d') => {
            start_delete_note(app, list, confirm_deleting_changes, state)?;
        }
        Action::Char('p') => {
            start_preview_note(app, list, preview, state)?;
        }
        other => list.handle(&other, app),
    }
    Ok(())
}

fn handle_edit_state(
    app: &mut App,
    edit: &mut EditView,
    list: &mut ListView,
    confirm_discarding_changes: &mut ConfirmDialog,
    action: Action,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    match action {
        Action::Save => {
            save_changes(app, edit, list, state)?;
        }
        Action::Esc => {
            discard_changes(app, edit, confirm_discarding_changes, state);
        }
        other => {
            edit.handle(&other, app);
            if matches!(app.mode, Mode::List) {
                edit.set_focus(false);
                list.set_focus(true);
                *state = State::List;
            }
        }
    }
    Ok(())
}

fn handle_preview_state(
    app: &mut App,
    preview: &mut MarkdownView,
    list: &mut ListView,
    action: Action,
    state: &mut State,
) {
    preview.handle(&action, app);
    if let Action::Esc = action {
        preview.set_focus(false);
        app.mode = Mode::List;
        list.set_focus(true);
        *state = State::List;
    }
}

fn handle_confirm_delete_state(
    app: &mut App,
    confirm_deleting_changes: &mut ConfirmDialog,
    list: &mut ListView,
    action: Action,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    confirm_deleting_changes.handle(&action, app);
    if let Some(ok) = confirm_deleting_changes.take_result() {
        confirm_deleting_changes.set_focus(false);
        if ok {
            delete_note(app)?;
        }
        app.mode = Mode::List;
        list.set_focus(true);
        *state = State::List;
    }
    Ok(())
}

fn handle_confirm_discard_state(
    app: &mut App,
    confirm_discarding_changes: &mut ConfirmDialog,
    edit: &mut EditView,
    list: &mut ListView,
    action: Action,
    state: &mut State,
) {
    confirm_discarding_changes.handle(&action, app);
    if let Some(ok) = confirm_discarding_changes.take_result() {
        confirm_discarding_changes.set_focus(false);
        if ok {
            app.input.clear();
            app.buffer.clear();
            app.mode = Mode::List;
            list.set_focus(true);
            *state = State::List;
        } else {
            app.mode = Mode::EditContent;
            edit.set_focus(true);
            *state = State::Edit;
        }
    }
}

fn start_add_note(app: &mut App, list: &mut ListView, edit: &mut EditView, state: &mut State) {
    app.input.clear();
    app.buffer.clear();
    app.mode = Mode::AddTitle;
    list.set_focus(false);
    edit.set_focus(true);
    *state = State::Edit;
}

fn start_edit_note(
    app: &mut App,
    list: &mut ListView,
    edit: &mut EditView,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    let notes = app.note_client.get_all_notes().unwrap_or_default();
    if let Some(n) = notes.get(app.selected) {
        app.edit_id = Some(n.id);
        app.input = n.title.clone();
        app.mode = Mode::EditTitle;
        list.set_focus(false);
        edit.set_focus(true);
        *state = State::Edit;
    }
    Ok(())
}

fn start_preview_note(
    app: &mut App,
    list: &mut ListView,
    preview: &mut MarkdownView,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    let notes = app.note_client.get_all_notes().unwrap_or_default();
    if let Some(n) = notes.get(app.selected) {
        app.buffer = n.title.clone();
        app.input = n.content.clone();
        app.mode = Mode::EditContent;
        list.set_focus(false);
        preview.set_focus(true);
        *state = State::Preview;
    }
    Ok(())
}

fn start_delete_note(
    app: &mut App,
    list: &mut ListView,
    confirm_deleting_changes: &mut ConfirmDialog,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    let notes = app.note_client.get_all_notes().unwrap_or_default();
    if let Some(n) = notes.get(app.selected) {
        app.delete_id = Some(n.id);
        list.set_focus(false);
        confirm_deleting_changes.set_focus(true);
        *state = State::ConfirmDelete;
    }
    Ok(())
}

fn save_changes(
    app: &mut App,
    edit: &mut EditView,
    list: &mut ListView,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
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
    *state = State::List;
    Ok(())
}

fn discard_changes(
    app: &mut App,
    edit: &mut EditView,
    confirm_discarding_changes: &mut ConfirmDialog,
    state: &mut State,
) {
    edit.set_focus(false);
    confirm_discarding_changes.set_focus(true);
    *state = State::ConfirmDiscard;
}

fn delete_note(app: &mut App) -> Result<(), Box<dyn Error>> {
    if let Some(id) = app.delete_id.take() {
        let _ = app.note_client.delete_note(id);
        let len = app.note_client.get_all_notes().unwrap_or_default().len();
        if app.selected >= len && len > 0 {
            app.selected = len - 1;
        }
    }
    Ok(())
}
