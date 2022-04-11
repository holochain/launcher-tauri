use crate::launcher::state::LauncherState;
use holochain_manager::versions::HolochainVersion;

#[tauri::command]
pub async fn enable_app(
  state: tauri::State<'_, LauncherState>,
  holochain_version: HolochainVersion,
  app_id: String,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

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
  holochain_version: HolochainVersion,
  app_id: String,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_version)?
    .disable_app(app_id.clone())
    .await?;

  log::info!("Disabled app: app_id = {}", app_id);
  Ok(())
}
