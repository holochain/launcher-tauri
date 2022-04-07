use enum_dispatch::enum_dispatch;
use std::path::PathBuf;
use url2::Url2;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

pub use holochain_conductor_api_0_0_132 as holochain_conductor_api_latest;
pub use holochain_types_0_0_132 as holochain_types_latest;
use lair_keystore_manager::versions::LairKeystoreVersion;
pub use mr_bundle_0_0_9 as mr_bundle_latest;

pub mod v0_0_127;
pub mod v0_0_132;
pub mod version_manager;

use version_manager::VersionManager;

use v0_0_127::HolochainV0_0_127;
use v0_0_132::HolochainV0_0_132;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HdkVersion {
  #[serde(rename = "0.0.123")]
  V0_0_123,
  #[serde(rename = "0.0.127")]
  V0_0_127,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HolochainVersion {
  #[serde(rename = "0.0.127")]
  V0_0_127,
  #[serde(rename = "0.0.132")]
  V0_0_132,
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

  pub fn supported_versions() -> Vec<HolochainVersion> {
    return vec![HolochainVersion::V0_0_127, HolochainVersion::V0_0_132];
  }

  pub fn manager(&self) -> HolochainVersionManager {
    match self {
      HolochainVersion::V0_0_127 => HolochainVersionManager::HolochainV0_0_127(HolochainV0_0_127),
      HolochainVersion::V0_0_132 => HolochainVersionManager::HolochainV0_0_132(HolochainV0_0_132),
    }
  }
}


#[enum_dispatch(VersionManager)]
pub enum HolochainVersionManager {
  HolochainV0_0_127,
  HolochainV0_0_132,
}
