use std::collections::HashMap;

use holochain_manager::versions::holochain_types_latest::web_app::WebAppBundle;
use holochain_web_app_manager::WebAppManager;
// use holochain_manager::versions::version_manager::VersionManager;


pub async fn install_default_apps_if_necessary(manager: &mut WebAppManager, window: tauri::window::Window) -> Result<(), String> {
  let apps = manager.list_apps().await?;

  // let version: String = manager.holochain_manager.version.manager().hdi_version().into();
  let holochain_version: String = manager.holochain_manager.version.into();

  let devhub_app_id = format!("DevHub-{}", holochain_version);

  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&devhub_app_id) == false {

    // emitting signal to the front-end for progress indication
    window.emit("progress-update", String::from("Installing DevHub"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    let dev_hub_bundle = WebAppBundle::decode(include_bytes!("../../../DevHub.webhapp"))
      .or(Err("Malformed Web hApp bundle file"))?;

    let network_seed = if cfg!(debug_assertions) { Some(String::from("launcher-dev")) } else { None };

    manager
      .install_web_app(
        devhub_app_id,
        dev_hub_bundle,
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