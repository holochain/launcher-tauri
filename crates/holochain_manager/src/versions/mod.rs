use enum_dispatch::enum_dispatch;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::path::PathBuf;
use url2::Url2;

// NEW_VERSION: Upgrade these two crates so that they refer to the latest version of them
pub use holochain_conductor_api_0_0_156 as holochain_conductor_api_latest;
pub use holochain_types_0_0_156 as holochain_types_latest;

use lair_keystore_manager::versions::LairKeystoreVersion;
pub use mr_bundle as mr_bundle_latest;

mod common;
pub mod version_manager;
use version_manager::VersionManager;

// NEW_VERSION: Create a new VersionManager for the new version,
// by copying one of the version manager files in this folder (eg. v0_0_127)
// Import the new VersionManager here


pub mod v0_0_145;
pub mod v0_0_150;
pub mod v0_0_152;
pub mod v0_0_156;

use v0_0_145::HolochainV0_0_145;
use v0_0_150::HolochainV0_0_150;
use v0_0_152::HolochainV0_0_152;
use v0_0_156::HolochainV0_0_156;


// NEW_VERSION: Add the new HDK version to this enum (if there is a new HDK version)
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HdkVersion {
  #[serde(rename = "0.0.138")]
  V0_0_138,
  #[serde(rename = "0.0.142")]
  V0_0_142,
  #[serde(rename = "0.0.144")]
  V0_0_144,
  #[serde(rename = "0.0.147")]
  V0_0_147,
}

// NEW_VERSION: Add the new Holochain version to this enum
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HolochainVersion {
  #[serde(rename = "0.0.145")]
  V0_0_145,
  #[serde(rename = "0.0.150")]
  V0_0_150,
  #[serde(rename = "0.0.152")]
  V0_0_152,
  #[serde(rename = "0.0.156")]
  V0_0_156,
}

impl Into<String> for HolochainVersion {
  fn into(self) -> String {
    self.to_string()
  }
}

impl HolochainVersion {
  // Will be run by default when the launcher starts and is the version where the DevHub is installed
  // Not necessarily the newest one
  // NEW_VERSION Switch devhub holochain version in case there is a new version of the devhub
  pub fn default() -> HolochainVersion {
    HolochainVersion::V0_0_156
  }

  // Will be the config with the the custom binary is run, when present
  pub fn latest() -> HolochainVersion {
    HolochainVersion::V0_0_156
  }

  pub fn supported_versions() -> Vec<HolochainVersion> {
    // NEW_VERSION: Add the new version to this array
    return vec![
      HolochainVersion::V0_0_145,
      HolochainVersion::V0_0_150,
      HolochainVersion::V0_0_152,
      HolochainVersion::V0_0_156,
    ];
  }

  pub fn manager(&self) -> HolochainVersionManager {
    // NEW_VERSION: Create a new version manager, duplicating one of the files in this folder
    // Then, import and add the new version manager here
    match self {
      HolochainVersion::V0_0_145 => HolochainVersionManager::HolochainV0_0_145(HolochainV0_0_145),
      HolochainVersion::V0_0_150 => HolochainVersionManager::HolochainV0_0_150(HolochainV0_0_150),
      HolochainVersion::V0_0_152 => HolochainVersionManager::HolochainV0_0_152(HolochainV0_0_152),
      HolochainVersion::V0_0_156 => HolochainVersionManager::HolochainV0_0_156(HolochainV0_0_156),
    }
  }
}

// NEW_VERSION: Add the new version manager to this enum
#[enum_dispatch(VersionManager)]
pub enum HolochainVersionManager {
  HolochainV0_0_145,
  HolochainV0_0_150,
  HolochainV0_0_152,
  HolochainV0_0_156,
}
