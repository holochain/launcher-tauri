use holochain_client::AgentPubKey;
use holochain_types::prelude::ZomeCallUnsigned;
use holochain_zome_types::{Signature, CellId, ZomeName, FunctionName, CapSecret, ExternIO, Timestamp};
use holochain_conductor_api::ZomeCall;
use lair_keystore_api::LairClient;
use std::collections::HashMap;
use serde::Deserialize;

use crate::error::HcLaunchError;

#[derive(Deserialize, Debug)]
pub struct ZomeCallUnsignedTauri {
  pub provenance: AgentPubKey,
  pub cell_id: CellId,
  pub zome_name: ZomeName,
  pub fn_name: FunctionName,
  pub cap_secret: Option<CapSecret>,
  pub payload: ExternIO,
  pub nonce: [u8; 32],
  pub expires_at: Timestamp,
}

#[tauri::command]
pub async fn sign_zome_call(
  window: tauri::Window,
  state: tauri::State<'_, HashMap<String, LairClient>>,
  zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, HcLaunchError> {

  let window_label = window.label().to_string();

  // if window_label != "admin" {
  //   () // this function is allowed to be called by any window
  // }

  // convert nonce to byte array [u8, 32], required because nonce seems to have "non-serde" deserialize behavior
  let zome_call_unsigned_converted = ZomeCallUnsigned {
    provenance: zome_call_unsigned.provenance,
    cell_id: zome_call_unsigned.cell_id,
    zome_name: zome_call_unsigned.zome_name,
    fn_name: zome_call_unsigned.fn_name,
    cap_secret: zome_call_unsigned.cap_secret,
    payload: zome_call_unsigned.payload,
    nonce: zome_call_unsigned.nonce.into(),
    expires_at: zome_call_unsigned.expires_at,
  };

  // get the right lair client from the hashmap
  let client = (*state).get(&window_label)
    .expect(format!("No lair client for this window with label '{}'", window_label).as_str());

  // sign the zome call
  let pub_key = zome_call_unsigned_converted.provenance.clone();
  let mut pub_key_2 = [0; 32];
  pub_key_2.copy_from_slice(pub_key.get_raw_32());

  let data_to_sign = zome_call_unsigned_converted.data_to_sign()
    .map_err(|e| HcLaunchError::DataToSignError(e.into()))?;

  let sig = client.sign_by_pub_key(
    pub_key_2.into(),
     None,
    data_to_sign)
    .await
    .map_err(|e| HcLaunchError::SignZomeCallError(e.str_kind().to_string()))?;

  let signature = Signature(*sig.0);

  let signed_zome_call = ZomeCall {
    cell_id: zome_call_unsigned_converted.cell_id,
    zome_name: zome_call_unsigned_converted.zome_name,
    fn_name: zome_call_unsigned_converted.fn_name,
    payload: zome_call_unsigned_converted.payload,
    cap_secret: zome_call_unsigned_converted.cap_secret,
    provenance: zome_call_unsigned_converted.provenance,
    nonce: zome_call_unsigned_converted.nonce,
    expires_at: zome_call_unsigned.expires_at,
    signature
  };

  return Ok(signed_zome_call)
}




