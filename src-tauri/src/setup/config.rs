
use std::{fs, path::PathBuf};
use tauri::api::path::{config_dir, data_dir};
use url2;

pub fn logs_path() -> PathBuf {
  logs_folder_path().join("launcher.log")
}

pub fn logs_folder_path() -> PathBuf {
  data_dir()
    .expect("Could not get config dir")
    .join("holochain-launcher")
}
