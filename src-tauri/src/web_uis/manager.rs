use std::{
  fs::{self, File},
  path::Path,
};

use mr_bundle::ResourceBytes;
use tauri::{window::WindowBuilder, AppHandle, WindowUrl, Wry};
use url::Url;

use crate::{holochain_version::HolochainVersion, utils::create_dir_if_necessary};

use self::{port_mapping::PortMapping, utils::unzip_file};

use super::file_system::FileSystemManager;

pub mod port_mapping;
pub mod utils;


pub struct CaddyWebUiManager {
  caddy_admin_port: u16,
}

impl CaddyWebUiManager {
  pub fn launch(
    holochain_version: HolochainVersion,
    admin_port: u16,
    app_port: u16,
  ) -> Result<Self, String> {
    let fs_manager = FileSystemManager::new(holochain_version);

    create_dir_if_necessary(fs_manager.uis_data_path());

    Ok(CaddyWebUiManager {
      holochain_version,
      admin_port,
      app_port,
    })
  }
  

}
