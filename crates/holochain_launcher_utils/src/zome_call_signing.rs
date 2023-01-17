
use holochain_client::AgentPubKey;
use holochain_conductor_api::ZomeCall;
use holochain_types::prelude::ZomeCallUnsigned;
use holochain_zome_types::{Signature, CellId, ZomeName, FunctionName, CapSecret, ExternIO, Timestamp};
use lair_keystore_api::LairClient;

use serde::Deserialize;


/// Signs an unsigned zome call with the given LairClient
pub async fn sign_zome_call_with_client(
  zome_call_unsigned: ZomeCallUnsigned,
  client: &LairClient,
) -> Result<ZomeCall, String> {

  // sign the zome call
  let pub_key = zome_call_unsigned.provenance.clone();
  let mut pub_key_2 = [0; 32];
  pub_key_2.copy_from_slice(pub_key.get_raw_32());

  let data_to_sign = zome_call_unsigned.data_to_sign()
    .map_err(|e| format!("Failed to get data to sign from unsigned zome call: {}", e))?;

  let sig = client.sign_by_pub_key(
    pub_key_2.into(),
     None,
    data_to_sign)
    .await
    .map_err(|e| format!("Failed to sign zome call by pubkey: {}", e.str_kind()))?;

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






/// The version of an unsigned zome call that's compatible with the serialization
/// behavior of tauri's IPC channel (serde serialization)
/// nonce is a byte array [u8, 32] because holochain's nonce type seems to
/// have "non-serde" deserialization behavior.
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


impl Into<ZomeCallUnsigned> for ZomeCallUnsignedTauri {
  fn into(self) -> ZomeCallUnsigned {
    ZomeCallUnsigned {
      provenance: self.provenance,
      cell_id: self.cell_id,
      zome_name: self.zome_name,
      fn_name: self.fn_name,
      cap_secret: self.cap_secret,
      payload: self.payload,
      nonce: self.nonce.into(),
      expires_at: self.expires_at,
    }
  }
}

impl Clone for ZomeCallUnsignedTauri {
  fn clone(&self) -> Self {
    ZomeCallUnsignedTauri {
      provenance: self.provenance.clone(),
      cell_id: self.cell_id.clone(),
      zome_name: self.zome_name.clone(),
      fn_name: self.fn_name.clone(),
      cap_secret: self.cap_secret.clone(),
      payload: self.payload.clone(),
      nonce: self.nonce.clone(),
      expires_at: self.expires_at.clone(),
    }
  }
}