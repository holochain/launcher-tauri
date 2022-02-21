use crate::{setup::config::plugins_folder_path, state::LauncherState};
use std::fs::File;
use std::io::copy;

#[tauri::command]
pub async fn execute_plugin_install(
  _state: tauri::State<'_, LauncherState>,
  url: String,
) -> Result<(), String> {
  log::info!("Installing plugin: {}", url);

  let plugins_folder = plugins_folder_path();

  let response = reqwest::get(url).await.map_err(|err| {
    log::error!("Error install plugin: {}", err);
    err.to_string()
  })?;

  let mut dest = {
    let fname = response
      .url()
      .path_segments()
      .and_then(|segments| segments.last())
      .and_then(|name| if name.is_empty() { None } else { Some(name) })
      .ok_or(String::from("Error install plugin: no name provided."))?;

    log::info!("File to download: '{}'", fname);
    let fname = plugins_folder.join(fname);
    log::info!("File will be located under: '{:?}'", fname);
    File::create(fname).map_err(|err| {
      log::error!("Error install plugin: {}", err);
      err.to_string()
    })?
  };
  let content = response.text().await.map_err(|err| {
    log::error!("Error install plugin: {}", err);
    err.to_string()
  })?;
  copy(&mut content.as_bytes(), &mut dest).map_err(|err| {
    log::error!("Error install plugin: {}", err);
    err.to_string()
  })?;

  Ok(())
}
