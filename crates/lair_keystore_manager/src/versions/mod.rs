use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

mod init;
mod launch;
pub mod v0_2_0;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq, Deserialize_enum_str, Serialize_enum_str)]
pub enum LairKeystoreVersion {
  #[serde(rename = "0.2.0")]
  V0_2_0,
}

impl Into<String> for LairKeystoreVersion {
  fn into(self) -> String {
    self.to_string()
  }
}
