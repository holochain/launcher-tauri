use super::port_mapping::{app_ui_folder_path, PortMapping};
use crate::{
  config::uis_data_path,
  uis::{caddy, utils::unzip_file},
};
use open;
use std::fs::{self, File};

#[tauri::command]
pub async fn install_ui(app_id: String, ui_bundle_path: String) -> Result<(), String> {
  log::info!(
    "Install UI: app_id = {}, ui_bundle_path = {}",
    app_id,
    ui_bundle_path
  );

  let mut port_mapping = PortMapping::read_port_mapping()?;

  if let Some(_) = port_mapping.get_ui_port_for_app(&app_id) {
    return Err(String::from("App is already installed"));
  }

  let ui_folder_path = app_ui_folder_path(app_id.clone());
  let ui_zip_path = uis_data_path().join(format!("{}.zip", app_id));

  fs::copy(ui_bundle_path, ui_zip_path.clone()).or(Err("Failed to read UI ZIP file"))?;

  unzip_file(
    File::open(ui_zip_path).or(Err("Failed to read file"))?,
    ui_folder_path,
  )?;

  let port = port_mapping.set_available_ui_port_for_app(app_id.clone())?;

  log::info!("Allocated new port {} for app {}", port, app_id.clone());
  
  caddy::reload_caddy().await?;

  Ok(())
}

#[tauri::command]
pub async fn activate_app_ui(app_id: String) -> Result<(), String> {
  log::info!("Activating UI: app_id = {}", app_id);

  caddy::reload_caddy().await
}

#[tauri::command]
pub async fn deactivate_app_ui(app_id: String) -> Result<(), String> {
  log::info!("Deactivating UI: app_id = {}", app_id);

  caddy::reload_caddy().await
}

#[tauri::command]
pub fn open_app_ui(app_id: String) -> Result<(), String> {
  let port_mapping = PortMapping::read_port_mapping()?;

  let port = port_mapping
    .get_ui_port_for_app(&app_id)
    .ok_or("App not registered")?;

  let app_url = format!("http://localhost:{}", port);

  let result = open::that_in_background(app_url.as_str());
  log::info!(
    "Opening app {} at {}, result: {:?}",
    app_id.clone(),
    app_url,
    result
  );

  Ok(())
}
