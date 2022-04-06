use holochain_manager::versions::HolochainVersion;
use std::{fs, path::PathBuf};

use crate::file_system::data_path_for_holochain_version;

#[tauri::command]
pub fn save_app(
  holochain_version: HolochainVersion,
  app_bundle_bytes: Vec<u8>,
) -> Result<PathBuf, String> {
  let path = data_path_for_holochain_version(holochain_version).join("app_to_install.webhapp");

  fs::write(path.clone(), app_bundle_bytes)
    .map_err(|err| format!("Failed to write app bundle: {}", err))?;

  Ok(path)
}
