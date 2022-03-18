pub struct ManagerConfig {
  pub log_level: log::Level,
}

impl Default for ManagerConfig {
  fn default() -> ManagerConfig {
    ManagerConfig {
      log_level: log::Level::Warn,
    }
  }
}
