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

const LAUNCHER_ENV_URL: &str = ".launcher-env.json";

pub struct UiManager {
  holochain_version: HolochainVersion,
  admin_port: u16,
  app_port: u16,
}

impl UiManager {
  pub fn launch(
    holochain_version: HolochainVersion,
    admin_port: u16,
    app_port: u16,
  ) -> Result<Self, String> {
    let fs_manager = FileSystemManager::new(holochain_version);

    create_dir_if_necessary(fs_manager.uis_data_path());

    Ok(UiManager {
      holochain_version,
      admin_port,
      app_port,
    })
  }

  pub fn open_app(&self, app_id: &String, app_handle: &AppHandle<Wry>) -> Result<(), String> {
    let port_mapping = PortMapping::read_port_mapping()?;

    let port = port_mapping
      .get_ui_port_for_app(&self.holochain_version, &app_id)
      .ok_or(String::from("This app doesn't have a UI installed."))?;

    WindowBuilder::new(
      app_handle,
      app_id.clone(),
      WindowUrl::External(Url::parse(format!("http://localhost:{}", port).as_str()).unwrap()),
    )
    .inner_size(1000.0, 700.0)
    .title(app_id)
    .build()
    .map_err(|err| format!("Error opening app: {:?}", err))?;

    Ok(())
  }

  pub async fn install_app_ui(
    &self,
    app_id: &String,
    web_ui_zip_bytes: ResourceBytes,
  ) -> Result<(), String> {
    let fs_manager = FileSystemManager::new(self.holochain_version);

    let ui_folder_path = fs_manager.app_ui_path(app_id);
    let ui_zip_path = fs_manager.uis_data_path().join(format!("{}.zip", app_id));

    fs::write(ui_zip_path.clone(), web_ui_zip_bytes).or(Err("Failed to write Web UI Zip file"))?;

    let file = File::open(ui_zip_path).or(Err("Failed to read Web UI Zip file"))?;
    unzip_file(file, ui_folder_path)?;

    let mut port_mapping = PortMapping::read_port_mapping()?;

    port_mapping.set_available_ui_port_for_app(&self.holochain_version, app_id)?;

    Ok(())
  }

  pub async fn uninstall_app_ui(&self, app_id: &String) -> Result<(), String> {
    let fs_manager = FileSystemManager::new(self.holochain_version);

    let ui_folder_path = fs_manager.app_ui_path(app_id);

    if Path::new(&ui_folder_path).exists() {
      fs::remove_dir_all(ui_folder_path).or(Err("Failed to remove UI folder"))?;
    }

    let mut port_mapping = PortMapping::read_port_mapping()?;
    port_mapping.remove_app_from_mapping(self.holochain_version, app_id.clone())?;

    Ok(())
  }

  pub fn caddy_config_for_apps(&self, running_apps: &Vec<String>) -> Result<String, String> {
    let port_mapping = PortMapping::read_port_mapping()?;

    let config_vec: Vec<String> = running_apps
      .into_iter()
      .filter_map(|app_id| {
        port_mapping
          .get_ui_port_for_app(&self.holochain_version, &app_id)
          .map(|ui_port| self.caddyfile_config_for_app(ui_port, app_id))
      })
      .collect();

    let empty_line = r#"
"#;

    Ok(config_vec.join(empty_line))
  }

  fn caddyfile_config_for_app(&self, ui_port: u16, app_id: &String) -> String {
    format!(
      r#":{} {{
        handle_path /{} {{
                respond 200 {{
                        body `{{
                                "APP_INTERFACE_PORT": {},
                                "ADMIN_INTERFACE_PORT": {},
                                "INSTALLED_APP_ID": "{}"
                        }}`
                        close
                }}
        }}

        header Cache-Control no-cache, no-store

        handle {{
                root * "{}"
                try_files {{path}} {{file}} /index.html
                file_server
        }}
}}
"#,
      ui_port,
      LAUNCHER_ENV_URL,
      self.app_port,
      self.admin_port,
      app_id.clone(),
      FileSystemManager::new(self.holochain_version)
        .app_ui_path(&app_id)
        .into_os_string()
        .to_str()
        .unwrap(),
    )
  }
}
