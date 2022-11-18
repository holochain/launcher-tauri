
use lair_keystore_api::prelude::ipc_keystore_connect;

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



// let client = ipc_keystore_connect(Url::from(connection_url.clone()), password.clone().into_bytes())
// .await
// .map_err(|_e| LairKeystoreError::ErrorCreatingLairClient(String::from("Failed to create LairClient.")))?;
