use hdk::prelude::CellId;

use crate::launcher::{state::LauncherState, manager::HolochainId};

#[tauri::command]
pub async fn enable_app(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  holochain_id: HolochainId,
  app_id: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (D)"))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_id)?
    .enable_app(app_id.clone())
    .await?;

  log::info!("Enabled app: app_id = {}", app_id);

  manager.on_apps_changed().await?;

  Ok(())
}

#[tauri::command]
pub async fn disable_app(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  holochain_id: HolochainId,
  app_id: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command."))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_id)?
    .disable_app(app_id.clone())
    .await?;

  log::info!("Disabled app: app_id = {}", app_id);

  manager.on_apps_changed().await?;

  Ok(())
}

#[tauri::command]
pub async fn delete_clone(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  holochain_id: HolochainId,
  app_id: String,
  cell_id: CellId,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command."))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_id)?
    .delete_clone(app_id.clone(), cell_id.clone())
    .await?;

  log::info!("Deleted cloned cell: app_id = {}, cell_id = {:?}", app_id, cell_id);

  manager.on_apps_changed().await?;

  Ok(())
}