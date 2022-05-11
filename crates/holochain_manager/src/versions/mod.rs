use enum_dispatch::enum_dispatch;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::path::PathBuf;
use url2::Url2;

// NEW_VERSION: Upgrade these two crates so that they refer to the latest version of them
pub use holochain_conductor_api_0_0_132 as holochain_conductor_api_latest;
pub use holochain_types_0_0_132 as holochain_types_latest;

use lair_keystore_manager::versions::LairKeystoreVersion;
pub use mr_bundle_0_0_9 as mr_bundle_latest;

mod common;
pub mod version_manager;
use version_manager::VersionManager;

// NEW_VERSION: Import the new VersionManager
pub mod v0_0_127;
pub mod v0_0_131;
pub mod v0_0_132;
pub mod v0_0_136;

use v0_0_127::HolochainV0_0_127;
use v0_0_131::HolochainV0_0_131;
use v0_0_132::HolochainV0_0_132;
use v0_0_136::HolochainV0_0_136;

// NEW_VERSION: Add the new HDK version to this enum (if there is a new HDK version)
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HdkVersion {
  #[serde(rename = "0.0.123")]
  V0_0_123,
  #[serde(rename = "0.0.126")]
  V0_0_126,
  #[serde(rename = "0.0.127")]
  V0_0_127,
  #[serde(rename = "0.0.130")]
  V0_0_130,
}

// NEW_VERSION: Add the new Holochain version to this enum
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HolochainVersion {
  #[serde(rename = "0.0.127")]
  V0_0_127,
  #[serde(rename = "0.0.131")]
  V0_0_131,
  #[serde(rename = "0.0.132")]
  V0_0_132,
  #[serde(rename = "0.0.136")]
  V0_0_136,
}

impl Into<String> for HolochainVersion {
  fn into(self) -> String {
    self.to_string()
  }
}

impl HolochainVersion {
  // Will be run by default when the launcher starts and
  pub fn default() -> HolochainVersion {
    HolochainVersion::V0_0_132
  }

  // Will be the config with the the custom binary is run, when present
  pub fn latest() -> HolochainVersion {
    HolochainVersion::V0_0_136
  }

  pub fn supported_versions() -> Vec<HolochainVersion> {
    // NEW_VERSION: Add the new version to this array
    return vec![
      HolochainVersion::V0_0_127,
      HolochainVersion::V0_0_131,
      HolochainVersion::V0_0_132,
      HolochainVersion::V0_0_136,
    ];
  }

  pub fn manager(&self) -> HolochainVersionManager {
    // NEW_VERSION: Create a new version manager, duplicating one of the files in this folder
    // Then, import and add the new version manager here
    match self {
      HolochainVersion::V0_0_127 => HolochainVersionManager::HolochainV0_0_127(HolochainV0_0_127),
      HolochainVersion::V0_0_131 => HolochainVersionManager::HolochainV0_0_131(HolochainV0_0_131),
      HolochainVersion::V0_0_132 => HolochainVersionManager::HolochainV0_0_132(HolochainV0_0_132),
      HolochainVersion::V0_0_136 => HolochainVersionManager::HolochainV0_0_136(HolochainV0_0_136),
    }
  }
}

// NEW_VERSION: Add the new version manager to this enum
#[enum_dispatch(VersionManager)]
pub enum HolochainVersionManager {
  HolochainV0_0_127,
  HolochainV0_0_131,
  HolochainV0_0_132,
  HolochainV0_0_136,
}
