use std::collections::HashMap;

use holochain_manager::versions::holochain_types_latest::web_app::WebAppBundle;
use holochain_web_app_manager::{WebAppManager, ReleaseInfo};
// use holochain_manager::versions::version_manager::VersionManager;


// Versions of Appstore and Devhub (commit hash of Github release).
// Those are used to install new Appstore/DevHub UI's if necessary
// NEW_VERSION Change appstore and devhub versions here if required
const APPSTORE_VERSION: &str = "36b8793";
const DEVHUB_VERSION: &str = "92417d3";

const APPSTORE_APP_ID: &str = "AppStore";
const DEVHUB_APP_ID: &str = "DevHub";



/// Installs the AppStore if it is not already installed and updates AppStore/DevHub UI's if necessary
pub async fn install_default_apps_if_necessary(manager: &mut WebAppManager, window: tauri::window::Window) -> Result<(), String> {
  let apps = manager.list_apps().await?;

  let appstore_bundle = WebAppBundle::decode(include_bytes!("../../../AppStore.webhapp"))
    .or(Err("Malformed AppStore webhapp bundle file"))?;

  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&APPSTORE_APP_ID.to_string()) == false {

    // emitting signal to the front-end for progress indication
    window.emit("progress-update", String::from("Installing AppStore"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    let network_seed = if cfg!(debug_assertions) { Some(String::from("launcher-dev2")) } else { Some(String::from("test-network-0.994")) };

    manager
      .install_web_app(
        APPSTORE_APP_ID.to_string(),
        appstore_bundle,
        network_seed,
        HashMap::new(),
        None,
        None,
        None,
      )
      .await?;
  } else { // If the AppStore is already installed, check UI version

    // emitting signal to the front-end for progress indication
    window.emit("progress-update", String::from("Checking AppStore Admin UI version"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

      let new_release_info = ReleaseInfo {
        resource_locator: None,
        version: Some(APPSTORE_VERSION.to_string()),
      };

      let current_release_info = manager.get_gui_release_info(&APPSTORE_APP_ID.to_string(), &String::from("default"));

      let new_ui_available = match current_release_info {
        None => true,
        Some(current_info) => current_info.version != new_release_info.version
      };

      if new_ui_available {
        // install new UI
        window.emit("progress-update", String::from("Installing new AppStore Admin UI"))
          .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

        let new_ui = appstore_bundle.web_ui_zip_bytes().await
          .map_err(|e| format!("Failed to get web UI zip bytes from AppStore bundle: {}", e))?;

        manager.install_app_ui(
          APPSTORE_APP_ID.to_string(),
          new_ui.into_owned(),
          &String::from("default"),
          Some(new_release_info),
        )?;

      }

  }

  // Check whether DevHub is installed and if yes, check whether a new UI needs to be installed
  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&DEVHUB_APP_ID.to_string()) == true {

    // emitting signal to the front-end for progress indication
    window.emit("progress-update", String::from("Checking DevHub UI version"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    let new_release_info = ReleaseInfo {
      resource_locator: None,
      version: Some(DEVHUB_VERSION.to_string()),
    };

    let current_release_info = manager.get_gui_release_info(&DEVHUB_APP_ID.to_string(), &String::from("default"));

    let new_ui_available = match current_release_info {
      None => true,
      Some(current_info) => current_info.version != new_release_info.version
    };

    if new_ui_available {
      window.emit("progress-update", String::from("Installing new DevHub UI"))
        .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

      let devhub_bundle = WebAppBundle::decode(include_bytes!("../../../DevHub.webhapp"))
        .or(Err("Malformed DevHub webhapp bundle file"))?;

      // install new UI
      let new_ui = devhub_bundle.web_ui_zip_bytes().await
        .map_err(|e| format!("Failed to get web UI zip bytes from DevHub bundle: {}", e))?;

      manager.install_app_ui(
        DEVHUB_APP_ID.to_string(),
        new_ui.into_owned(),
        &String::from("default"),
        Some(new_release_info),
      )?;

    }
  }



  Ok(())
}

