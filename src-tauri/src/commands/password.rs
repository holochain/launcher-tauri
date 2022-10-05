use crate::launcher::state::LauncherState;

#[tauri::command]
pub async fn initialize_keystore(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  password: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (J)"))
  }
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .initialize_and_launch_keystore(password)
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn unlock_and_launch(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  password: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (K)"))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager.launch_keystore(password).await?;

  Ok(())
}
