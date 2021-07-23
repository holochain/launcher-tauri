use crate::{
  config::{admin_url, uis_data_path},
  uis::{
    caddy,
    port_mapping::{app_ui_folder_path, PortMapping},
    utils::unzip_file,
  },
};
use holochain_conductor_client::{AdminWebsocket, InstallAppBundlePayload};
use holochain_types::prelude::{AppBundle, AppBundleSource};
use std::{
  collections::HashMap,
  fs::{self, File},
};

#[tauri::command]
pub async fn install_app(app_bundle_path: String, ui_bundle_path: String) -> Result<(), String> {
  log::info!(
    "Installing: app_id = {}, ui_bundle_path = {}",
    app_bundle_path,
    ui_bundle_path
  );

  let app_id = install_happ(app_bundle_path).await.map_err(|err| {
    log::error!("Error installing hApp: {}", err);
    err
  })?;

  log::info!("Installed hApp {} in the conductor", app_id);

  install_ui(app_id.clone(), ui_bundle_path)
    .await
    .map_err(|err| {
      log::error!("Error installing the UI for hApp: {}", err);
      err
    })?;

  log::info!("Installed UI for hApp {}", app_id);

  Ok(())
}

async fn install_happ(app_bundle_path: String) -> Result<String, String> {
  let app_bundle =
    AppBundle::decode(&fs::read(&app_bundle_path).or(Err("Failed to read hApp bundle file"))?)
      .or(Err("Malformed hApp bundle file"))?;

  let app_id = app_bundle.manifest().app_name();

  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  let new_key = ws
    .generate_agent_pub_key()
    .await
    .map_err(|err| format!("Error generating public key: {:?}", err))?;

  let payload = InstallAppBundlePayload {
    source: AppBundleSource::Path(app_bundle_path.into()),
    agent_key: new_key,
    installed_app_id: Some(app_id.into()),
    membrane_proofs: HashMap::new(),
    uid: None,
  };
  ws.install_app_bundle(payload)
    .await
    .map_err(|err| format!("Error install hApp bundle: {:?}", err))?;
  ws.enable_app(app_id.into())
    .await
    .map_err(|err| format!("Error enabling app: {:?}", err))?;

  Ok(app_id.into())
}

async fn install_ui(app_id: String, ui_bundle_path: String) -> Result<(), String> {
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
