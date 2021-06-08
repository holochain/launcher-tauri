use std::{fs, path::PathBuf};
use tauri::api::path::{config_dir, data_dir};

const ADMIN_PORT: u16 = 8889;

pub fn admin_url() -> String {
  format!("ws://localhost:{}", ADMIN_PORT)
}

pub fn holochain_config_path() -> PathBuf {
  config_dir()
    .expect("Could not get config dir")
    .join("holochain")
}

pub fn conductor_config_path() -> PathBuf {
  holochain_config_path().join("conductor-config.yml")
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
  if let Err(_) = fs::read(holochain_config_path()) {
    let _result = fs::create_dir(holochain_config_path());
    let _result = fs::create_dir(holochain_data_path());
    let _result = fs::create_dir(keystore_data_path());
    let _result = fs::create_dir(uis_data_path());
    fs::write(
      conductor_config_path(),
      initial_config(ADMIN_PORT, holochain_data_path()),
    )
    .expect("Could not write conductor config");
  }
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
    passphrase_service: ~
    admin_interfaces: 
        - driver:
            type: websocket
            port: {}
    network:
        network_type: quic_bootstrap
        bootstrap_service: https://bootstrap-staging.holo.host
        transport_pool:
            - type: quic
    "#,
    environment_path.into_os_string().to_str().unwrap(),
    keystore_data_path().into_os_string().to_str().unwrap(),
    admin_port
  )
}
