use serde::{Serialize, Deserialize};

mod init;
mod launch;
pub mod v0_1_0;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum LairKeystoreVersion {
  #[serde(rename = "0.1.0")]
  V0_1_0,
}

impl Into<String> for LairKeystoreVersion {
  fn into(self) -> String {
    match self {
      LairKeystoreVersion::V0_1_0 => String::from("v0.1.0"),
    }
  }
}
