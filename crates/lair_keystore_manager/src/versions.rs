mod init;
mod launch;
pub mod v0_1_0;

pub enum LairKeystoreVersion {
  V0_1_0,
}

impl Into<String> for LairKeystoreVersion {
  fn into(self) -> String {
    match self {
      LairKeystoreVersion::V0_1_0 => String::from("v0.1.0"),
    }
  }
}
