use std::sync::Arc;

use holochain_websocket::{connect, WebsocketConfig};
use url::Url;

pub mod config;
pub mod logs;

pub async fn is_holochain_already_running() -> bool {
  match config::config_admin_port() {
    Err(_) => false,
    Ok(maybe_port) => match maybe_port {
      None => false,
      Some(port) => {
        let url = Url::parse(&format!("ws://localhost:{}", port)).unwrap();
        let websocket_config = WebsocketConfig::default().default_request_timeout_s(20);

        match connect(url.clone().into(), Arc::new(websocket_config)).await {
          Ok(_) => true,
          Err(_) => false,
        }
      }
    },
  }
}
