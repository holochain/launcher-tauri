use std::{collections::HashMap, thread, time::Duration};

use holochain_client::AdminWebsocket;
use tauri::api::process::{Command, CommandEvent};

use crate::{setup::config, state::RunningPorts, uis::caddy};

pub async fn launch_children_processes(running_ports: RunningPorts) -> Result<(), String> {
  config::setup_config(running_ports.admin_interface_port);

  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from("warn"));

  let (mut lair_rx, _) = Command::new_sidecar("lair-keystore")
    .or(Err(String::from("Can't find lair-keystore binary")))?
    .args(&[
      "-d",
      config::keystore_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    ])
    .envs(envs.clone())
    .spawn()
    .map_err(|err| format!("Failed to execute lair-keystore: {:?}", err))?;

  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = lair_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[LAIR] {}", line),
        CommandEvent::Stderr(line) => log::info!("[LAIR] {}", line),
        _ => log::info!("[LAIR] {:?}", event),
      }
    }
  });

  log::info!("Launched lair-keystore");

  thread::sleep(Duration::from_millis(1000));

  let (mut holochain_rx, holochain_child) = Command::new_sidecar("holochain")
    .or(Err(String::from("Can't find holochain binary")))?
    .args(&[
      "-c",
      config::conductor_config_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    ])
    .envs(envs)
    .spawn()
    .map_err(|err| format!("Failed to execute holochain: {:?}", err))?;

  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = holochain_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[HOLOCHAIN] {}", line),
        CommandEvent::Stderr(line) => log::info!("[HOLOCHAIN] {}", line),
        _ => log::info!("[HOLOCHAIN] {:?}", event),
      };
      if format!("{:?}", event).contains("Installing lair_keystore") {
        // Lair keystore can't be executed, Holochain is trying to download and install Lair, kill it
        log::error!("Holochain is trying to download and install lair_keystore directly! Killing Holochain...");
        let result = holochain_child.kill();
        log::error!("Holochain terminated: {:?}", result);
        break;
      }
    }
  });
  log::info!("Launched holochain");

  setup_conductor(running_ports.admin_interface_port).await?;

  caddy::launch_caddy(running_ports.clone()).await?;

  launch_plugins(running_ports).await?;

  Ok(())
}

async fn setup_conductor(admin_port: u16) -> Result<(), String> {
  let mut ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  log::info!("Connected to admin conductor");

  let app_interfaces = ws
    .list_app_interfaces()
    .await
    .or(Err(String::from("Could not list app interfaces")))?;

  if app_interfaces.len() == 0 {
    let free_port = portpicker::pick_unused_port().expect("No ports free");

    ws.attach_app_interface(free_port)
      .await
      .or(Err(String::from("Could not attach app interface")))?;
    log::info!("Attached app interface to {}", free_port);
  }

  Ok(())
}

async fn launch_plugins(running_ports: RunningPorts) -> Result<(), String> {
  if config::plugins_folder_path().join("ad4m-macos-x64").exists() {
    // TODO the binary file name needs to load dynamically,
    // the command needs to specify in config,
    // init command is not good, but necessary for now.
    let init_cmd = Command::new(
      config::plugins_folder_path()
        .join("ad4m-macos-x64")
        .to_str()
        .unwrap(),
    )
    .args(&["init"]);
    let (mut rx, _) = init_cmd.spawn().unwrap();

    tauri::async_runtime::spawn(async move {
      while let Some(event) = rx.recv().await {
        match event.clone() {
          CommandEvent::Stdout(line) => log::info!("[Ad4m] {}", line),
          CommandEvent::Stderr(line) => log::info!("[Ad4m] {}", line),
          _ => log::info!("[Ad4m] {:?}", event),
        }
      }
    });
    log::info!("Initialized ad4m service");
    thread::sleep(Duration::from_millis(20000));

    let mut ws = AdminWebsocket::connect(format!(
      "ws://localhost:{}",
      running_ports.admin_interface_port
    ))
    .await
    .or(Err(String::from("Could not connect to conductor")))?;
    let app_interfaces = ws
      .list_app_interfaces()
      .await
      .or(Err(String::from("Could not list app interfaces")))?;

    let serve_cmd = Command::new(
      config::plugins_folder_path()
        .join("ad4m-macos-x64")
        .to_str()
        .unwrap(),
    )
    .args(&[
      "serve",
      "--connectHolochain",
      "--hcAdminPort",
      &running_ports.admin_interface_port.to_string(),
      "--hcAppPort",
      &app_interfaces[0].to_string(),
    ]);
    let (mut rx, _) = serve_cmd.spawn().unwrap();

    tauri::async_runtime::spawn(async move {
      while let Some(event) = rx.recv().await {
        match event.clone() {
          CommandEvent::Stdout(line) => log::info!("[Ad4m] {}", line),
          CommandEvent::Stderr(line) => log::info!("[Ad4m] {}", line),
          _ => log::info!("[Ad4m] {:?}", event),
        }
      }
    });
    log::info!("Launched ad4m service");
    thread::sleep(Duration::from_millis(1000));
  }

  Ok(())
}
