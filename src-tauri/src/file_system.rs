use std::path::PathBuf;

use holochain_manager::versions::HolochainVersion;
use lair_keystore_manager::versions::LairKeystoreVersion;
use tauri::api::path::{config_dir, data_dir};

/** Config */

pub fn root_config_path() -> PathBuf {
  config_dir()
    .expect("Could not get config dir")
    .join("holochain")
}

pub fn holochain_config_path(holochain_version: HolochainVersion) -> PathBuf {
  let version: String = holochain_version.into();

  root_config_path().join(version)
}

pub fn config_environment_path(holochain_version: HolochainVersion) -> PathBuf {
  holochain_config_path(holochain_version)
}

/** Logs */

pub fn logs_path() -> PathBuf {
  logs_folder_path().join("launcher.log")
}

pub fn logs_folder_path() -> PathBuf {
  data_dir()
    .expect("Could not get config dir")
    .join("holochain-launcher")
}

/** Data */

pub fn root_data_path() -> PathBuf {
  data_dir()
    .expect("Could not get config dir")
    .join("holochain")
}

pub fn data_path_for_holochain_version(holochain_version: HolochainVersion) -> PathBuf {
  let version: String = holochain_version.into();

  root_data_path().join(version)
}

pub fn root_lair_path() -> PathBuf {
  data_dir().expect("Could not get config dir").join("lair")
}

pub fn keystore_data_path(lair_keystore_version: LairKeystoreVersion) -> PathBuf {
  let version: String = lair_keystore_version.into();

  root_lair_path().join(version)
}
