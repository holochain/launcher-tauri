use holochain_types::prelude::{ZomeCallUnsigned, AgentPubKey};
use holochain_conductor_api::ZomeCall;
use lair_keystore_api::LairClient;
use std::collections::HashMap;
use holochain_launcher_utils::zome_call_signing::{ZomeCallUnsignedTauri, sign_zome_call_with_client};

use crate::error::HcLaunchError;



#[tauri::command]
pub async fn sign_zome_call(
  window: tauri::Window,
  lair_clients_state: tauri::State<'_, HashMap<String, LairClient>>,
  pubkey_map_state: tauri::State<'_, HashMap<String, AgentPubKey>>,
  zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, HcLaunchError> {

  let window_label = window.label();

  // if window_label != "admin" {
  //   () // this function is allowed to be called by any window
  // }

  let unsigned_zome_call_converted: ZomeCallUnsigned = zome_call_unsigned.into();

  // validate that the agent public key added to the ZomeCallUnsigned field is actually the
  // one associated to the UI that's making the call
  let maybe_authorized_pubkey = (*pubkey_map_state).get(window_label);

  match maybe_authorized_pubkey {
    Some(pubkey) => {
      if pubkey != &unsigned_zome_call_converted.provenance {
        return Err(HcLaunchError::SignZomeCallError(String::from("The provided public key in the provenance field is not authorized to make a zome call to the requested cell.")));
      }
    },
    None => {
      return Err(HcLaunchError::SignZomeCallError(String::from("No authorized public key found for this window.")));
    }
  }

  // get the right lair client from the hashmap
  let client = (*lair_clients_state).get(window_label)
    .expect(format!("No lair client for this window with label '{}'", window_label).as_str());

  // sign the zome call
  sign_zome_call_with_client(
    unsigned_zome_call_converted,
    client,
  )
  .await
  .map_err(|e| HcLaunchError::SignZomeCallError(e))
}




