use std::path::PathBuf;

use url2::Url2;

#[derive(Clone)]
pub struct LaunchHolochainConfig {
  pub log_level: log::Level,
  pub admin_port: u16,
  pub config_environment_path: PathBuf,
  pub environment_path: PathBuf,
  pub keystore_connection_url: Url2,
}
