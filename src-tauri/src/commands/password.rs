use crate::{launcher::state::LauncherState, file_system::CustomPath};

#[tauri::command]
pub async fn initialize_keystore(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  custom_path: tauri::State<'_, CustomPath>,
  password: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (J)"))
  }
  // emitting signal to the front-end for progress indication
  window.emit("progress-update", String::from("Initializing keystore"))
    .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .initialize_and_launch_keystore(password, custom_path.custom_path.clone())
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn unlock_and_launch(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  custom_path: tauri::State<'_, CustomPath>,
  password: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (K)"))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager.launch_keystore(password, custom_path.custom_path.clone()).await?;

  Ok(())
}
