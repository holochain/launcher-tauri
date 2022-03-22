pub mod launch;
mod utils;
pub mod v0_0_130;

use std::path::PathBuf;

pub use holochain_types_0_0_130 as holochain_types_latest;
use lair_keystore_manager::LairKeystoreVersion;
use serde::{Deserialize, Serialize};

use crate::{error::LaunchHolochainError, HolochainManager};

use self::v0_0_130::HolochainManagerV0_0_130;

pub enum HdkVersion {
    V0_0_125,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum HolochainVersion {
    V0_0_130,
}

impl HolochainVersion {
    pub fn hdk_version(&self) -> HdkVersion {
        match self {
            HolochainVersion::V0_0_130 => HdkVersion::V0_0_125,
        }
    }

    pub fn lair_keystore_version(&self) -> LairKeystoreVersion {
        match self {
            HolochainVersion::V0_0_130 => LairKeystoreVersion::V0_0_9,
        }
    }

    pub fn latest() -> HolochainVersion {
        HolochainVersion::V0_0_130
    }

    pub fn supported_versions() -> Vec<HolochainVersion> {
        return vec![HolochainVersion::V0_0_130];
    }
}


impl Into<String> for HolochainVersion {
  fn into(self) -> String {
    match self {
      HolochainVersion::V0_0_130 => String::from("v0.0.130"),
    }
  }
}

pub async fn launch_holochain(
    holochain_version: HolochainVersion,
    log_level: log::Level,
    admin_port: u16,
    conductor_config_path: PathBuf,
    environment_path: PathBuf,
    keystore_path: PathBuf,
) -> Result<Box<dyn HolochainManager>, LaunchHolochainError> {
    match holochain_version {
        HolochainVersion::V0_0_130 => Ok(Box::new(
            HolochainManagerV0_0_130::launch(
                log_level,
                admin_port,
                conductor_config_path,
                environment_path,
                keystore_path,
            )
            .await?,
        )),
    }
}
