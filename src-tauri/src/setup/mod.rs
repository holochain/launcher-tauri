use holochain_conductor_client::AdminWebsocket;

pub mod config;
pub mod logs;

pub async fn is_holochain_already_running() -> Result<bool, String> {
  let maybe_port = config::config_admin_port()?;

  match maybe_port {
    None => Ok(false),
    Some(port) => match AdminWebsocket::connect(format!("ws://localhost:{}", port)).await {
      Ok(_) => Ok(true),
      Err(_) => Ok(false),
    },
  }
}
