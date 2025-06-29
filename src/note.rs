use chrono::{DateTime, Utc};
use rusqlite::Connection;
use uuid::Uuid;

pub(crate) struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // TODO: add categories in the future
}

impl Note {
    pub fn new(title: &str, content: &str) -> Self {
        Note {
            id: Uuid::new_v4(),
            title: title.to_string(),
            content: content.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub struct NoteClient {
    connection: Connection,
}

impl NoteClient {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(db_path)?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(NoteClient { connection })
    }

    pub fn add_note(&self, note: &Note) -> Result<usize, rusqlite::Error> {
        self.connection.execute(
            "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                note.id,
                note.title,
                note.content,
                note.created_at,
                note.updated_at
            ],
        )
    }

    pub fn update_note(&self, note: &Note) -> Result<usize, rusqlite::Error> {
        self.connection.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            rusqlite::params![note.title, note.content, note.updated_at, note.id],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let note = Note::new("Test Title", "This is a test content.");
        assert_eq!(note.title, "Test Title");
        assert_eq!(note.content, "This is a test content.");
        assert!(!note.id.is_nil());
        assert!(note.created_at <= Utc::now());
        assert!(note.updated_at <= Utc::now());
    }
}
