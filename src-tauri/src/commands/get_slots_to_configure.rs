use holochain_types::prelude::{AppSlotManifest, CellProvisioning};
use holochain_types::web_app::WebAppBundle;
use std::fs;

#[tauri::command]
pub async fn get_slots_to_configure(
  web_app_bundle_path: String,
) -> Result<Vec<AppSlotManifest>, String> {
  log::info!("Installing: web_app_bundle = {}", web_app_bundle_path);

  let web_app_bundle = WebAppBundle::decode(
    &fs::read(&web_app_bundle_path).or(Err("Failed to read Web hApp bundle file"))?,
  )
  .or(Err("Malformed Web hApp bundle file"))?;

  let app_bundle = web_app_bundle
    .happ_bundle()
    .await
    .or(Err("Failed to resolve hApp bundle"))?;

  let app_slots = app_bundle.manifest().app_slots();

  let slots_to_create: Vec<AppSlotManifest> = app_slots
    .into_iter()
    .filter(|slot| match slot.provisioning {
      Some(
        CellProvisioning::Create { .. }
        | CellProvisioning::CreateClone { .. }
        | CellProvisioning::CreateIfNotExists { .. },
      ) => true,
      _ => false,
    })
    .collect();

  Ok(slots_to_create)
}
