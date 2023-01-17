use enum_dispatch::enum_dispatch;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use std::path::PathBuf;
use url2::Url2;

// NEW_VERSION: Upgrade these two crates so that they refer to the latest version of them
pub use holochain_conductor_api_0_1_0BetaRc3 as holochain_conductor_api_latest;
pub use holochain_types_0_1_0BetaRc3 as holochain_types_latest;

use lair_keystore_manager::versions::LairKeystoreVersion;
pub use mr_bundle as mr_bundle_latest;

mod common;
pub mod version_manager;
use version_manager::VersionManager;

// NEW_VERSION: Create a new VersionManager for the new version,
// by copying one of the version manager files in this folder (eg. v0_0_127)
// Import the new VersionManager here


pub mod v0_1_0_beta_rc3;

use v0_1_0_beta_rc3::HolochainV0_1_0BetaRc3;


// NEW_VERSION: Add the new HDK version to this enum (if there is a new HDK version)
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HdkVersion {
  #[serde(rename = "0.1.0-beta-rc.2")]
  V0_1_0BetaRc2,
}

impl Into<String> for HdkVersion {
  fn into(self) -> String {
    self.to_string()
  }
}

// NEW_VERSION: Add the new HDK version to this enum (if there is a new HDK version)
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HdiVersion {
  #[serde(rename = "0.2.0-beta-rc.2")]
  V0_2_0BetaRc2,
}

impl Into<String> for HdiVersion {
  fn into(self) -> String {
    self.to_string()
  }
}

// NEW_VERSION: Add the new Holochain version to this enum
#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum HolochainVersion {
  #[serde(rename = "Custom Binary")]
  CustomBinary,
  #[serde(rename = "0.1.0-beta-rc.3")]
  V0_1_0BetaRc3,
  // !!! ONLY USE MINOR VERSION HERE ONCE BETA 0.1.0 IS OUT since this affects the folder structure, i.e. whether
  // the same conductor database will be used across holochain bumps
}

impl Into<String> for HolochainVersion {
  fn into(self) -> String {
    self.to_string()
  }
}

impl HolochainVersion {

  // Will be the config with which the custom binary is run, when present
  pub fn custom() -> HolochainVersion {
    HolochainVersion::CustomBinary
  }
  // Will be run by default when the launcher starts and is the version where the DevHub is installed
  // Not necessarily the newest one
  // NEW_VERSION Switch devhub holochain version in case there is a new version of the devhub
  pub fn default() -> HolochainVersion {
    HolochainVersion::V0_1_0BetaRc3
  }

  // NEW_VERSION (latest() is currently unused)
  pub fn latest() -> HolochainVersion {
    HolochainVersion::V0_1_0BetaRc3
  }

  pub fn supported_versions() -> Vec<HolochainVersion> {
    // NEW_VERSION: Add the new version to this array
    return vec![
      HolochainVersion::V0_1_0BetaRc3,
    ];
  }

  pub fn manager(&self) -> HolochainVersionManager {
    // NEW_VERSION: Create a new version manager, duplicating one of the files in this folder
    // Then, import and add the new version manager here
    match self { // NEW_VERSION assume the latest version for the custom binary
      HolochainVersion::CustomBinary => HolochainVersionManager::HolochainV0_1_0BetaRc3(HolochainV0_1_0BetaRc3),
      HolochainVersion::V0_1_0BetaRc3 => HolochainVersionManager::HolochainV0_1_0BetaRc3(HolochainV0_1_0BetaRc3),
    }
  }
}

// NEW_VERSION: Add the new version manager to this enum
#[enum_dispatch(VersionManager)]
pub enum HolochainVersionManager {
  HolochainV0_1_0BetaRc3,
}
