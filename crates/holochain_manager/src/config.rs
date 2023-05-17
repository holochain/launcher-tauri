use std::path::PathBuf;

use tauri::api::process::Command;
use url2::Url2;

pub struct LaunchHolochainConfig {
  pub log_level: log::Level,
  pub admin_port: u16,
  pub command: Command,
  pub conductor_config_dir: PathBuf,
  pub environment_path: PathBuf,
  pub keystore_connection_url: Url2,
  pub bootstrap_server_url: Option<String>,
  pub signaling_server_url: Option<String>,
}
