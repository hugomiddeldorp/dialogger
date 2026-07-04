use rusqlite::{params, Connection, Result};
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::gemini::{Dialogue, DialogueParticipant};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationInfo {
  uuid: String,
  title: String,
}

pub fn run_migrations(conn: &mut Connection) -> Result<()> {
  let version: i32 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;

  if version < 1 {
    conn.execute(
      "ALTER TABLE participants
      ADD COLUMN voice TEXT NOT NULL
      CHECK(voice IN ('male', 'female')) DEFAULT 'female'",
      [])?;
    conn.execute("PRAGMA user_version = 1", [])?;
  }
  Ok(())
}

pub fn write_dialogue(conn: &mut Connection, dialogue: &Dialogue) -> Result<String> {
  let conversation_id = Uuid::now_v7().to_string();
  let tx = conn.transaction()?;

  tx.execute(
    "INSERT INTO conversations (uuid, title) VALUES (?1, ?2)",
    params![conversation_id, dialogue.title]
  )?;

  for (i, participant) in dialogue.people.iter().enumerate() {
    tx.execute(
      "INSERT INTO participants (conversation_id, \"name\", position, voice) VALUES (?1, ?2, ?3, ?4)",
      params![conversation_id, participant.name, i as u8, participant.voice]
    )?;
  }

  for (i, line) in dialogue.dialogue.iter().enumerate() {
    tx.execute(
      "INSERT INTO dialogue_lines (conversation_id, \"order\", content) VALUES (?1, ?2, ?3)",
      params![conversation_id, i as u8, line]
    )?;
  }

  tx.commit()?;
  Ok(conversation_id)
}

pub fn get_dialogue(conn: &mut Connection, conversation_id: &str) -> Result<Dialogue> {
  // Get title
  let title: String = conn.query_row(
    "SELECT title FROM conversations WHERE uuid = ?1",
    params![conversation_id],
    |row| row.get(0)
  )?;

  // Get participants
  let mut stmt = conn.prepare(
    "SELECT name, voice FROM participants WHERE conversation_id = ?1 ORDER BY \"position\""
  )?;
  let people: Vec<DialogueParticipant> = stmt
    .query_map(params![conversation_id], |row| {
      Ok(DialogueParticipant {
        name: row.get(0)?,
        voice: row.get(1)?
      })
    })?
    .collect::<Result<Vec<_>, _>>()?;
  let people: [DialogueParticipant; 2] = people.try_into()
    .map_err(|v: Vec<DialogueParticipant>| {
      rusqlite::Error::InvalidParameterCount(v.len(), 2)
    })?;

  // Get lines of dialogue
  let mut stmt = conn.prepare(
    "SELECT content FROM dialogue_lines WHERE conversation_id = ?1 ORDER BY \"order\""
  )?;
  let dialogue: Vec<String> = stmt
    .query_map(params![conversation_id], |row| row.get(0))?
    .collect::<Result<Vec<_>, _>>()?;

  Ok(Dialogue { title, people, dialogue })
}

pub fn get_conversations(conn: &mut Connection) -> Result<Vec<ConversationInfo>> {
  // TODO: Will need to do proper pagination if user wants to load more than last 10 conversations
  let mut stmt = conn.prepare(
    "SELECT uuid, title FROM conversations ORDER BY created_date DESC LIMIT 10"
  )?;
  let conversation_list = stmt
    .query_map([], |row| {
      let uuid: String = row.get(0)?;
      let title: String = row.get(1)?;
      Ok(ConversationInfo { uuid, title })
    })?
    .collect::<Result<Vec<_>, _>>()?;
  Ok(conversation_list)
}
