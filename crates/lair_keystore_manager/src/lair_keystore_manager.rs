use std::path::PathBuf;

use crate::{error::LairKeystoreError, versions::LairKeystoreVersion};
use async_trait::async_trait;
use url2::Url2;
use holochain_zome_types::ZomeCallUnsigned;
pub use holochain_conductor_api::ZomeCall;

#[async_trait]
pub trait LairKeystoreManager: Send + Sync {
  fn lair_keystore_version() -> LairKeystoreVersion
  where
    Self: Sized;

  fn is_initialized(keystore_path: PathBuf) -> bool
  where
    Self: Sized;

  async fn initialize(keystore_path: PathBuf, password: String) -> Result<(), LairKeystoreError>
  where
    Self: Sized;

  async fn launch(
    log_level: log::Level,
    keystore_path: PathBuf,
    password: String,
  ) -> Result<Self, LairKeystoreError>
  where
    Self: Sized;

  fn connection_url(&self) -> Url2;

  async fn sign_zome_call(&self, zome_call: ZomeCallUnsigned) -> Result<ZomeCall, LairKeystoreError>;

  fn password(&self) -> String;
}
