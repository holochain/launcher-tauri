use holochain_manager::versions::holochain_conductor_api_latest::AppInfo;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum WebUiInfo {
  Headless,
  WebApp {
    path_to_web_app: PathBuf,
    app_ui_port: u16,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstalledWebAppInfo {
  pub installed_app_info: AppInfo,
  pub web_ui_info: WebUiInfo,
}
