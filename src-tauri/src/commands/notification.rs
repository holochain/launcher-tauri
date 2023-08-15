use tauri::{api::notification::Notification, AppHandle};


#[tauri::command]
pub async fn notify(
  _window: tauri::Window,
  app_handle: AppHandle,
  notification: String,
) -> tauri::Result<()> {

  let mut os_notification =  Notification::new(&app_handle.config().tauri.bundle.identifier)
    .body(notification);
  os_notification.show().unwrap();
  Ok(())
}