use holochain_manager::versions::holochain_conductor_api_latest::AppInfo;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WebUiInfo {
  Headless,
  WebApp {
    path_to_ui: PathBuf,
    gui_release_hash: Option<String>, // B64 hash
  },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledWebAppInfo {
  pub installed_app_info: AppInfo,
  pub happ_release_hash: Option<String>,
  pub web_uis: HashMap<String, WebUiInfo>, // Name of the UI as key. Currently only "default"
}
