use crate::launcher::state::LauncherState;
use lair_keystore_manager::*;
use holochain_types::prelude::ZomeCallUnsigned;

use holochain_launcher_utils::zome_call_signing::ZomeCallUnsignedTauri;


#[tauri::command]
pub async fn sign_zome_call(
  state: tauri::State<'_, LauncherState>,
  // lair_keystore_manager: T<LairKeystoreManager>,
  zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, String> {

  // if window.label() != "admin" {
  //   () // this function is allowed to be called in any window
  // }

  let zome_call_unsigned_converted: ZomeCallUnsigned = zome_call_unsigned.into();

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let lair_keystore_manager = manager.get_lair_keystore_manager()?;
  let signed_zome_call = lair_keystore_manager.sign_zome_call(zome_call_unsigned_converted)
    .await
    .map_err(|_| String::from("Signing zome call failed."))?;

  return Ok(signed_zome_call)
}

