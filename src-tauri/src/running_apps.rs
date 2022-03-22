use std::collections::HashMap;
use std::path::PathBuf;

use holochain_manager::versions::HolochainVersion;

pub type RunningApps = HashMap<HolochainVersion, HashMap<String, RunningApp>>;

pub struct RunningApp {
  app_ui_port: u16,
  web_app_files_path: PathBuf,
}
