use std::path::PathBuf;

use enum_dispatch::enum_dispatch;
use lair_keystore_manager::versions::LairKeystoreVersion;
use url2::Url2;

use super::HdkVersion;

#[enum_dispatch]
pub trait VersionManager {
  fn hdk_version(&self) -> HdkVersion;

  fn lair_keystore_version(&self) -> LairKeystoreVersion {
    // For now all holochain versions run the same lair keystore version
    LairKeystoreVersion::V0_1_1
  }

  fn initial_config(
    &self,
    admin_port: u16,
    conductor_environment_path: PathBuf,
    keystore_connection_url: Url2,
  ) -> String;

  fn overwrite_config(
    &self,
    conductor_config: String,
    admin_port: u16,
    keystore_connection_url: Url2,
  ) -> String;
}
