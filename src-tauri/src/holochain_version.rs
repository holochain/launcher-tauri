pub use holochain_types_0_0_130 as holochain_types_latest;
use serde::{Deserialize, Serialize};

pub enum HdkVersion {
  V0_0_125,
}

pub enum LairKeystoreVersion {
  V0_0_9,
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
}

impl Into<String> for HolochainVersion {
  fn into(self) -> String {
    match self {
      HolochainVersion::V0_0_130 => String::from("v0.0.130"),
    }
  }
}

impl Into<String> for LairKeystoreVersion {
  fn into(self) -> String {
    match self {
      LairKeystoreVersion::V0_0_9 => String::from("v0.0.9"),
    }
  }
}
