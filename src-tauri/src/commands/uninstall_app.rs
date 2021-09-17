use std::fs;

use holochain_conductor_client::AdminWebsocket;
use crate::{
  config::admin_url,
  uis::{
    caddy,
    port_mapping::{app_ui_folder_path, PortMapping},
  },
};

#[tauri::command]
pub async fn uninstall_app(app_id: String) -> Result<(), String> {
  log::info!("Uninstalling: installed_app_id = {}", app_id);

  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  ws.uninstall_app(app_id.clone())
    .await
    .map_err(|err| format!("Error uninstalling app: {:?}", err))?;

  log::info!("Uninstalled hApp {} from the conductor", app_id);

  uninstall_ui(app_id.clone()).await.map_err(|err| {
    log::error!("Error removing the UI for hApp: {}", err);
    err
  })?;

  log::info!("Removed UI for hApp {}", app_id);

  Ok(())
}

// We are assuming that the app id is in the PortMapping
async fn uninstall_ui(app_id: String) -> Result<(), String> {
  let mut port_mapping = PortMapping::read_port_mapping()?;

  let ui_folder_path = app_ui_folder_path(app_id.clone());

  fs::remove_dir_all(ui_folder_path).or(Err("Failed to remove UI folder"))?;

  port_mapping.remove_app_from_mapping(app_id.clone())?;

  caddy::reload_caddy().await?;

  Ok(())
}
