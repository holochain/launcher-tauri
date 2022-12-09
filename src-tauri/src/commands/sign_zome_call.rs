use crate::launcher::state::LauncherState;
use hdk::prelude::{CellId, ZomeName, FunctionName, CapSecret, ExternIO, Timestamp, AgentPubKey};
use lair_keystore_manager::*;
use holochain_types::prelude::ZomeCallUnsigned;
use serde::Deserialize;
use std::convert::TryInto;

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
  state: tauri::State<'_, LauncherState>,
  // lair_keystore_manager: T<LairKeystoreManager>,
  zome_call_unsigned: ZomeCallUnsignedTauri,
) -> Result<ZomeCall, String> {

  // if window.label() != "admin" {
  //   () // this function is allowed to be called in any window
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

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let lair_keystore_manager = manager.get_lair_keystore_manager()?;
  let signed_zome_call = lair_keystore_manager.sign_zome_call(zome_call_unsigned_converted)
    .await
    .map_err(|_| String::from("Signing zome call failed."))?;

  return Ok(signed_zome_call)
}

