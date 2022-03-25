use crate::launcher::state::LauncherState;
use holochain_manager::{app_manager::AppManager, versions::HolochainVersion};

#[tauri::command]
pub async fn enable_app(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  holochain_version: HolochainVersion,
  app_id: String,
) -> Result<(), String> {
  let mut manager = (*state).get_launcher_manager()?.lock().await;

  manager
    .get_web_happ_manager(holochain_version)?
    .enable_app(app_id.clone())
    .await?;

  log::info!("Enabled app: app_id = {}", app_id);
  Ok(())
}

#[tauri::command]
pub async fn disable_app(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  holochain_version: HolochainVersion,
  app_id: String,
) -> Result<(), String> {
  let mut manager = state.get_launcher_manager()?.lock().await;

  manager
    .get_web_happ_manager(holochain_version)?
    .disable_app(app_id.clone())
    .await?;

  log::info!("Disabled app: app_id = {}", app_id);
  Ok(())
}
