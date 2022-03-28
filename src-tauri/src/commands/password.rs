use crate::launcher::state::LauncherState;

#[tauri::command]
pub async fn initialize_keystore(
  state: tauri::State<'_, LauncherState>,
  password: String,
) -> Result<(), String> {
  let mut launcher_manager = state.get_launcher_manager()?.lock().await;

  launcher_manager
    .initialize_and_launch_keystore(password)
    .await?;

  Ok(())
}

#[tauri::command]
pub async fn introduce_keystore_password(
  state: tauri::State<'_, LauncherState>,
  password: String,
) -> Result<(), String> {
  let mut launcher_manager = state.get_launcher_manager()?.lock().await;

  launcher_manager.launch_keystore(password).await?;

  Ok(())
}
