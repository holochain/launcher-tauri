use enum_dispatch::enum_dispatch;
use std::collections::HashMap;

pub mod launch;
pub mod v0_0_131;

pub use holochain_conductor_api_0_0_131 as holochain_conductor_api_latest;
pub use holochain_types_0_0_131 as holochain_types_latest;
use lair_keystore_manager::versions::LairKeystoreVersion;
pub use mr_bundle_0_0_9 as mr_bundle_latest;
use serde::{Deserialize, Serialize};
use crate::HolochainManager;
use crate::{config::LaunchHolochainConfig, error::LaunchHolochainError};
use self::v0_0_131::HolochainManagerV0_0_131;

use holochain_types_latest::prelude::*;
use holochain_conductor_api_latest::InstalledAppInfo;


pub enum HdkVersion {
  V0_0_126,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum HolochainVersion {
  V0_0_131,
}

impl HolochainVersion {
  pub fn hdk_version(&self) -> HdkVersion {
    match self {
      HolochainVersion::V0_0_131 => HdkVersion::V0_0_126,
    }
  }

  pub fn lair_keystore_version(&self) -> LairKeystoreVersion {
    match self {
      HolochainVersion::V0_0_131 => LairKeystoreVersion::V0_1_0,
    }
  }

  pub fn latest() -> HolochainVersion {
    HolochainVersion::V0_0_131
  }

  pub fn supported_versions() -> Vec<HolochainVersion> {
    return vec![HolochainVersion::V0_0_131];
  }
}

impl Into<String> for HolochainVersion {
  fn into(self) -> String {
    match self {
      HolochainVersion::V0_0_131 => String::from("v0.0.131"),
    }
  }
}

impl TryFrom<String> for HolochainVersion {
  type Error = String;
  fn try_from(s: String) -> Result<HolochainVersion, String> {
    match s.as_str() {
      "v0.0.131" => Ok(HolochainVersion::V0_0_131),
      _ => Err(format!("Bad Holochain version")),
    }
  }
}

#[enum_dispatch(HolochainManager)]
pub enum HolochainManagerEnum {
  HolochainManagerV0_0_131,
}

pub async fn launch_holochain(
  holochain_version: HolochainVersion,
  config: LaunchHolochainConfig,
  password: String,
) -> Result<HolochainManagerEnum, LaunchHolochainError> {
  match holochain_version {
    HolochainVersion::V0_0_131 => Ok(HolochainManagerEnum::HolochainManagerV0_0_131(
      HolochainManagerV0_0_131::launch(config, password).await?,
    )),
  }
}
