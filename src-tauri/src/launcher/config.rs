use holochain_manager::versions::HolochainVersion;
use log::Level;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs};

use crate::file_system::launcher_config_path;

use super::error::LauncherError;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  pub log_level: Level,
  pub custom_binary_path: Option<String>,

  pub running_versions: HashSet<HolochainVersion>,
}

impl Default for LauncherConfig {
  fn default() -> Self {
    LauncherConfig {
      log_level: log::Level::Warn,
      custom_binary_path: None,
      running_versions: HashSet::from([HolochainVersion::default()]),
    }
  }
}

impl LauncherConfig {
  pub fn read() -> LauncherConfig {
    match fs::read_to_string(launcher_config_path()) {
      Ok(str) => {
        serde_yaml::from_str::<LauncherConfig>(str.as_str()).unwrap_or(LauncherConfig::default())
      }
      Err(_) => {
        let config = LauncherConfig::default();
        config.write().expect("Could not write launcher config");
        config
      }
    }
  }

  pub fn write(&self) -> Result<(), LauncherError> {
    let serde_config = serde_yaml::to_string(&self).expect("Could not serialize launcher config");

    fs::write(launcher_config_path(), serde_config)
      .map_err(|err| LauncherError::ConfigError(format!("{}", err)))
  }
}
