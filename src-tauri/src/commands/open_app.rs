use crate::launcher::{state::LauncherState, manager::HolochainId};

#[tauri::command]
pub async fn open_app_ui(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  holochain_id: HolochainId,
  app_id: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (J)"))
  }
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .open_app(holochain_id, &app_id)
    .map_err(|err| format!("Error opening app: {}", err))?;

  log::info!("Opening app {}", app_id.clone(),);

  Ok(())
}

#[tauri::command]
pub fn report_issue_cmd(window: tauri::Window) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command."))
  }
  Ok(report_issue())
}

pub fn report_issue() -> () {
  open_url("https://github.com/holochain/launcher/issues/new?assignees=&labels=bug&template=bug_report.md&title=".into()).unwrap()
}

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
