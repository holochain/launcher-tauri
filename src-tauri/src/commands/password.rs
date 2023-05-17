use crate::{launcher::state::LauncherState, file_system::Profile};

#[tauri::command]
pub async fn initialize_keystore(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  password: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (K)"))
  }
  // emitting signal to the front-end for progress indication
  window.emit("progress-update", String::from("Initializing keystore"))
    .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .initialize_keystore_and_launch(password, profile.inner().clone())
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn unlock_and_launch(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  password: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (K)"))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager.launch_managers(password, profile.inner().clone()).await?;

  Ok(())
}
