use std::path::PathBuf;

use tauri::api::path::{config_dir, data_dir};
use holochain_manager::versions::HolochainVersion;
use lair_keystore_manager::versions::LairKeystoreVersion;

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

pub fn conductor_config_path(holochain_version: HolochainVersion) -> PathBuf {
  holochain_config_path(holochain_version).join("conductor-config.yml")
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

fn root_data_path() -> PathBuf {
  data_dir()
    .expect("Could not get config dir")
    .join("holochain")
}

fn data_path_for_holochain_version(holochain_version: HolochainVersion) -> PathBuf {
  let version: String = holochain_version.into();

  root_data_path().join(version)
}

pub fn conductor_data_path(holochain_version: HolochainVersion) -> PathBuf {
  data_path_for_holochain_version(holochain_version).join("conductor")
}

pub fn app_ui_path(holochain_version: HolochainVersion, app_id: &String) -> PathBuf {
  uis_data_path(holochain_version).join(app_id)
}

pub fn uis_data_path(holochain_version: HolochainVersion) -> PathBuf {
  data_path_for_holochain_version(holochain_version).join("uis")
}

pub fn caddyfile_path() -> PathBuf {
  root_data_path().join("Caddyfile")
}

pub fn port_mapping_path(holochain_version: HolochainVersion) -> PathBuf {
  root_data_path().join("port_mapping.yml")
}

fn root_lair_path() -> PathBuf {
  data_dir().expect("Could not get config dir").join("lair")
}

pub fn keystore_data_path(lair_keystore_version: LairKeystoreVersion) -> PathBuf {
  let version: String = lair_keystore_version.into();

  root_lair_path().join(version)
}

pub fn pid_file_path() -> PathBuf {
  root_data_path().join("launcher.pid")
}
