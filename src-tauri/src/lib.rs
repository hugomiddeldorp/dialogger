use tauri::Manager;
use serde::{Serialize, Deserialize};
use std::fs;

mod gemini;

#[derive(Serialize, Deserialize, Default)]
struct ApiConfig {
  api_key: String
}

fn get_api_file(app_handle: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
  let config_dir = &app_handle.path().app_data_dir()
    .map_err(|e| e.to_string())?;
  Ok(config_dir.join("api_key.json"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let config_dir = app.path().app_data_dir()?;

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![my_custom_command, save_api_key])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn my_custom_command(app_handle: tauri::AppHandle) -> Result<String, String> {
  let key_path = get_api_file(&app_handle)?;
  let api_config = fs::read_to_string(&key_path)
    .map_err(|e| e.to_string())?;
  let config_json: ApiConfig = serde_json::from_str(&api_config)
    .map_err(|e| e.to_string())?;
  let api_key = config_json.api_key;

  gemini::generate_dialogue(api_key)
    .await
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
