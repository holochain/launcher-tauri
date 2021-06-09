use crate::{config::{admin_url, DEFAULT_APP_PORT}, uis::activate::activate_uis_for_active_apps};
use holochain_conductor_api_rust::AdminWebsocket;

pub async fn setup_conductor() -> Result<(), String> {
  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  let app_interfaces = ws.list_app_interfaces().await.or(Err(String::from("Could not list app interfaces")))?;

  if !app_interfaces.contains(&DEFAULT_APP_PORT) {
    ws.attach_app_interface(DEFAULT_APP_PORT).await.or(Err(String::from("Could not attach app interface")))?;
  }

  activate_uis_for_active_apps(&mut ws).await
}
