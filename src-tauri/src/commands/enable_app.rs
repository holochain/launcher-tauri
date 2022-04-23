use crate::launcher::{state::LauncherState, manager::HolochainId};

#[tauri::command]
pub async fn enable_app(
  state: tauri::State<'_, LauncherState>,
  holochain_id: HolochainId,
  app_id: String,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_id)?
    .enable_app(app_id.clone())
    .await?;

  log::info!("Enabled app: app_id = {}", app_id);
  Ok(())
}

#[tauri::command]
pub async fn disable_app(
  state: tauri::State<'_, LauncherState>,
  holochain_id: HolochainId,
  app_id: String,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_id)?
    .disable_app(app_id.clone())
    .await?;

  log::info!("Disabled app: app_id = {}", app_id);
  Ok(())
}
