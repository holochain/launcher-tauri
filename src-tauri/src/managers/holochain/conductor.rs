use std::{collections::HashMap, fs, thread, time::Duration};

use async_trait::async_trait;
use holochain_types_latest::prelude::SerializedBytes;
use serde::Serialize;

mod utils;
pub mod versions;

use crate::{
  holochain_version::{holochain_types_latest, HolochainVersion},
  managers::file_system::FileSystemManager,
  utils::create_dir_if_necessary,
};

use utils::{launch_holochain_process, launch_lair_keystore_process};

#[async_trait]
pub trait ConductorManager: Sized {
  type ConductorConfig: Serialize;
  type AdminWebsocket;

  async fn launch_holochain(log_level: log::Level, admin_port: u16) -> Result<Self, String> {
    let fs_manager = FileSystemManager::new(Self::holochain_version());

    Self::setup_filesystem(admin_port)?;

    launch_lair_keystore_process(log_level, fs_manager.keystore_data_path())?;

    thread::sleep(Duration::from_millis(1000));

    launch_holochain_process(log_level, fs_manager.conductor_config_path())?;

    let manager = Self::connect(admin_port).await?;

    let f = manager.setup_conductor();

    f.await?;

    Ok(manager)
  }

  async fn connect(admin_port: u16) -> Result<Self, String>;

  fn setup_filesystem(admin_port: u16) -> Result<(), String> {
    let fs_manager = FileSystemManager::new(Self::holochain_version());

    create_dir_if_necessary(fs_manager.holochain_config_path());
    create_dir_if_necessary(fs_manager.conductor_data_path());
    create_dir_if_necessary(fs_manager.keystore_data_path());

    let config = match Self::get_current_conductor_config() {
      Ok(config) => Self::overwrite_config(config, admin_port),
      Err(_) => Self::initial_config(admin_port),
    };

    let serde_config =
      serde_yaml::to_string(&config).expect("Could not serialize initial conductor config");

    fs::write(fs_manager.conductor_config_path(), serde_config)
      .expect("Could not write conductor config");

    Ok(())
  }

  fn holochain_version() -> HolochainVersion;

  async fn get_app_port(&self) -> Result<u16, String>;

  /** Config */

  fn initial_config(admin_port: u16) -> Self::ConductorConfig;

  fn overwrite_config(
    conductor_config: Self::ConductorConfig,
    admin_port: u16,
  ) -> Self::ConductorConfig;

  fn get_current_conductor_config() -> Result<Self::ConductorConfig, String>;

  fn get_admin_port_from_conductor_config() -> Result<Option<u16>, String>;

  /** Possible actions when we successfully connected to the conductor */

  async fn setup_conductor(&self) -> Result<(), String>;

  async fn install_app(
    &self,
    app_id: &String,
    app_bundle: holochain_types_latest::prelude::AppBundle,
    uid: Option<String>,
    membrane_proofs: HashMap<String, SerializedBytes>,
  ) -> Result<(), String>;

  async fn uninstall_app(&self, app_id: &String) -> Result<(), String>;

  async fn enable_app(&self, app_id: &String) -> Result<(), String>;

  async fn disable_app(&self, app_id: &String) -> Result<(), String>;

  async fn list_running_apps(&self) -> Result<Vec<String>, String>;
}
