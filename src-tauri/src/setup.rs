use crate::config::{admin_url, DEFAULT_APP_PORT};
use holochain_conductor_client::AdminWebsocket;

pub async fn setup_conductor() -> Result<(), String> {
  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  log::info!("Connected to admin conductor");

  let app_interfaces = ws
    .list_app_interfaces()
    .await
    .or(Err(String::from("Could not list app interfaces")))?;

  if !app_interfaces.contains(&DEFAULT_APP_PORT) {
    ws.attach_app_interface(DEFAULT_APP_PORT)
      .await
      .or(Err(String::from("Could not attach app interface")))?;
    log::info!("Attached app interface to {}", DEFAULT_APP_PORT);
  }

  Ok(())
}
