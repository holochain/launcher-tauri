use std::collections::HashMap;

use holochain_manager::versions::HolochainVersion;
use holochain_types::web_app::WebAppBundle;

use crate::launcher::{state::LauncherState, manager::HolochainId};


/// Installs the DevHub if it is not already installed.
#[tauri::command]
pub async fn install_devhub(
  state: tauri::State<'_, LauncherState>,
  window: tauri::window::Window
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'install_devhub' which is not allowed in this window."))
  }

  log::info!("Requested to install DevHub");

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let webapp_manager = manager.get_web_happ_manager(HolochainId::HolochainVersion(HolochainVersion::default()))?;

  let apps = webapp_manager.list_apps().await?;

  let devhub_app_id = String::from("DevHub");

  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&devhub_app_id) == false {

    let devhub_bundle = WebAppBundle::decode(include_bytes!("../../../DevHub.webhapp"))
      .or(Err("Malformed webhapp bundle file"))?;

    let network_seed = if cfg!(debug_assertions) { Some(String::from("launcher-dev2")) } else { Some(String::from("test-network-0.994")) };

    webapp_manager
      .install_web_app(
        devhub_app_id,
        devhub_bundle,
        network_seed,
        HashMap::new(),
        None,
        None,
        None,
      )
      .await?;

    Ok(())

  } else {
    Err(String::from("DevHub is already installed."))
  }

}