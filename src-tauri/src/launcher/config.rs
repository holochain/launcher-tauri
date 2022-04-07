use holochain_manager::versions::HolochainVersion;
use log::Level;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::file_system::launcher_config_path;

use super::error::LauncherError;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  pub log_level: Level,

  pub running_versions: Vec<HolochainVersion>,
  pub version_for_devhub: HolochainVersion,
}

impl Default for LauncherConfig {
  fn default() -> Self {
    LauncherConfig {
      log_level: log::Level::Info,
      running_versions: vec![HolochainVersion::default()],
      version_for_devhub: HolochainVersion::default(),
    }
  }
}

impl LauncherConfig {
  pub fn read() -> LauncherConfig {
    match fs::read_to_string(launcher_config_path()) {
      Ok(str) => {
        serde_yaml::from_str::<LauncherConfig>(str.as_str()).unwrap_or(LauncherConfig::default())
      }
      Err(_) => LauncherConfig::default(),
    }
  }

  pub fn write(&self) -> Result<(), LauncherError> {
    let serde_config = serde_yaml::to_string(&self).expect("Could not serialize launcher config");

    fs::write(launcher_config_path(), serde_config)
      .map_err(|err| LauncherError::ConfigError(format!("{}", err)))
  }
}
