use std::collections::HashMap;
use futures::lock::Mutex;
use std::sync::Arc;
use crate::launcher::state::LauncherState;
use hdk::prelude::AgentPubKey;
use lair_keystore_manager::*;
use holochain_types::prelude::ZomeCallUnsigned;

use holochain_launcher_utils::zome_call_signing::ZomeCallUnsignedTauri;


#[tauri::command]
pub async fn sign_zome_call(
  window: tauri::Window,
  launcher_state: tauri::State<'_, LauncherState>,
  pubkey_map_state: tauri::State<'_, Arc<Mutex<HashMap<String, AgentPubKey>>>>,
  zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, String> {

  // if window.label() != "admin" {
  //   () // this function is allowed to be called in any window
  // }

  let window_label = window.label();

  {
    // validate that the agent public key added to the ZomeCallUnsigned field is actually
    // one associated to the UI that's making the call
    // NOTE: This part is in its own scope such that the pubkey_map_state lock gets released
    // before the launcher_state lock is acquired. Otherwise, it can lead to a deadlock when
    // calling install_app() around the same time (https://github.com/holochain/launcher/issues/182)
    let pubkey_map = &*pubkey_map_state.lock().await;
    let maybe_authorized_pubkey = pubkey_map.get(window_label);

    if window_label != "admin" {
      match maybe_authorized_pubkey {
        Some(pubkey) => {
          if pubkey != &zome_call_unsigned.provenance {
            log::warn!("[ZOME CALL SIGNING] WARGNING: A tauri window attempted to make a zome call with a public key that it is not authorized to make zome calls with. Window label: '{}'", window_label);
            return Err(String::from("The provided public key in the provenance field is not authorized to make a zome call to the requested cell."));
          }
        },
        None => {
          log::warn!("[ZOME CALL SIGNING] WARGNING: A tauri window attempted to make a zome call with a public key that it is not authorized to make zome calls with. Window label: '{}'", window_label);
          return Err(String::from("No authorized public key found for this window."));
        }
      }
    }
  }

  let zome_call_unsigned_converted: ZomeCallUnsigned = zome_call_unsigned.into();

  let mut mutex = (*launcher_state).lock().await;
  let manager = mutex.get_running()?;

  let lair_keystore_manager = manager.get_lair_keystore_manager()?;
  let signed_zome_call = lair_keystore_manager.sign_zome_call(zome_call_unsigned_converted)
    .await
    .map_err(|e| format!("Signing zome call failed: {}", e))?;

  Ok(signed_zome_call)
}

