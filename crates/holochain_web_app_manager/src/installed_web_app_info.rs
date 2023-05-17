use holochain_manager::versions::holochain_conductor_api_latest::AppInfo;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, collections::HashMap};

use crate::ReleaseInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WebUiInfo {
  Headless,
  WebApp {
    path_to_ui: PathBuf,
    app_ui_port: u16,
    gui_release_info: Option<ReleaseInfo>, // B64 hash
  },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledWebAppInfo {
  pub installed_app_info: AppInfo,
  pub happ_release_info: Option<ReleaseInfo>,
  pub web_uis: HashMap<String, WebUiInfo>, // Name of the UI as key. Currently only "default"
  pub icon_src: Option<String>,
}
