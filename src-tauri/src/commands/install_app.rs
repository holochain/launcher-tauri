use holochain_manager::versions::{
  holochain_types_latest::{
    prelude::{AppBundle, SerializedBytes, UnsafeBytes},
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
  app_bundle_path: String,
  uid: Option<String>,
  membrane_proofs: HashMap<String, Vec<u8>>,
) -> Result<(), String> {
  log::info!("Installing: web_app_bundle = {}", app_bundle_path);

  let mut converted_membrane_proofs: HashMap<String, SerializedBytes> = HashMap::new();
  for (dna_slot, proof) in membrane_proofs.iter() {
    converted_membrane_proofs.insert(
      dna_slot.clone(),
      SerializedBytes::from(UnsafeBytes::from(proof.clone())),
    );
  }

  let mut manager = state.get_launcher_manager()?.lock().await;

  let bytes = fs::read(&app_bundle_path).or(Err("Failed to read Web hApp bundle file"))?;

  match WebAppBundle::decode(&bytes) {
    Ok(web_app_bundle) => {
      manager
        .get_web_happ_manager(holochain_version)?
        .install_web_app(
          app_id.clone(),
          web_app_bundle,
          uid,
          converted_membrane_proofs,
        )
        .await?;
    }
    Err(_) => {
      let app_bundle = AppBundle::decode(&bytes).or(Err("Failed to read Web hApp bundle file"))?;
      manager
        .get_web_happ_manager(holochain_version)?
        .install_app(app_id.clone(), app_bundle, uid, converted_membrane_proofs)
        .await?;
    }
  }

  log::info!("Installed hApp {}", app_id);

  Ok(())
}
