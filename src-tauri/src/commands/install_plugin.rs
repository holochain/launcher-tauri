use super::config::plugins_folder_path;

#[tauri::command]
pub async fn execute_plugin_install(
  state: tauri::State<'_, LauncherState>,
  url: String,
) -> Result<(), String> {
  log::info!("Installing plugin: {}", url);

  let plugins_folder = plugins_folder_path();

  let response = reqwest::get(url).await?;

  let mut dest = {
    let fname = response
      .url()
      .path_segments()
      .and_then(|segments| segments.last())
      .and_then(|name| {
        if name.is_empty() {
          Err("Error: no file name".to_string())
        } else {
          Ok(name)
        }
      })?;

    log::info!("File to download: '{}'", fname);
    let fname = plugins_folder.join(fname);
    log::info!("File will be located under: '{:?}'", fname);
    File::create(fname)?
  };
  let content = response.text().await?;
  copy(&mut content.as_bytes(), &mut dest)?;

  Ok(())
}
