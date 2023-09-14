use std::path::PathBuf;
use lair_keystore_api::prelude::*;
use lair_keystore_api::ipc_keystore::*;
use holochain_types::prelude::ZomeCallUnsigned;
use holochain_conductor_api::ZomeCall;
use holochain_launcher_utils::zome_call_signing::sign_zome_call_with_client;

use async_trait::async_trait;
use url2::Url2;
use url::Url;

use super::{
  init::{initialize, is_initialized},
  launch::launch_lair_keystore_process,
  LairKeystoreVersion,
};
use crate::{error::LairKeystoreError, utils::create_dir_if_necessary, LairKeystoreManager};

pub struct LairKeystoreManagerV0_3 {
  _keystore_path: PathBuf,
  connection_url: Url2,
  password: String,
  client: LairClient,
}

#[async_trait]
impl LairKeystoreManager for LairKeystoreManagerV0_3 {
  fn lair_keystore_version() -> LairKeystoreVersion {
    LairKeystoreVersion::V0_3
  }

  fn is_initialized(keystore_path: PathBuf) -> bool {
    is_initialized(keystore_path)
  }

  async fn initialize(keystore_path: PathBuf, password: String) -> Result<(), LairKeystoreError> {
    create_dir_if_necessary(&keystore_path)?;
    initialize(keystore_path, password).await
  }

  async fn launch(
    log_level: log::Level,
    keystore_path: PathBuf,
    password: String,
  ) -> Result<Self, LairKeystoreError> {
    let connection_url =
      launch_lair_keystore_process(log_level, keystore_path.clone(), password.clone()).await?;

    let client = ipc_keystore_connect(Url::from(connection_url.clone()), password.clone().into_bytes())
      .await
      .map_err(|e| LairKeystoreError::ErrorCreatingLairClient(format!("Failed to create LairClient: {:?}", e)))?;


    Ok(LairKeystoreManagerV0_3 {
      password,
      connection_url,
      _keystore_path: keystore_path,
      client,
    })
  }

  fn connection_url(&self) -> Url2 {
    self.connection_url.clone()
  }

  async fn sign_zome_call(&self, unsigned_zome_call: ZomeCallUnsigned) -> Result<ZomeCall, LairKeystoreError> {
    sign_zome_call_with_client(
      unsigned_zome_call,
      &self.client,
    )
    .await
    .map_err(|e| LairKeystoreError::SignZomeCallError(e))
  }

  fn password(&self) -> String {
    self.password.clone()
  }
}
