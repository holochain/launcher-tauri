use std::path::PathBuf;

use tauri::api::process::Command;
use url2::Url2;

pub struct LaunchHolochainConfig {
  pub log_level: log::Level,
  pub admin_port: u16,
  pub command: Command,
  pub config_environment_path: PathBuf,
  pub environment_path: PathBuf,
  pub keystore_connection_url: Url2,
}
