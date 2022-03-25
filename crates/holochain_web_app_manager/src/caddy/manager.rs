use std::{fs, path::PathBuf};

use holochain_manager::versions::utils::create_dir_if_necessary;
use lair_keystore_manager::error::LaunchTauriSidecarError;

use crate::running_apps::RunningApps;

use super::utils::{build_caddyfile_contents, launch_caddy_process, reload_caddy};
pub struct CaddyManager {
  environment_path: PathBuf,
  caddy_admin_port: u16,
  conductor_admin_port: u16,
  conductor_app_interface_port: u16,
}

impl CaddyManager {
  pub fn launch(
    environment_path: PathBuf,
    conductor_admin_port: u16,
    conductor_app_interface_port: u16,
  ) -> Result<Self, LaunchTauriSidecarError> {
    let caddy_admin_port = portpicker::pick_unused_port().expect("No ports free");

    create_dir_if_necessary(&environment_path);

    launch_caddy_process(caddyfile_path(environment_path.clone()))?;

    Ok(CaddyManager {
      environment_path,
      caddy_admin_port,
      conductor_admin_port,
      conductor_app_interface_port,
    })
  }

  pub fn update_running_apps(
    &mut self,
    running_apps: &RunningApps,
  ) -> Result<(), LaunchTauriSidecarError> {
    let new_caddyfile = build_caddyfile_contents(
      self.caddy_admin_port,
      self.conductor_admin_port,
      self.conductor_app_interface_port,
      running_apps,
    );

    let caddyfile_path = caddyfile_path(self.environment_path.clone());

    fs::write(caddyfile_path.clone(), new_caddyfile).expect("Could not write Caddyfile");

    reload_caddy(caddyfile_path)?;

    Ok(())
  }
}

fn caddyfile_path(environment_path: PathBuf) -> PathBuf {
  environment_path.join("Caddyfile")
}
