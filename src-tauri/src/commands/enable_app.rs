use crate::{managers::holochain::conductor::ConductorManager, state::LauncherState};

#[tauri::command]
pub async fn enable_app(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  app_id: String,
) -> Result<(), String> {
  let mut manager = state.get_launcher_manager()?.lock().await;

  manager
    .get_holochain_manager()?
    .conductor_manager
    .enable_app(&app_id)
    .await?;

  manager.on_apps_changed(&app_handle).await?;

  log::info!("Enabled app: app_id = {}", app_id);
  Ok(())
}

#[tauri::command]
pub async fn disable_app(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  app_id: String,
) -> Result<(), String> {
  let mut manager = state.get_launcher_manager()?.lock().await;

  manager
    .get_holochain_manager()?
    .conductor_manager
    .disable_app(&app_id)
    .await?;

  manager.on_apps_changed(&app_handle).await?;

  log::info!("Disabled app: app_id = {}", app_id);
  Ok(())
}
