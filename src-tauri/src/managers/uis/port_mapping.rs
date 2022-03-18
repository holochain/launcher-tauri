use crate::{holochain_version::HolochainVersion, managers::file_system::FileSystemManager};
use serde::{Deserialize, Serialize};
use std::{
  collections::{BTreeMap, HashMap},
  fs,
  io::ErrorKind,
  path::PathBuf,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PortMapping(HashMap<HolochainVersion, BTreeMap<String, u16>>);

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

  pub fn read_port_mapping() -> Result<PortMapping, String> {
    match fs::read_to_string(Self::path()) {
      Err(error) => match error.kind() {
        ErrorKind::NotFound => Ok(PortMapping(HashMap::new())),
        _ => Err(format!("Error reading the UIs port mapping {:?}", error)),
      },
      Ok(contents) => {
        let mapping: PortMapping =
          serde_yaml::from_str(contents.as_str()).or(Err("Malformed port mapping file"))?;
        Ok(mapping)
      }
    }
  }

  pub fn set_available_ui_port_for_app(
    &mut self,
    holochain_version: HolochainVersion,
    app_id: String,
  ) -> Result<u16, String> {
    let port = self.get_next_available_port();

    let version_map = self
      .0
      .entry(holochain_version)
      .or_insert_with(|| BTreeMap::new());

    version_map.insert(app_id, port);

    self.write_port_mapping()?;

    Ok(port)
  }

  pub fn remove_app_from_mapping(
    &mut self,
    holochain_version: HolochainVersion,
    app_id: String,
  ) -> Result<(), String> {
    if let Some(version_map) = self.0.get(&holochain_version) {
      version_map.remove(&app_id);
    }

    self.write_port_mapping()?;

    Ok(())
  }

  fn get_next_available_port(&self) -> u16 {
    portpicker::pick_unused_port().expect("No ports free")
  }

  fn write_port_mapping(&self) -> Result<(), String> {
    let s = serde_yaml::to_string(&self).or(Err("Could not format into yaml"))?;

    fs::write(Self::path(), s).or(Err("Could not write port mapping to file disk".into()))
  }
}
