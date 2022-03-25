use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebUiInfo {
  pub path_to_web_app: PathBuf,
  pub app_ui_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AppUiInfo {
  Headless,
  WebApp(WebUiInfo),
}

pub type RunningApps = HashMap<String, AppUiInfo>;
