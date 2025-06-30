// src/models/note.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Note {
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
