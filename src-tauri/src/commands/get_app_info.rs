use holochain_manager::versions::holochain_types_latest::{
  prelude::{AppBundle, AppRoleManifest, CellProvisioning},
  web_app::WebAppBundle,
};
use std::fs;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebAppInfo {
  pub app_name: String,
  pub roles_to_create: Vec<AppRoleManifest>,
}

/// Read roles of app to use in the InstallAppDialog to ask for membrane proofs
#[tauri::command]
pub async fn get_app_info(window: tauri::Window, app_bundle_path: String) -> Result<WebAppInfo, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (F)"))
  }

  let app_bundle = read_app_bundle(app_bundle_path).await?;

  let app_slots = app_bundle.manifest().app_roles();

  let roles_to_create: Vec<AppRoleManifest> = app_slots
    .into_iter()
    .filter(|slot| match slot.provisioning {
      Some(
        CellProvisioning::Create { .. }
        | CellProvisioning::CloneOnly
      ) => true,
      _ => false,
    })
    .collect();

  Ok(WebAppInfo {
    app_name: app_bundle.manifest().app_name().to_string(),
    roles_to_create,
  })
}

async fn read_app_bundle(path: String) -> Result<AppBundle, String> {
  let bytes = fs::read(&path).or(Err("Failed to read Web hApp bundle file"))?;
  match WebAppBundle::decode(&bytes) {
    Ok(web_app_bundle) => {
      let app_bundle = web_app_bundle
        .happ_bundle()
        .await
        .or(Err("Failed to resolve hApp bundle"))?;

      Ok(app_bundle)
    }
    Err(_) => {
      let bundle = AppBundle::decode(&bytes).or(Err("Failed to resolve hApp bundle"))?;
      Ok(bundle)
    }
  }
}
