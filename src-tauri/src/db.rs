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
