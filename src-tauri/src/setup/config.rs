use holochain_conductor_api::{
  conductor::KeystoreConfig, config::conductor::ConductorConfig, AdminInterfaceConfig,
  InterfaceDriver,
};
use holochain_p2p::kitsune_p2p::*;
use std::{fs, path::PathBuf};
use tauri::api::path::{config_dir, data_dir};
use url2;

pub fn holochain_config_path() -> PathBuf {
  config_dir()
    .expect("Could not get config dir")
    .join("holochain")
}

pub fn conductor_config_path() -> PathBuf {
  holochain_config_path().join("conductor-config.yml")
}

pub fn logs_path() -> PathBuf {
  logs_folder_path().join("launcher.log")
}

pub fn logs_folder_path() -> PathBuf {
  data_dir()
    .expect("Could not get config dir")
    .join("holochain-launcher")
}

pub fn caddyfile_path() -> PathBuf {
  uis_data_path().join("Caddyfile")
}

pub fn uis_data_path() -> PathBuf {
  holochain_data_path().join("uis")
}

pub fn holochain_data_path() -> PathBuf {
  data_dir()
    .expect("Could not get config dir")
    .join("holochain")
}

pub fn keystore_data_path() -> PathBuf {
  data_dir().expect("Could not get config dir").join("lair")
}

// Also set the admin port and keystore path in the config
pub fn setup_config(admin_port: u16) -> () {
  create_dir_if_necessary(holochain_config_path());
  create_dir_if_necessary(holochain_data_path());
  create_dir_if_necessary(keystore_data_path());
  create_dir_if_necessary(uis_data_path());
  create_dir_if_necessary(logs_folder_path());

  setup_conductor_config(admin_port);
}

fn get_current_config() -> Result<ConductorConfig, String> {
  ConductorConfig::load_yaml(&conductor_config_path())
    .map_err(|err| format!("Unable to open the conductor config file: {}", err))
}

pub fn config_admin_port() -> Result<Option<u16>, String> {
  let current_config = get_current_config()?;
  if let Some(admin_interfaces) = current_config.admin_interfaces {
    if let Some(interface) = admin_interfaces.get(0) {
      let InterfaceDriver::Websocket { port } = interface.driver;
      return Ok(Some(port));
    }
  }

  Ok(None)
}

fn setup_conductor_config(admin_port: u16) -> () {
  let mut config = match get_current_config() {
    Ok(config) => config,
    Err(_) => initial_config(admin_port),
  };

  config.admin_interfaces = Some(vec![AdminInterfaceConfig {
    driver: InterfaceDriver::Websocket { port: admin_port },
  }]);
  config.environment_path = holochain_data_path().into();
  config.keystore = KeystoreConfig::LairServerLegacyDeprecated {
    keystore_path: Some(keystore_data_path()),
    danger_passphrase_insecure_from_config: "test-passphrase".to_string(),
  };

  write_config(config)
}

fn write_config(conductor_config: ConductorConfig) -> () {
  let serde_config =
    serde_yaml::to_string(&conductor_config).expect("Could not serialize initial conductor config");

  fs::write(conductor_config_path(), serde_config).expect("Could not write conductor config");
}

fn initial_config(admin_port: u16) -> ConductorConfig {
  let mut network_config = KitsuneP2pConfig::default();
  network_config.bootstrap_service = Some(url2::url2!("https://bootstrap-staging.holo.host"));
  network_config.transport_pool.push(TransportConfig::Proxy {
      sub_transport: Box::new(TransportConfig::Quic {
          bind_to: None,
          override_host: None,
          override_port: None,
      }),
      proxy_config: ProxyConfig::RemoteProxyClient {
        proxy_url: url2::url2!("kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--"),
      },
  });
  let mut tuning_params =
      holochain_p2p::kitsune_p2p::dependencies::kitsune_p2p_types::config::tuning_params_struct::KitsuneP2pTuningParams::default();
  tuning_params.gossip_strategy = "sharded-gossip".to_string();

  ConductorConfig {
    environment_path: holochain_data_path().into(),
    dpki: None,
    keystore: KeystoreConfig::LairServerLegacyDeprecated {
      keystore_path: Some(keystore_data_path()),
      danger_passphrase_insecure_from_config: "test-passphrase".to_string(),
    },
    admin_interfaces: Some(vec![AdminInterfaceConfig {
      driver: InterfaceDriver::Websocket { port: admin_port },
    }]),
    network: Some(network_config),
    db_sync_strategy: Default::default(),
  }
}

fn create_dir_if_necessary(path: PathBuf) {
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::create_dir(path);
  }
}
