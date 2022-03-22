use crate::{holochain_version::HolochainVersion, managers::file_system::FileSystemManager};
use serde::{Deserialize, Serialize};
use std::{
  collections::{BTreeMap, HashMap},
  fs,
  io::ErrorKind,
  path::PathBuf,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortMapping {
  port_mapping_path: PathBuf,
  app_ports: HashMap<String, u16>,
}

impl PortMapping {
  fn path() -> PathBuf {
    FileSystemManager::port_mapping_path()
  }

  pub fn get_ui_port_for_app(
    &self,
    holochain_version: &HolochainVersion,
    app_id: &String,
  ) -> Option<u16> {
    if let Some(versions) = self.0.get(holochain_version) {
      return versions.get(app_id).map(|p| p.clone());
    }
    None
  }

  pub fn read(port_mapping_path: PathBuf) -> Result<PortMapping, String> {
    match fs::read_to_string(root_path) {
      Err(error) => match error.kind() {
        ErrorKind::NotFound => Ok(PortMapping(HashMap::new())),
        _ => Err(format!("Error reading the UIs port mapping {:?}", error)),
      },
      Ok(contents) => {
        let app_ports: HashMap<String, u16> =
          serde_yaml::from_str(contents.as_str()).or(Err("Malformed port mapping file"))?;
        Ok(PortMapping {
          port_mapping_path,
          app_ports,
        })
      }
    }
  }

  pub fn set_available_ui_port_for_app(&mut self, app_id: &String) -> Result<u16, String> {
    let app_port = portpicker::pick_unused_port().expect("No ports free");

    self.app_ports.insert(app_id.clone(), app_port);

    self.write_port_mapping()?;

    Ok(port)
  }

  pub fn remove_app_from_mapping(&mut self, app_id: String) -> Result<(), String> {
    self.app_ports.remove(&app_id);

    self.write_port_mapping()?;

    Ok(())
  }

  fn write_port_mapping(&self) -> Result<(), String> {
    let s = serde_yaml::to_string(&self.app_ports).or(Err("Could not format into yaml"))?;

    fs::write(Self::path(), s).or(Err("Could not write port mapping to file disk".into()))
  }
}
