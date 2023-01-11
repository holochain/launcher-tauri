use holochain_manager::versions::HolochainVersion;
use log::Level;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs};

use crate::file_system::{profile_config_dir, Profile, launcher_config_path};

use super::error::LauncherError;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  pub log_level: Level,
  pub custom_binary_path: Option<String>,

  pub running_versions: HashSet<HolochainVersion>,
  profile: Profile,
}

impl Default for LauncherConfig {
  fn default() -> Self {
    LauncherConfig {
      log_level: log::Level::Warn,
      custom_binary_path: None,
      running_versions: HashSet::from([HolochainVersion::default()]),
      profile: String::from("default"),
    }
  }
}



impl LauncherConfig {
  pub fn new(profile: Profile) -> Self {
    LauncherConfig {
      log_level: log::Level::Warn,
      custom_binary_path: None,
      running_versions: HashSet::from([HolochainVersion::default()]),
      profile: profile,
    }
  }

  pub fn read(profile: Profile) -> LauncherConfig {

    let config_path = match launcher_config_path(profile.clone()) {
      Ok(path) => path,
      Err(e) => {
        log::error!("Warning: Found no launcher config file at expected path. Generating default config instead.");
        return LauncherConfig::new(profile.clone());
      }
    };

    match fs::read_to_string(config_path) {
      Ok(str) => {
        match serde_yaml::from_str::<LauncherConfig>(str.as_str()){
          Ok(config) => config,
          Err(e) => {
            log::error!("Failed to read launcher config to string: {}. Generating default config instead.", e);
            LauncherConfig::new(profile)
          }
        }
      }
      Err(_) => {
        let config = LauncherConfig::new(profile);
        if let Err(e) = config.write() {
          log::error!("Failed to write launcher config: {}.", e);
        }
        config
      }
    }
  }

  pub fn write(&self) -> Result<(), LauncherError> {
    let serde_config = serde_yaml::to_string(&self).expect("Could not serialize launcher config");

    fs::write(launcher_config_path(self.custom_path.clone()), serde_config)
      .map_err(|err| LauncherError::ConfigError(format!("{}", err)))
  }
}
