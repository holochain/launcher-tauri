use serde_yaml::{Mapping, Value};
use std::path::PathBuf;

use enum_dispatch::enum_dispatch;
use lair_keystore_manager::versions::LairKeystoreVersion;
use url2::Url2;

use super::{HdkVersion, HdiVersion};

#[enum_dispatch]
pub trait VersionManager {
  fn hdk_version(&self) -> HdkVersion;

  fn hdi_version(&self) -> HdiVersion;

  fn lair_keystore_version(&self) -> LairKeystoreVersion {
    // For now all holochain versions run the same lair keystore version
    LairKeystoreVersion::V0_3
  }

  fn initial_config(
    &self,
    admin_port: u16,
    conductor_environment_path: PathBuf,
    keystore_connection_url: Url2,
    bootstrap_server_url: Option<String>,
    signaling_server_url: Option<String>,
  ) -> String;

  fn overwrite_config(
    &self,
    conductor_config: String,
    admin_port: u16,
    keystore_connection_url: Url2,
    bootstrap_server_url: Option<String>,
    signaling_server_url: Option<String>,
  ) -> Result<String, String> {
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

    // set signal_url and bootstrap_service
    let maybe_network_mapping = config.get_mut(&Value::String(String::from("network")));

    let network_mapping = match signaling_server_url {
      Some(url) => {
        let mut web_rtc_config = Mapping::new();
        web_rtc_config.insert(Value::String(String::from("type")), Value::String(String::from("webrtc")));
        web_rtc_config.insert(Value::String(String::from("signal_url")), Value::String(url));

        // ATTENTION: We are assuming that there is only one transport pool item (webrtc) and we overwrite any existing
        // transport pool items. If that assumption were wrong, we would need to check for others and selectively overwrite
        // only the one of type webrtc
        let mut transport_pool = Vec::new();
        transport_pool.push(Value::Mapping(web_rtc_config));

        let network_mapping = match maybe_network_mapping {
          Some(value) => {
            match value {
              Value::Mapping(mapping) => {
                mapping.insert(Value::String(String::from("transport_pool")), Value::Sequence(transport_pool));
                mapping.clone()
              },
              _ => {
                return Err(String::from("Failed to overwrite config: 'network' value of conductor-config.yaml is of unexpected type: {:?}"));
              }
            }
          },
          None => {
            let mut mapping = Mapping::new();
            mapping.insert(Value::String(String::from("transport_pool")), Value::Sequence(transport_pool));
            mapping
          }
        };

        Some(network_mapping)
      },
      None => None,
    };

    let network_mapping = match bootstrap_server_url {
      Some(url) => {
        match network_mapping {
          Some(mut mapping) => {
            mapping.insert(Value::String(String::from("bootstrap_service")), Value::String(url));
            Some(mapping)
          },
          None => {
            let mut mapping = Mapping::new();
            mapping.insert(Value::String(String::from("bootstrap_service")), Value::String(url));
            Some(mapping)
          }
        }
      },
      None => network_mapping
    };

    if let Some(mapping) = network_mapping {
      config.insert(Value::String(String::from("network")), Value::Mapping(mapping));
    }


    Ok(serde_yaml::to_string(&config).expect("Could not convert conductor config to string"))
  }
}
