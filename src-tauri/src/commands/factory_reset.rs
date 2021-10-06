use std::{fs, io, path::PathBuf};

use tauri::api::process::kill_children;
use portpicker::pick_unused_port;

use crate::{launch::launch_children_processes, setup::config::{holochain_config_path, holochain_data_path, keystore_data_path}, state::{LauncherState, RunningPorts}};

#[tauri::command]
pub async fn execute_factory_reset(state: tauri::State<'_, LauncherState>) -> Result<(), String> {
  // Holochain may be down; if it is, pick a new port for the relaunch
  let ports = match state.get_running_ports() {
    Ok(ports) => ports,
    Err(_) => RunningPorts {
      admin_interface_port: pick_unused_port().expect("No ports free"),
      caddy_admin_port: pick_unused_port().expect("No ports free"),
    },
  };

  factory_reset(ports).await
}

async fn factory_reset(ports: RunningPorts) -> Result<(), String> {
  log::warn!("A factory reset has been requested, initiating...");

  // Kill all the children processes to avoid messing up with the filesystem
  kill_children();
  log::info!("Stopped children processes");

  remove_dir_if_exists(holochain_config_path()).map_err(|err| {
    log::error!("Could not remove holochain config path: {}", err);
    String::from("Could not remove holochain config path")
  })?;
  remove_dir_if_exists(keystore_data_path()).map_err(|err| {
    log::error!("Could not remove lair path: {}", err);
    String::from("Could not remove lair path")
  })?;
  remove_dir_if_exists(holochain_data_path())
    .or(Err(String::from("Could not remove holochain data path")))?;

  launch_children_processes(ports).await.map_err(|err| {
    log::error!("Failed to restart Holochain: {}", err);
    String::from("Failed to restart Holochain")
  })?;

  log::info!("Started children processes again, factory reset completed");

  Ok(())
}

fn remove_dir_if_exists(path: PathBuf) -> io::Result<()> {
  if let Ok(_) = fs::read_dir(path.clone()) {
    fs::remove_dir_all(path)?;
  }
  Ok(())
}
