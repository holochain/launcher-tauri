use crate::launcher::state::LauncherState;

#[tauri::command]
pub async fn initialize_keystore(
  state: tauri::State<'_, LauncherState>,
  password: String,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .initialize_and_launch_keystore(password)
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn unlock_and_launch(
  state: tauri::State<'_, LauncherState>,
  password: String,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager.launch_keystore(password).await?;

  Ok(())
}
