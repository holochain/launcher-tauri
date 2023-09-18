use std::collections::HashMap;

use holochain_manager::versions::holochain_types_latest::web_app::WebAppBundle;
use holochain_web_app_manager::{WebAppManager, ReleaseInfo};

use crate::commands::install_devhub::install_devhub_inner;
// use holochain_manager::versions::version_manager::VersionManager;


// Versions of Appstore and Devhub (commit hash of Github release of the UI or shasum of the .webhapp file).
// Those are used to install new Appstore/DevHub UI's if necessary
// NEW_VERSION Change appstore and devhub versions here if required
const APPSTORE_VERSION: &str = "048f0e820f1049f2c4a72c8bebde51460cabd947"; // shasum
pub const DEVHUB_VERSION: &str = "39dafa29e0a29ee927d2aaf92ffabfbe1e555266"; // shasum

const APPSTORE_APP_ID: &str = "AppStore NEW";
const OLD_APPSTORE_APP_ID: &str = "AppStore";
pub const DEVHUB_APP_ID: &str = "DevHub NEW";
const OLD_DEVHUB_APP_ID: &str = "DevHub";

const APPSTORE_NETWORK_SEED: &str = "launcher";
pub const DEVHUB_NETWORK_SEED: &str = "launcher";



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

    let network_seed = if cfg!(debug_assertions) { Some(String::from("launcher-dev")) } else { Some(APPSTORE_NETWORK_SEED.into()) };

    let happ_release_info = ReleaseInfo {
      resource_locator: None,
      version: Some(APPSTORE_VERSION.to_string()),
    };

    let gui_release_info = ReleaseInfo {
      resource_locator: None,
      version: Some(APPSTORE_VERSION.to_string()),
    };

    manager
      .install_web_app(
        APPSTORE_APP_ID.to_string(),
        appstore_bundle,
        network_seed,
        HashMap::new(),
        None,
        Some(happ_release_info),
        Some(gui_release_info),
      )
      .await?;

    // try disabling old appstore
    // emitting signal to the front-end for progress indication
    window.emit("progress-update", String::from("Disabling previous AppStore"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;
    match manager.disable_app(OLD_APPSTORE_APP_ID.into()).await {
      Ok(()) => (),
      Err(_) => (),
    }

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

  // Check whether old DevHub is already installed and if yes and new devhub is not yet installed, install new one
  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&OLD_DEVHUB_APP_ID.to_string())
    && !apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&DEVHUB_APP_ID.to_string())
  {
    window.emit("progress-update", String::from("Installing new DevHub"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;
    install_devhub_inner(manager).await?;
  }

  // Check whether DevHub is installed and if yes, check whether a new UI needs to be installed
  if apps.iter()
    .map(|info| info.installed_app_info.installed_app_id.clone())
    .collect::<Vec<String>>()
    .contains(&DEVHUB_APP_ID.to_string()) {

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

