use std::path::PathBuf;

use holochain_manager::versions::HolochainVersion;
use lair_keystore_manager::versions::LairKeystoreVersion;
use tauri::api::path::{config_dir, data_dir};

#[derive(Debug)]
pub struct CustomPath {
  pub custom_path: Option<String>
}

impl Clone for CustomPath {
  fn clone(&self) -> Self {
      Self {
        custom_path: self.custom_path.clone()
      }
  }
}

/** Config */

fn component_name(name: &str) -> String {
  if cfg!(debug_assertions) {
    format!("{}-dev", name)
  } else {
    String::from(name)
  }
}

pub fn root_config_path(custom_path: Option<String>) -> PathBuf {
  match custom_path {
    Some(p) => custom_root_path(p).join("config"),
    None => config_dir()
      .expect("Could not get config dir")
      .join(component_name("holochain"))
  }
}

pub fn holochain_config_path(holochain_version: HolochainVersion, custom_path: Option<String>) -> PathBuf {
  let version: String = holochain_version.into();
  root_config_path(custom_path).join(version)
}

pub fn config_environment_path(holochain_version: HolochainVersion, custom_path: Option<String>) -> PathBuf {
  holochain_config_path(holochain_version, custom_path)
}

pub fn launcher_config_path(custom_path: Option<String>) -> PathBuf {
  root_config_path(custom_path).join("launcher-config.yaml")
}

/** Logs */

pub fn logs_path(custom_path: Option<String>) -> PathBuf {
  logs_folder_path(custom_path).join("launcher.log")
}

pub fn logs_folder_path(custom_path: Option<String>) -> PathBuf {
  match custom_path {
    Some(p) => custom_root_path(p).join(component_name("holochain-launcher")),
    None => tauri_data_path().join(component_name("holochain-launcher")),
  }
}

/** Data */

fn tauri_data_path() -> PathBuf {
  data_dir().expect("Could not get config dir")
}

pub fn root_holochain_data_path(custom_path: Option<String>) -> PathBuf {
  match custom_path {
    Some(p) => custom_root_path(p).join(component_name("holochain")),
    None => tauri_data_path().join(component_name("holochain"))
  }
}

pub fn data_path_for_holochain_version(holochain_version: HolochainVersion, custom_path: Option<String>) -> PathBuf {
  let version: String = holochain_version.into();
  root_holochain_data_path(custom_path).join(version)
}

pub fn root_lair_path(custom_path: Option<String>) -> PathBuf {
  match custom_path {
    Some(p) => custom_root_path(p).join(component_name("lair")),
    None => tauri_data_path().join(component_name("lair")),
  }
}

pub fn keystore_data_path(lair_keystore_version: LairKeystoreVersion, custom_path: Option<String>) -> PathBuf {
  let version: String = lair_keystore_version.into();
  root_lair_path(custom_path).join(version)
}


/** Custom root path */
pub fn custom_root_path(folder_name: String) -> PathBuf {
  tauri_data_path().join("custom-holochains").join(folder_name)
}