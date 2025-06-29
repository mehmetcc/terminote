mod note;

fn main() {
    let client = note::NoteClient::new("notes.db").expect("Failed to create NoteClient");

    let my_first_note = note::Note::new("My First Note", "This is the content of my first note.");
    let mut my_second_note =
        note::Note::new("My Second Note", "This is the content of my second note.");

    client
        .add_note(&my_first_note)
        .expect("Failed to add first note");
    client
        .add_note(&my_second_note)
        .expect("Failed to add second note");

    my_second_note.content = "Updated content for my second note.".to_string();
    client
        .update_note(&my_second_note)
        .expect("Failed to update second note");
}
