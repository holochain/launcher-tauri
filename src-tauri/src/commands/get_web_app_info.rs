use holochain_types::prelude::{AppRoleManifest, CellProvisioning};
use holochain_types::web_app::WebAppBundle;
use std::fs;

#[derive(serde::Serialize)]
pub struct WebAppInfo {
  app_name: String,
  roles_to_create: Vec<AppRoleManifest>,
}

#[tauri::command]
pub async fn get_web_app_info(web_app_bundle_path: String) -> Result<WebAppInfo, String> {
  log::info!("Installing: web_app_bundle = {}", web_app_bundle_path);

  let web_app_bundle = WebAppBundle::decode(
    &fs::read(&web_app_bundle_path).or(Err("Failed to read Web hApp bundle file"))?,
  )
  .or(Err("Malformed Web hApp bundle file"))?;

  let app_bundle = web_app_bundle
    .happ_bundle()
    .await
    .or(Err("Failed to resolve hApp bundle"))?;

  let app_slots = app_bundle.manifest().app_roles();

  let roles_to_create: Vec<AppRoleManifest> = app_slots
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

  Ok(WebAppInfo {
    app_name: app_bundle.manifest().app_name().to_string(),
    roles_to_create,
  })
}
