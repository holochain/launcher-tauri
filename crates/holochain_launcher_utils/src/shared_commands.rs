

#[tauri::command]
pub fn open_url_cmd(window: tauri::Window, url: String) -> Result<(), String> {
  if window.label() != "admin" {
    // sanitize url if the open request does not come from the admin window
    if url.starts_with("http://") || url.starts_with("https://") {
      return open_url(url);
    } else {
      return Err(String::from("Unauthorized: Accessing resources other than http(s) via anchor tags not allowed in Holochain Launcher windows."))
    }
  }

  open_url(url)
}

pub fn open_url(url: String) -> Result<(), String>  {
  tauri::async_runtime::spawn(async move {
    open::that(url.clone().as_str()).map_err(|err| format!("Could not open url: {}", err))
  });

  Ok(())
}
