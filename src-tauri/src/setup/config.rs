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

  if should_overwrite_config() {
    write_initial_config(admin_port);
  }
}

fn should_overwrite_config() -> bool {
  if let Err(_) = fs::read(conductor_config_path()) {
    return true;
  }
  if let Err(_) = get_current_config() {
    return true;
  }
  false
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

fn setup_conductor_config(admin_port: u16) -> Result<(), String> {
  let mut current_config = get_current_config()?;

  current_config.admin_interfaces = Some(vec![AdminInterfaceConfig {
    driver: InterfaceDriver::Websocket { port: admin_port },
  }]);
  current_config.environment_path = holochain_data_path().into();
  current_config.keystore = KeystoreConfig::LairServerLegacyDeprecated {
    keystore_path: Some(keystore_data_path()),
    danger_passphrase_insecure_from_config: "test-passphrase".to_string(),
  };

  write_config(current_config)
}

fn write_config(conductor_config: ConductorConfig) -> Result<(), String> {
  let serde_config = serde_yaml::to_string(&conductor_config)
    .map_err(|err| format!("Could not serialize initial conductor config: {}", err))?;

  fs::write(conductor_config_path(), serde_config).expect("Could not write conductor config");

  Ok(())
}

fn write_initial_config(admin_port: u16) -> Result<(), String> {
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

  let config = ConductorConfig {
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
    db_sync_level: Default::default(),
  };
  write_config(config)

  /*
  format!(
    r#"---
    environment_path: {}
    signing_service_uri: ~
    encryption_service_uri: ~
    decryption_service_uri: ~
    dpki: ~
    keystore:
        type: lair_server_legacy_deprecated
        keystore_path: {}
        danger_passphrase_insecure_from_config: "test-passphrase"
    admin_interfaces:
        - driver:
            type: websocket
            port: {}
    db_sync_level: Normal
    network:
        network_type: quic_bootstrap
        bootstrap_service: https://bootstrap-staging.holo.host
        transport_pool:
          - type: proxy
            sub_transport:
              type: quic
            proxy_config:
              type: remote_proxy_client
              proxy_url: "kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--"
        tuning_params:
            gossip_strategy: sharded-gossip
            gossip_loop_iteration_delay_ms: "1000"
            gossip_outbound_target_mbps: "0.5"
            gossip_inbound_target_mbps: "0.5"
            gossip_historic_outbound_target_mbps: "0.1"
            gossip_historic_inbound_target_mbps: "0.1"
            gossip_peer_on_success_next_gossip_delay_ms: "60000"
            gossip_peer_on_error_next_gossip_delay_ms: "300000"
            gossip_local_sync_delay_ms: "60000"
            gossip_dynamic_arcs: "false"
            gossip_single_storage_arc_per_space: "false"
            default_rpc_single_timeout_ms: "30000"
            default_rpc_multi_remote_agent_count: "3"
            default_rpc_multi_remote_request_grace_ms: "3000"
            agent_info_expires_after_ms: "1200000"
            tls_in_mem_session_storage: "512"
            proxy_keepalive_ms: "120000"
            proxy_to_expire_ms: "300000"
            concurrent_limit_per_thread: "4096"
            tx2_quic_max_idle_timeout_ms: "30000"
            tx2_pool_max_connection_count: "4096"
            tx2_channel_count_per_connection: "16"
            tx2_implicit_timeout_ms: "30000"
            tx2_initial_connect_retry_delay_ms: "200"
    "#,
    environment_path.into_os_string().to_str().unwrap(),
    keystore_data_path().into_os_string().to_str().unwrap(),
    admin_port
  ) */
}

fn create_dir_if_necessary(path: PathBuf) {
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::create_dir(path);
  }
}
