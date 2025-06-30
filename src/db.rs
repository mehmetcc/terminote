// src/db.rs

use crate::note::Note;
use chrono::Utc;
use rusqlite::{Connection, Row, params, OptionalExtension};
use uuid::Uuid;

pub struct NoteClient {
    connection: Connection,
}

impl NoteClient {
    pub fn new(connection: Connection) -> Result<Self, rusqlite::Error> {
        connection.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             CREATE TABLE IF NOT EXISTS notes (
                 id TEXT PRIMARY KEY,
                 title TEXT NOT NULL,
                 content TEXT NOT NULL,
                 created_at TEXT NOT NULL,
                 updated_at TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at);
            ",
        )?;
        Ok(NoteClient { connection })
    }

    fn row_to_note(row: &Row) -> rusqlite::Result<Note> {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }

    pub fn add_note(&self, note: &Note) -> Result<usize, rusqlite::Error> {
        self.connection.execute(
            "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                note.id,
                note.title,
                note.content,
                note.created_at,
                note.updated_at
            ],
        )
    }

    pub fn update_note(&self, note: &mut Note) -> Result<usize, rusqlite::Error> {
        note.updated_at = Utc::now();
        self.connection.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            params![note.title, note.content, note.updated_at, note.id],
        )
    }

    pub fn get_note_by_id(&self, id: Uuid) -> Result<Option<Note>, rusqlite::Error> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, content, created_at, updated_at
             FROM notes
             WHERE id = ?1",
        )?;
        let note = stmt
            .query_row(params![id], |row| Self::row_to_note(row))
            .optional()?;
        Ok(note)
    }

    pub fn get_all_notes(&self) -> Result<Vec<Note>, rusqlite::Error> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, content, created_at, updated_at
             FROM notes
             ORDER BY created_at DESC",
        )?;
        let notes = stmt
            .query_map([], |row| Self::row_to_note(row))?
            .collect::<Result<_, _>>()?;
        Ok(notes)
    }

    // Starts from page 1
    pub fn get_notes_paginated(
        &self,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<Note>, rusqlite::Error> {
        let offset = per_page.saturating_mul(page.saturating_sub(1));
        let mut stmt = self.connection.prepare(
            "SELECT id, title, content, created_at, updated_at
             FROM notes
             ORDER BY created_at DESC
             LIMIT ?1 OFFSET ?2",
        )?;
        let notes = stmt
            .query_map(params![per_page, offset], |row| Self::row_to_note(row))?
            .collect::<Result<_, _>>()?;
        Ok(notes)
    }
}
