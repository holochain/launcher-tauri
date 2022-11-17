use holochain_types::prelude::ZomeCallUnsigned;
use holochain_zome_types::Signature;
use holochain_conductor_api::ZomeCall;
use lair_keystore_api::LairClient;
use std::collections::HashMap;

use crate::error::HcLaunchTauriError;

#[tauri::command]
pub async fn sign_zome_call(
  window: tauri::Window,
  state: tauri::State<'_, HashMap<String, LairClient>>,
  zome_call_unsigned: ZomeCallUnsigned,
) -> Result<ZomeCall, HcLaunchTauriError> {

  let window_label = window.label().to_string();

  // if window_label != "admin" {
  //   () // this function is allowed to be called by any window
  // }

  // get the right lair client from the hashmap
  let client = (*state).get(&window_label)
    .expect(format!("No lair client for this window with label '{}'", window_label).as_str());

  // sign the zome call
  let pub_key = zome_call_unsigned.provenance.clone();
  let mut pub_key_2 = [0; 32];
  pub_key_2.copy_from_slice(pub_key.get_raw_32());

  let data_to_sign = zome_call_unsigned.data_to_sign()
    .map_err(|e| HcLaunchTauriError::DataToSignError(e.into()))?;

  let sig = client.sign_by_pub_key(
    pub_key_2.into(),
     None,
    data_to_sign)
    .await
    .map_err(|e| HcLaunchTauriError::SignZomeCallError(e.str_kind().to_string()))?;

  let signature = Signature(*sig.0);

  let signed_zome_call = ZomeCall {
    cell_id: zome_call_unsigned.cell_id,
    zome_name: zome_call_unsigned.zome_name,
    fn_name: zome_call_unsigned.fn_name,
    payload: zome_call_unsigned.payload,
    cap_secret: zome_call_unsigned.cap_secret,
    provenance: zome_call_unsigned.provenance,
    nonce: zome_call_unsigned.nonce,
    expires_at: zome_call_unsigned.expires_at,
    signature
  };

  return Ok(signed_zome_call)
}




