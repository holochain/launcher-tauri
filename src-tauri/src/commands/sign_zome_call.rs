use crate::launcher::state::LauncherState;
use lair_keystore_manager::*;
use holochain_types::prelude::ZomeCallUnsigned;

use holochain_launcher_utils::zome_call_signing::ZomeCallUnsignedTauri;


// // I need a mapping between window label and agent public key
// HashMap<String, AgentPubKey> where the string is HolochainVersionId.into::<String>()

#[tauri::command]
pub async fn sign_zome_call(
  window: tauri::Window,
  launcher_state: tauri::State<'_, LauncherState>,
  // state storing the information about what app id is associated with what window label
  // lair_keystore_manager: T<LairKeystoreManager>,
  zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, String> {

  // if window.label() != "admin" {
  //   () // this function is allowed to be called in any window
  // }

  // validate that the agent public key added to the ZomeCallUnsigned field is actually
  // one associated to the UI that's making the call

  // get the agent public key associated to the tauri window that makes the call



  let zome_call_unsigned_converted: ZomeCallUnsigned = zome_call_unsigned.into();

  let mut mutex = (*launcher_state).lock().await;
  let manager = mutex.get_running()?;

  let lair_keystore_manager = manager.get_lair_keystore_manager()?;
  let signed_zome_call = lair_keystore_manager.sign_zome_call(zome_call_unsigned_converted)
    .await
    .map_err(|_| String::from("Signing zome call failed."))?;

  return Ok(signed_zome_call)
}

