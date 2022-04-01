use log::Level;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::file_system::launcher_config_path;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  pub log_level: Level,
}

impl Default for LauncherConfig {
  fn default() -> Self {
    LauncherConfig {
      log_level: log::Level::Info,
    }
  }
}

impl LauncherConfig {
  pub fn read() -> Result<LauncherConfig, String> {
    match fs::read_to_string(launcher_config_path()) {
      Ok(str) => serde_yaml::from_str::<LauncherConfig>(str.as_str())
        .map_err(|err| format!("Could not read launcher config: {}", err)),
      Err(_) => Ok(LauncherConfig::default()),
    }
  }

  pub fn write(&self) -> Result<(), String> {
    let serde_config = serde_yaml::to_string(&self).expect("Could not serialize launcher config");

    fs::write(launcher_config_path(), serde_config)
      .map_err(|err| format!("Could not write LauncherConfig: {}", err))
  }
}
