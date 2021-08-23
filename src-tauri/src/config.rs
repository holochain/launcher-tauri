use std::{fs, path::PathBuf};
use tauri::api::path::{config_dir, data_dir};

pub const DEFAULT_ADMIN_PORT: u16 = 8889;
pub const DEFAULT_APP_PORT: u16 = 8888;

pub fn admin_url() -> String {
  format!("ws://localhost:{}", DEFAULT_ADMIN_PORT)
}

pub fn holochain_config_path() -> PathBuf {
  config_dir()
    .expect("Could not get config dir")
    .join("holochain")
}

pub fn conductor_config_path() -> PathBuf {
  holochain_config_path().join("conductor-config.yml")
}

pub fn logs_path() -> PathBuf {
  holochain_data_path().join("launcher.log")
}

pub fn holochain_logs_path() -> PathBuf {
  holochain_data_path().join("holochain.log")
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

pub fn create_initial_config_if_necessary() -> () {
  create_dir_if_necessary(holochain_config_path());
  create_dir_if_necessary(holochain_data_path());
  create_dir_if_necessary(keystore_data_path());
  create_dir_if_necessary(uis_data_path());
  fs::write(
    conductor_config_path(),
    initial_config(DEFAULT_ADMIN_PORT, holochain_data_path()),
  )
  .expect("Could not write conductor config");
}

fn initial_config(admin_port: u16, environment_path: PathBuf) -> String {
  format!(
    r#"---
    environment_path: {}
    use_dangerous_test_keystore: false
    signing_service_uri: ~
    encryption_service_uri: ~
    decryption_service_uri: ~
    dpki: ~
    keystore_path: {}
    passphrase_service:
        type: danger_insecure_from_config
        passphrase: "test-passphrase"
    admin_interfaces: 
        - driver:
            type: websocket
            port: {}
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
            gossip_peer_on_success_next_gossip_delay_ms: 500
            gossip_peer_on_error_next_gossip_delay_ms: 2000
            gossip_loop_iteration_delay_ms: 42
            default_notify_remote_agent_count: 5
            default_notify_timeout_ms: 1000
            default_rpc_single_timeout_ms: 20000
            default_rpc_multi_remote_agent_count: 2
            default_rpc_multi_timeout_ms: 2000
            agent_info_expires_after_ms: 180000
            tls_in_mem_session_storage: 512
            proxy_keepalive_ms: 120000
            proxy_to_expire_ms: 300000
    "#,
    environment_path.into_os_string().to_str().unwrap(),
    keystore_data_path().into_os_string().to_str().unwrap(),
    admin_port
  )
}

fn create_dir_if_necessary(path: PathBuf) {
  if let Err(_) = fs::read(path.clone()) {
    let _result = fs::create_dir(path);
  }
}
