use tauri::Manager;
use tauri::path::BaseDirectory;
use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use std::fs;
use std::sync::Mutex;
use std::io::Cursor;
use piper_rs::Piper;

use crate::gemini::Dialogue;
use crate::db::ConversationInfo;

mod gemini;
mod db;

const SCHEMA: &str = include_str!("../sql/schema.sql");

#[derive(Serialize, Deserialize, Default)]
struct ApiConfig {
  api_key: String
}

struct AppState {
  conn: Mutex<Connection>
}

fn get_api_file(app_handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
  let config_dir = &app_handle.path().app_data_dir()
    .map_err(|e| e.to_string())?;
  Ok(config_dir.join("api_key.json"))
}

fn load_request_body() -> anyhow::Result<Value> {
  let request_file = fs::read_to_string("src/dialogue_prompt.json")?;
  let request_body: Value = serde_json::from_str(&request_file)?;
  Ok(request_body)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let config_dir = app.path().app_data_dir()
        .expect("could not result app data dir");
      let mut conn = Connection::open(config_dir.join("app.db"))
        .expect("failed to open DB");

      // Run schema and migrations
      conn.execute_batch(SCHEMA)?;
      db::run_migrations(&mut conn)?;

      app.manage(AppState { conn: Mutex::new(conn) });

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![generate_dialogue, save_api_key, get_conversations, get_dialogue, speak])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn generate_dialogue(state: tauri::State<'_, AppState>, app_handle: tauri::AppHandle, prompt: String) -> Result<String, String> {
  // TODO: This assumes the config file exists, which isn't true on first run
  let key_path = get_api_file(&app_handle)?;
  let api_config = fs::read_to_string(&key_path)
    .map_err(|e| e.to_string())?;
  let config_json: ApiConfig = serde_json::from_str(&api_config)
    .map_err(|e| e.to_string())?;
  let api_key = config_json.api_key;

  // Hydrate the request with the user's prompt
  let mut request_body = load_request_body()
    .map_err(|e| e.to_string())?;
  request_body["input"] = json!(prompt);

  let dialogue = gemini::generate_dialogue(api_key, request_body)
    .await
    .map_err(|e| e.to_string())?;

  let mut conn = state.conn.lock().unwrap();
  db::write_dialogue(&mut conn, &dialogue)
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_dialogue(state: tauri::State<'_, AppState>, conversation_id: String) -> Result<Dialogue, String> {
  let mut conn = state.conn.lock().unwrap();

  db::get_dialogue(&mut conn, &conversation_id)
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_conversations(state: tauri::State<'_, AppState>) -> Result<Vec<ConversationInfo>, String> {
  let mut conn = state.conn.lock().unwrap();

  db::get_conversations(&mut conn)
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_api_key(app_handle: tauri::AppHandle, key_string: String) -> Result<(), String> {
  let key_path = get_api_file(&app_handle)?;
  let api_config = ApiConfig { api_key: key_string };
  let json = serde_json::to_string(&api_config)
    .map_err(|e| e.to_string())?;
  fs::write(&key_path, json)
    .map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command]
async fn speak(app: tauri::AppHandle, text: String, voice: String) -> Result<Vec<u8>, String> {
  let model_path = app.path().resolve("models/fr_FR-upmc-medium.onnx", BaseDirectory::Resource)
    .map_err(|e| e.to_string())?;
  let model_config_path = app.path().resolve("models/fr_FR-upmc-medium.onnx.json", BaseDirectory::Resource)
    .map_err(|e| e.to_string())?;

  // TODO: this will not be consistent across different models
  let speaker_id = match voice.as_str() {
    "female"=> Some(0),
    "male"=> Some(1),
    _=> None,
  };

  let mut piper = Piper::new(&model_path.as_path(), &model_config_path.as_path())
    .map_err(|e| e.to_string())?;

  let (samples, sample_rate) = piper
    .create(&text, false, speaker_id, None, None, None)
    .map_err(|e| e.to_string())?;

  let wav_bytes = encode_wav(&samples, sample_rate)
    .map_err(|e| e.to_string())?;

  Ok(wav_bytes)
}

fn encode_wav(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>, hound::Error> {
  let spec = hound::WavSpec {
    channels: 1,
    sample_rate,
    bits_per_sample: 32,
    sample_format: hound::SampleFormat::Float,
  };

  let mut cursor = Cursor::new(Vec::new());
  let mut writer = hound::WavWriter::new(&mut cursor, spec)?;

  for &sample in samples {
    writer.write_sample(sample)?;
  }
  writer.finalize()?;

  Ok(cursor.into_inner())
}
