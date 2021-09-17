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
use holochain_types::web_app::WebAppBundle;
use mr_bundle::ResourceBytes;
use std::{
  collections::HashMap,
  fs::{self, File},
};

#[tauri::command]
pub async fn install_app(app_id: String, web_app_bundle_path: String) -> Result<(), String> {
  log::info!("Installing: web_app_bundle = {}", web_app_bundle_path);

  let web_app_bundle = WebAppBundle::decode(
    &fs::read(&web_app_bundle_path).or(Err("Failed to read Web hApp bundle file"))?,
  )
  .or(Err("Malformed Web hApp bundle file"))?;

  let app_bundle = web_app_bundle
    .happ_bundle()
    .await
    .or(Err("Failed to resolve hApp bundle"))?;

  install_happ(app_id.clone(), app_bundle).await.map_err(|err| {
    log::error!("Error installing hApp: {}", err);
    err
  })?;

  log::info!("Installed hApp {} in the conductor", app_id);

  let web_ui_zip_bytes = web_app_bundle
    .web_ui_zip_bytes()
    .await
    .or(Err("Failed to resolve Web UI"))?;

  install_ui(app_id.clone(), web_ui_zip_bytes.as_slice().to_vec())
    .await
    .map_err(|err| {
      log::error!("Error installing the UI for hApp: {}", err);
      err
    })?;

  log::info!("Installed UI for hApp {}", app_id);

  Ok(())
}

async fn install_happ(app_id: String, app_bundle: AppBundle) -> Result<(), String> {
  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  let new_key = ws
    .generate_agent_pub_key()
    .await
    .map_err(|err| format!("Error generating public key: {:?}", err))?;

  let payload = InstallAppBundlePayload {
    source: AppBundleSource::Bundle(app_bundle),
    agent_key: new_key,
    installed_app_id: Some(app_id.clone().into()),
    membrane_proofs: HashMap::new(),
    uid: None,
  };
  ws.install_app_bundle(payload)
    .await
    .map_err(|err| format!("Error install hApp bundle: {:?}", err))?;
  ws.enable_app(app_id.into())
    .await
    .map_err(|err| format!("Error enabling app: {:?}", err))?;

  Ok(())
}

async fn install_ui(app_id: String, web_ui_zip_bytes: ResourceBytes) -> Result<(), String> {
  let mut port_mapping = PortMapping::read_port_mapping()?;

  if let Some(_) = port_mapping.get_ui_port_for_app(&app_id) {
    return Err(String::from("App is already installed"));
  }

  let ui_folder_path = app_ui_folder_path(app_id.clone());
  let ui_zip_path = uis_data_path().join(format!("{}.zip", app_id));

  fs::write(ui_zip_path.clone(), web_ui_zip_bytes).or(Err("Failed to write Web UI Zip file"))?;

  let file = File::open(ui_zip_path).or(Err("Failed to read Web UI Zip file"))?;
  unzip_file(file, ui_folder_path)?;

  let port = port_mapping.set_available_ui_port_for_app(app_id.clone())?;

  log::info!("Allocated new port {} for app {}", port, app_id.clone());

  caddy::reload_caddy().await?;

  Ok(())
}
