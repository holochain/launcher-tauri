use std::path::PathBuf;
use lair_keystore_api::prelude::*;
use lair_keystore_api::ipc_keystore::*;
use holochain_types::prelude::ZomeCallUnsigned;
use holochain_conductor_api::ZomeCall;
use holochain_zome_types::Signature;

use async_trait::async_trait;
use url2::Url2;
use url::Url;

use super::{
  init::{initialize, is_initialized},
  launch::launch_lair_keystore_process,
  LairKeystoreVersion,
};
use crate::{error::LairKeystoreError, utils::create_dir_if_necessary, LairKeystoreManager};

pub struct LairKeystoreManagerV0_2 {
  _keystore_path: PathBuf,
  connection_url: Url2,
  password: String,
  client: LairClient,
}

#[async_trait]
impl LairKeystoreManager for LairKeystoreManagerV0_2 {
  fn lair_keystore_version() -> LairKeystoreVersion {
    LairKeystoreVersion::V0_2
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


    Ok(LairKeystoreManagerV0_2 {
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

    let pub_key = unsigned_zome_call.provenance.clone();
    let mut pub_key_2 = [0; 32];
    pub_key_2.copy_from_slice(pub_key.get_raw_32());

    let data_to_sign = unsigned_zome_call.data_to_sign()
      .map_err(|e| LairKeystoreError::SignZomeCallError(e.into()))?;

    let sig = self.client.sign_by_pub_key(
      pub_key_2.into(),
       None,
      data_to_sign)
      .await
      .map_err(|_e| LairKeystoreError::SignZomeCallError(String::from("Failed to sign by public key.")))?;

    let signature = Signature(*sig.0);

    let signed_zome_call = ZomeCall {
      cell_id: unsigned_zome_call.cell_id,
      zome_name: unsigned_zome_call.zome_name,
      fn_name: unsigned_zome_call.fn_name,
      payload: unsigned_zome_call.payload,
      cap_secret: unsigned_zome_call.cap_secret,
      provenance: unsigned_zome_call.provenance,
      nonce: unsigned_zome_call.nonce,
      expires_at: unsigned_zome_call.expires_at,
      signature
    };

    Ok(signed_zome_call)
  }

  fn password(&self) -> String {
    self.password.clone()
  }
}
