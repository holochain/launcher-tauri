use std::path::PathBuf;

use lair_keystore_manager::error::LaunchChildError;
use lair_keystore_manager::utils::create_dir_if_necessary;
use tauri::api::process::CommandChild;

use crate::installed_web_app_info::InstalledWebAppInfo;

use super::utils::{launch_caddy_process, reload_caddy, write_caddyfile};

pub struct CaddyManager {
  environment_path: PathBuf,
  caddy_admin_port: u16,
  conductor_admin_port: u16,
  conductor_app_interface_port: u16,
  command_child: CommandChild,
}

impl CaddyManager {
  pub fn launch(
    environment_path: PathBuf,
    conductor_admin_port: u16,
    conductor_app_interface_port: u16,
  ) -> Result<Self, LaunchChildError> {
    let caddy_admin_port = portpicker::pick_unused_port().expect("No ports free");

    create_dir_if_necessary(&environment_path)?;
    let caddyfile = caddyfile_path(environment_path.clone());

    write_caddyfile(
      caddyfile.clone(),
      caddy_admin_port,
      conductor_admin_port,
      conductor_app_interface_port,
      &vec![],
    );
    let command_child = launch_caddy_process(caddyfile)?;

    Ok(CaddyManager {
      environment_path,
      caddy_admin_port,
      conductor_admin_port,
      conductor_app_interface_port,
      command_child,
    })
  }

  pub fn update_running_apps(
    &mut self,
    installed_apps: &Vec<InstalledWebAppInfo>,
  ) -> Result<(), LaunchChildError> {
    let caddyfile = caddyfile_path(self.environment_path.clone());

    write_caddyfile(
      caddyfile.clone(),
      self.caddy_admin_port,
      self.conductor_admin_port,
      self.conductor_app_interface_port,
      installed_apps,
    );
    reload_caddy(caddyfile)?;

    Ok(())
  }

  pub fn kill(self) -> Result<(), String> {
    self
      .command_child
      .kill()
      .map_err(|err| format!("Could not kill the caddy process: {}", err))
  }
}

fn caddyfile_path(environment_path: PathBuf) -> PathBuf {
  environment_path.join("Caddyfile")
}
