use std::sync::Arc;

use holochain_websocket::{connect, WebsocketConfig};
use url2;

pub mod config;
pub mod version;
pub mod logs;

pub async fn is_holochain_already_running() -> bool {
  match config::config_admin_port() {
    Err(_) => false,
    Ok(maybe_port) => match maybe_port {
      None => false,
      Some(port) => {
        let url = url2::url2!("ws://localhost:{}", port);
        let websocket_config = WebsocketConfig::default().default_request_timeout_s(20);

        match connect(url, Arc::new(websocket_config)).await {
          Ok(_) => true,
          Err(_) => false,
        }
      }
    },
  }
}
