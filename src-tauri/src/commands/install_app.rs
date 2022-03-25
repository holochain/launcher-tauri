use holochain_manager::versions::{
  holochain_types_latest::{
    prelude::{SerializedBytes, UnsafeBytes},
    web_app::WebAppBundle,
  },
  HolochainVersion,
};
use std::{collections::HashMap, fs};

use crate::launcher::state::LauncherState;

#[tauri::command]
pub async fn install_app(
  state: tauri::State<'_, LauncherState>,
  holochain_version: HolochainVersion,
  app_id: String,
  web_app_bundle_path: String,
  uid: Option<String>,
  membrane_proofs: HashMap<String, Vec<u8>>,
) -> Result<(), String> {
  log::info!("Installing: web_app_bundle = {}", web_app_bundle_path);

  let web_app_bundle = WebAppBundle::decode(
    &fs::read(&web_app_bundle_path).or(Err("Failed to read Web hApp bundle file"))?,
  )
  .or(Err("Malformed Web hApp bundle file"))?;

  let mut converted_membrane_proofs: HashMap<String, SerializedBytes> = HashMap::new();
  for (dna_slot, proof) in membrane_proofs.iter() {
    converted_membrane_proofs.insert(
      dna_slot.clone(),
      SerializedBytes::from(UnsafeBytes::from(proof.clone())),
    );
  }

  let mut manager = state.get_launcher_manager()?.lock().await;

  manager
    .get_web_happ_manager(holochain_version)?
    .install_web_app(
      app_id.clone(),
      web_app_bundle,
      uid,
      converted_membrane_proofs,
    )
    .await?;

  log::info!("Installed hApp {}", app_id);

  Ok(())
}
