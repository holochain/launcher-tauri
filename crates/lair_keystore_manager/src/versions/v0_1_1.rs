use std::path::PathBuf;

use async_trait::async_trait;
use url2::Url2;

use super::{
  init::{initialize, is_initialized},
  launch::launch_lair_keystore_process,
  LairKeystoreVersion,
};
use crate::{error::LairKeystoreError, utils::create_dir_if_necessary, LairKeystoreManager};

pub struct LairKeystoreManagerV0_1_1 {
  _keystore_path: PathBuf,
  connection_url: Url2,
  password: String,
}

#[async_trait]
impl LairKeystoreManager for LairKeystoreManagerV0_1_1 {
  fn lair_keystore_version() -> LairKeystoreVersion {
    LairKeystoreVersion::V0_1_1
  }

  fn is_initialized(keystore_path: PathBuf) -> bool {
    is_initialized(keystore_path)
  }

  fn initialize(keystore_path: PathBuf, password: String) -> Result<(), LairKeystoreError> {
    create_dir_if_necessary(&keystore_path)?;

    initialize(keystore_path, password)
  }

  async fn launch(
    log_level: log::Level,
    keystore_path: PathBuf,
    password: String,
  ) -> Result<Self, LairKeystoreError> {
    let connection_url =
      launch_lair_keystore_process(log_level, keystore_path.clone(), password.clone()).await?;

    Ok(LairKeystoreManagerV0_1_1 {
      password,
      connection_url,
      _keystore_path: keystore_path,
    })
  }

  fn connection_url(&self) -> Url2 {
    self.connection_url.clone()
  }

  fn password(&self) -> String {
    self.password.clone()
  }
}
