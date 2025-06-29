use std::error::Error;

use rusqlite::Connection;

mod config;
mod note;

fn main() -> Result<(), Box<dyn Error>> {
    let settings = config::Settings::new("Settings.toml")?;
    let db_path = settings.db_path();
    let conn = Connection::open(&db_path)?;
    let client = note::NoteClient::new(conn)?;

    // Example usage:
    let my_first_note = note::Note::new("My First Note", "This is the content of my first note.");
    let mut my_second_note =
        note::Note::new("My Second Note", "This is the content of my second note.");

    client.add_note(&my_first_note)?;
    client.add_note(&my_second_note)?;

    my_second_note.content = "Updated content for my second note.".to_string();
    client.update_note(&mut my_second_note)?;

    Ok(())
}
