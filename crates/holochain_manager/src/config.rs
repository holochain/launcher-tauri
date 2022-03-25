use std::path::PathBuf;

#[derive(Clone)]
pub struct LaunchHolochainConfig {
  pub log_level: log::Level,
  pub admin_port: u16,
  pub conductor_config_path: PathBuf,
  pub environment_path: PathBuf,
  pub keystore_path: PathBuf,
}
