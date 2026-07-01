use rusqlite::{params, Connection, Result};
use uuid::Uuid;

use crate::gemini::Dialogue;

pub fn write_dialogue(conn: &mut Connection, dialogue: &Dialogue) -> Result<String> {
  let conversation_id = Uuid::now_v7().to_string();
  let tx = conn.transaction()?;

  tx.execute(
    "INSERT INTO conversations (uuid, title) VALUES (?1, ?2)",
    params![conversation_id, dialogue.title]
  )?;

  for (i, name) in dialogue.people.iter().enumerate() {
    tx.execute(
      "INSERT INTO participants (conversation_id, \"name\", position) VALUES (?1, ?2, ?3)",
      params![conversation_id, name, i as u8]
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
    "SELECT name FROM participants WHERE conversation_id = ?1 ORDER BY \"position\""
  )?;
  let people: Vec<String> = stmt
    .query_map(params![conversation_id], |row| row.get(0))?
    .collect::<Result<Vec<_>, _>>()?;
  let people: [String; 2] = people.try_into().map_err(|v: Vec<String>| {
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
