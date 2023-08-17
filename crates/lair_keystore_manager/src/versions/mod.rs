use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

mod init;
mod launch;
pub mod v0_3;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum LairKeystoreVersion {
  #[serde(rename = "0.3")]
  V0_3,
}

impl Into<String> for LairKeystoreVersion {
  fn into(self) -> String {
    self.to_string()
  }
}
