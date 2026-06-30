use tauri::Manager;
use std::fs;

mod gemini;

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
async fn my_custom_command() -> Result<String, String> {
  gemini::generate_dialogue()
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_api_key(app_handle: tauri::AppHandle, key_string: &str) -> Result<(), String> {
  let config_dir = app_handle.path().app_data_dir()
    .map_err(|e| e.to_string())?;
  let key_path = config_dir.join("api_key.json");

  fs::write(&key_path, format!(r#"{{"api_key": "{key_string}"}}"#))
    .map_err(|e| e.to_string())?;

  Ok(())
}
