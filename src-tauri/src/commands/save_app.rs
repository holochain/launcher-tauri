use std::{env::temp_dir, fs, path::PathBuf, time::SystemTime};

#[tauri::command]
pub fn save_app(
  window: tauri::Window,
  app_bundle_bytes: Vec<u8>,
) -> Result<PathBuf, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (L)"))
  }

  let now = SystemTime::now();

  let path = temp_dir().join(format!("app_to_install{:?}.webhapp", now));

  fs::write(path.clone(), app_bundle_bytes)
    .map_err(|err| format!("Failed to write app bundle: {}", err))?;

  Ok(path)
}
