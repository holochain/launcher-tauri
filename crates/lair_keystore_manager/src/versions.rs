pub mod v0_0_9;
mod launch;

pub enum LairKeystoreVersion {
  V0_0_9,
}

impl Into<String> for LairKeystoreVersion {
  fn into(self) -> String {
    match self {
      LairKeystoreVersion::V0_0_9 => String::from("v0.0.9"),
    }
  }
}
