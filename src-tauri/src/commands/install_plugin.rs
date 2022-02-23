use reqwest::Url;

use crate::{setup::config::plugins_folder_path, state::LauncherState};
use std::fs;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::os::unix::fs::PermissionsExt;

#[tauri::command]
pub async fn execute_plugin_install(
  _state: tauri::State<'_, LauncherState>,
  url: String,
) -> Result<(), String> {
  log::info!("Installing plugin: {}", url);

  let url = Url::parse(&url).map_err(|err| {
    log::error!("Error parse url: {}", err);
    err.to_string()
  })?;

  let fname = url
    .path_segments()
    .and_then(|segments| segments.last())
    .and_then(|name| if name.is_empty() { None } else { Some(name) })
    .ok_or(String::from("Error install plugin: no name provided."))?;
  let plugins_folder = plugins_folder_path();
  log::info!("File to download: '{}'", fname);
  let fname = plugins_folder.join(fname);
  log::info!("File will be located under: '{:?}'", fname);
  let mut dest = File::create(fname.clone()).map_err(|err| {
    log::error!("Error install plugin: {}", err);
    err.to_string()
  })?;

  let response = reqwest::get(url.clone())
    .await
    .map_err(|err| err.to_string())?;
  let mut content = Cursor::new(response.bytes().await.map_err(|err| err.to_string())?);
  copy(&mut content, &mut dest).map_err(|err| err.to_string())?;

  fs::set_permissions(fname, fs::Permissions::from_mode(0o755)).map_err(|err| err.to_string())?;

  Ok(())
}
