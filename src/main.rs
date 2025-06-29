use std::error::Error;

mod note;

fn main() -> Result<(), Box<dyn Error>> {
    // Create the data folder, if it doesn't exist
    std::fs::create_dir_all("data")?;
    let connection = rusqlite::Connection::open("data/notes.db")?;
    let client = note::NoteClient::new(connection)?;

    let my_first_note = note::Note::new("My First Note", "This is the content of my first note.");
    let mut my_second_note =
        note::Note::new("My Second Note", "This is the content of my second note.");

    client.add_note(&my_first_note)?;
    client.add_note(&my_second_note)?;

    my_second_note.content = "Updated content for my second note.".to_string();
    client.update_note(&mut my_second_note)?;

    // everything succeeded
    Ok(())
}
