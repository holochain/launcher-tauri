use serde_yaml::{Mapping, Value};
use std::path::PathBuf;

use enum_dispatch::enum_dispatch;
use lair_keystore_manager::versions::LairKeystoreVersion;
use url2::Url2;

use super::HdkVersion;

#[enum_dispatch]
pub trait VersionManager {
  fn hdk_version(&self) -> HdkVersion;

  fn lair_keystore_version(&self) -> LairKeystoreVersion {
    // For now all holochain versions run the same lair keystore version
    LairKeystoreVersion::V0_2_0
  }

  fn initial_config(
    &self,
    admin_port: u16,
    conductor_environment_path: PathBuf,
    keystore_connection_url: Url2,
  ) -> String;

  fn overwrite_config(
    &self,
    conductor_config: String,
    admin_port: u16,
    keystore_connection_url: Url2,
  ) -> String {
    let mut config = serde_yaml::from_str::<serde_yaml::Mapping>(conductor_config.as_str())
      .expect("Couldn't convert string to conductor config");

    let mut websocket_interface = Mapping::new();
    websocket_interface.insert(
      Value::String(String::from("type")),
      Value::String(String::from("websocket")),
    );
    websocket_interface.insert(
      Value::String(String::from("port")),
      Value::Number(admin_port.into()),
    );

    let mut admin_interface = Mapping::new();
    admin_interface.insert(
      Value::String(String::from("driver")),
      Value::Mapping(websocket_interface),
    );

    config.insert(
      Value::String(String::from("admin_interfaces")),
      Value::Sequence(vec![Value::Mapping(admin_interface)]),
    );

    let mut keystore_mapping = Mapping::new();

    keystore_mapping.insert(
      Value::String(String::from("type")),
      Value::String(String::from("lair_server")),
    );
    keystore_mapping.insert(
      Value::String(String::from("connection_url")),
      Value::String(format!("{}", keystore_connection_url)),
    );

    config.insert(
      Value::String(String::from("keystore")),
      Value::Mapping(keystore_mapping),
    );

    serde_yaml::to_string(&config).expect("Could not convert conductor config to string")
  }
}
