use std::collections::HashMap;

use holochain_manager::versions::HolochainVersion;
use holochain_types::web_app::WebAppBundle;
use holochain_web_app_manager::ReleaseInfo;

use crate::launcher::{state::LauncherState, manager::HolochainId, default_apps::{DEVHUB_APP_ID, DEVHUB_VERSION}};


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

  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&DEVHUB_APP_ID.to_string()) == false {

    let devhub_bundle = WebAppBundle::decode(include_bytes!("../../../DevHub.webhapp"))
      .or(Err("Malformed webhapp bundle file"))?;

    let network_seed = if cfg!(debug_assertions) { Some(String::from("launcher-dev2")) } else { Some(String::from("test-network-0.10.974")) };

    let happ_release_info = ReleaseInfo {
      resource_locator: None,
      version: Some(DEVHUB_VERSION.to_string()),
    };

    let gui_release_info = ReleaseInfo {
      resource_locator: None,
      version: Some(DEVHUB_VERSION.to_string()),
    };

    webapp_manager
      .install_web_app(
        DEVHUB_APP_ID.to_string(),
        devhub_bundle,
        network_seed,
        HashMap::new(),
        None,
        Some(happ_release_info),
        Some(gui_release_info),
      )
      .await?;

    Ok(())

  } else {
    Err(String::from("DevHub is already installed."))
  }

}