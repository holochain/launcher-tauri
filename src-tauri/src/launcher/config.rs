use holochain_manager::versions::HolochainVersion;
use holochain_manager::versions::common::{bootstrap_service, signaling_server};

use log::Level;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs};

use crate::{file_system::{Profile, launcher_config_path}, SignalingServerUrl, BootstrapServerUrl};

use super::error::LauncherError;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  pub log_level: Level,
  pub custom_binary_path: Option<String>,
  pub signaling_server_url: SignalingServerUrl,
  pub bootstrap_server_url: BootstrapServerUrl,

  pub running_versions: HashSet<HolochainVersion>,
  pub profile: Profile,
}

impl Default for LauncherConfig {
  fn default() -> Self {
    LauncherConfig {
      log_level: log::Level::Warn,
      custom_binary_path: None,
      running_versions: HashSet::from([HolochainVersion::default()]),
      profile: String::from("default"),
      bootstrap_server_url: Some(bootstrap_service().to_string()),
      signaling_server_url: Some(signaling_server()),
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
      bootstrap_server_url: Some(bootstrap_service().to_string()),
      signaling_server_url: Some(signaling_server()),
    }
  }

  pub fn read(profile: Profile) -> LauncherConfig {

    let config_path = match launcher_config_path(profile.clone()) {
      Ok(path) => path,
      Err(e) => {
        log::error!("Warning: Found no launcher config file at expected path: {:?}.\nGenerating default config instead.", e);
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
    let config_path = launcher_config_path(self.profile.clone())?;

    fs::write(config_path, serde_config)
      .map_err(|err| LauncherError::ConfigError(format!("{}", err)))
  }
}
