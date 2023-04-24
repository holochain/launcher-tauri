use std::collections::HashMap;

use holochain_manager::versions::holochain_types_latest::web_app::WebAppBundle;
use holochain_web_app_manager::WebAppManager;
// use holochain_manager::versions::version_manager::VersionManager;


/// Installs the AppStore if it is not already installed
pub async fn install_default_apps_if_necessary(manager: &mut WebAppManager, window: tauri::window::Window) -> Result<(), String> {
  let apps = manager.list_apps().await?;

  let appstore_app_id = String::from("AppStore");

  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&appstore_app_id) == false {

    // emitting signal to the front-end for progress indication
    window.emit("progress-update", String::from("Installing AppStore"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    let appstore_bundle = WebAppBundle::decode(include_bytes!("../../../AppStore.webhapp"))
      .or(Err("Malformed webhapp bundle file"))?;

    let network_seed = if cfg!(debug_assertions) { Some(String::from("launcher-dev")) } else { Some(String::from("test-network-0.994")) };

    manager
      .install_web_app(
        appstore_app_id,
        appstore_bundle,
        network_seed,
        HashMap::new(),
        None,
        None,
        None,
      )
      .await?;
  }

  Ok(())
}

