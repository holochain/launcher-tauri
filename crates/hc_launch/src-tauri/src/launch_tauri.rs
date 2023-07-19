#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use holochain_client::AdminWebsocket;
use holochain_types::prelude::AgentPubKey;
use holochain_launcher_utils::window_builder::{happ_window_builder, UISource};
use tauri::utils::config::AppUrl;
use tauri::WindowUrl;
use tauri::{CustomMenuItem, Menu, Submenu};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::Manager;
use tauri::RunEvent;
use tauri::Window;
use tauri::WindowEvent;
use url::Url;

use lair_keystore_api::dependencies::sodoken;
use lair_keystore_api::{LairClient, ipc_keystore_connect};


use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};


pub fn launch_tauri(
  ui_source: UISource,
  app_id: String,
  local_storage_dir: PathBuf,
  watch: bool,
  passphrase: sodoken::BufRead
) -> () {
  // tauri::async_runtime::set(tokio::runtime::Handle::current());

  let mut context = tauri::generate_context!();

  // in case of a localhost port, mutate the context for IPC to be enabled on this URL
  if let UISource::Port(port) = ui_source {
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());
  }

  let builder = match ui_source {
    UISource::Path(_) => tauri::Builder::default(),
    UISource::Port(port) => tauri::Builder::default()
    .plugin(tauri_plugin_localhost::Builder::new(port).build())
  };

  // build tauri windows
  let builder_result = builder
    .invoke_handler(tauri::generate_handler![
      crate::commands::sign_zome_call::sign_zome_call,
      holochain_launcher_utils::shared_commands::open_url_cmd,
    ]) // uncomment when testing with right version
    .setup(move |app| {
      let pwd = std::env::current_dir().unwrap();

      // launch tauri windows

      // read the .hc file to get the number of apps
      let dot_hc_path = pwd.join(".hc");

      let dot_hc_content = match std::fs::read_to_string(dot_hc_path) {
        Ok(p) => p,
        Err(e) => {
          println!("[hc launch] ERROR: Failed to read content of .hc file: {}", e);
          panic!("Failed to read content of .hc file: {}", e);
        }
      };

      // open a tauri window for each app instance and create a lair client instance for each window
      let mut windows: Vec<Window> = vec![];
      let mut lair_clients: HashMap<String, LairClient> = HashMap::new();
      // map to store pubkeys allowed to make zome calls from a given window (key: window label, value: agent public key)
      let mut pubkey_map: HashMap<String, AgentPubKey> = HashMap::new();

      let mut app_counter = 0;

      let (windows, ui_source, lair_clients, pubkey_map, app) = tokio::task::block_in_place(|| {
        tauri::async_runtime::block_on(async move {
          for tmp_directory_path in dot_hc_content.lines() {
            let dot_hc_live_path: PathBuf = pwd.join(format!(".hc_live_{}", app_counter)).into();

            let admin_port = match std::fs::read_to_string(dot_hc_live_path) {
              Ok(p) => p,
              Err(e) => {
                println!("[hc launch] ERROR: Failed to read content of .hc_live file: {}", e);
                panic!("Failed to read content of .hc_live file: {}", e);
              }
            };

            let admin_port_string = admin_port.clone();

            let app_port = match get_app_websocket(admin_port_string).await {
              Ok(ws) => ws,
              Err(e) => {
                println!("[hc launch] ERROR:  Failed to get app websocket port: {}", e);
                panic!("Failed to get app websocket port.");
              }
            };

            let admin_port = match admin_port.trim().parse::<u16>() {
              Ok(u) => u,
              Err(e) => {
                println!("[hc launch] ERROR: Failed to convert admin port from String to u16: {}", e);
                panic!("Failed to convert admin port from String to u16: {}", e);
              }
            };

            // constraint by tauri that window labels can only contain alphanumeric characters, `-`, `/`, `:` and `_`
            let sanitized_app_id = sanitize_app_id(app_id.clone());

            let window_label = derive_window_label(app_counter, app_id.clone());
            let window_title = format!("Conductor {} - {}", app_counter, sanitized_app_id);

            let window_width = 1000.0;
            let window_height = 700.0;

            let app_handle = app.handle();

            let mut window_builder = match ui_source.clone() {
              UISource::Path(ui_path) => {
                happ_window_builder(
                  &app_handle,
                  app_id.clone(),
                  window_label.clone(),
                  window_title.clone(),
                  UISource::Path(ui_path.clone()),
                  local_storage_dir.clone().join(format!("Conductor-{}-{}", app_counter, sanitized_app_id)),
                  app_port,
                  admin_port,
                  false,
                )
              },
              UISource::Port(ui_port) => {
                happ_window_builder(
                  &app_handle,
                  app_id.clone(),
                  window_label.clone(),
                  window_title.clone(),
                  UISource::Port(ui_port),
                  local_storage_dir.clone().join(format!("Conductor-{}-{}", app_counter, sanitized_app_id)),
                  app_port,
                  admin_port,
                  false,
                )
              }
            };

            window_builder = window_builder.inner_size(window_width, window_height);

            let window = match window_builder
              .menu(Menu::new().add_submenu(Submenu::new(
                "Settings",
                Menu::new().add_item(CustomMenuItem::new("show-devtools", "Show DevTools")),
              )))
              .build() {
                Ok(window) => window,
                Err(e) => {
                  println!("[hc launch] ERROR: Failed to build window: {}", e);
                  panic!("Failed to build Window: {:?}", e);
                }
            };

            let a = app.handle().clone();

            let window_label_clone = window_label.clone();

            window.on_menu_event(move |_| {
              if let Some(w) = a.get_window(window_label_clone.as_str()) {
                w.open_devtools();
              }
            });

            window.on_window_event(|event| match event {
              WindowEvent::CloseRequested { api: _, .. } => (),
              _ => (),
            });

            windows.push(window);

            // get public key of installed app and add it to the pubkey map used to verify provenance
            let mut ws = match AdminWebsocket::connect(format!("ws://localhost:{}", admin_port)).await {
              Ok(ws) => ws,
              Err(e) => panic!("Failed to connect to admin websocket: {:?}", e),
            };

            let installed_apps = match ws.list_apps(None).await {
              Ok(apps) => apps,
              Err(e) => panic!("Failed to list apps : {:?}", e),
            };

            for app in installed_apps {
              if derive_window_label(app_counter, app.installed_app_id) == window_label.clone() {
                pubkey_map.insert(window_label.clone(), app.agent_pub_key.clone());
                // println!("[hc launch] Inserted public key {} as allowed public key for window with label {}", app.agent_pub_key, window_label);
              }
            }

            // read lair-keystore-config.yaml file
            let connection_url = match read_lair_yaml(PathBuf::from(tmp_directory_path)) {
              Some(url) => url,
              None => {
                println!("[hc launch] ERROR: No connectionUrl found in lair-keystore-config.yaml");
                panic!("No connectionUrl found in lair-keystore-config.yaml")
              }
            };

            let connection_url = Url::parse(connection_url.as_str()).unwrap();

            // create lair client and add it to hashmap
            let client =
              match ipc_keystore_connect(connection_url.clone(), passphrase.clone()).await {
                Ok(client) => client,
                Err(e) => {
                  println!("[hc launch] ERROR: Failed to connect to lair client: {:?}", e);
                  panic!("Failed to connect to lair client");
                }
              };

            lair_clients.insert(window_label.to_string(), client);

            app_counter += 1;
          }

          (windows, ui_source, lair_clients, pubkey_map, app)
        })
      });

      app.manage(lair_clients);
      app.manage(pubkey_map);

      // watch for file changes in the UI folder if requested
      match (watch, ui_source) {
        (true, UISource::Path(ui_path)) => {
          println!("[hc launch] Watching file changes in folder {:?}", ui_path.as_path());

          let _watch_handle = std::thread::spawn(move || {
            let (tx_watcher, rx_watcher) = std::sync::mpsc::channel();

            let mut watcher = match RecommendedWatcher::new(tx_watcher, Config::default()) {
              Ok(w) => w,
              Err(e) => panic!("Failed to create file system watcher: {:?}", e),
            };

            match watcher.watch(ui_path.as_path(), RecursiveMode::Recursive) {
              Ok(()) => (),
              Err(e) => {
                println!("[hc launch] ERROR: Failed to watch: {:?}", e);
                panic!("Failed to watch.");
              }
            };

            for res in rx_watcher {
              match res {
                Ok(event) => {
                  println!("event: {:?}", event);
                  for window in &windows {
                    window.eval("location.reload()").unwrap();
                  }
                }
                Err(e) => println!("[hc launch] ERROR: watch error: {:?}", e),
              }
            }
          });
        },
        (true, UISource::Port(_port)) => println!("[hc launch] WARNING: The --watch flag has no effect if the UI is served from localhost."),
        _ => (),
      }

      // // release app ports
      // holochain_cli_sandbox::save::release_ports(std::env::current_dir()?).await?;

      Ok(())
    })
    .build(context);

  match builder_result {
    Ok(builder) => {
      builder.run(move |_app_handle, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
          api.prevent_exit();
        }
      });
    }
    Err(e) => eprintln!("[hc launch] ERROR: Failed to build tauri windows: {:?}", e),
  }

  // .run(tauri::generate_context!())
  // .expect("error while running tauri application");
}

async fn get_app_websocket(admin_port: String) -> Result<u16, String> {
  println!("ADMIN PORT: {}", admin_port);
  // Try to connect twice. This fixes the os(111) error for now that occurs when the conducor is not ready yet.
  let mut ws = match AdminWebsocket::connect(format!("ws://localhost:{}", admin_port)).await {
    Ok(ws) => ws,
    Err(e) => return Err(format!("Could not connect to admin websocket: {:?}", e)),
  };

  let app_interface_port = {
    let app_interfaces = ws
      .list_app_interfaces()
      .await
      .or(Err(format!("Could not list app websocket interfaces.")))?;

    if app_interfaces.len() > 0 {
      app_interfaces[0]
    } else {
      let free_port = portpicker::pick_unused_port().expect("No ports free");

      ws.attach_app_interface(free_port)
        .await
        .or(Err(format!("Could not attach app websocket interface.")))?;
      free_port
    }
  };

  Ok(app_interface_port)
}

fn read_lair_yaml(path: PathBuf) -> Option<String> {
  let yaml_content = match std::fs::read_to_string(
    PathBuf::from(path)
      .join("keystore")
      .join("lair-keystore-config.yaml"),
  ) {
    Ok(content) => content,
    Err(e) => {
      println!("[hc launch] ERROR: Failed to read lair-keystore-config.yaml: {:?}", e);
      panic!("Failed to read lair-keystore-config.yaml");
    }
  };

  for line in yaml_content.lines() {
    match line.contains("connectionUrl") {
      true => return Some(line[15..].to_string()),
      false => (),
    }
  }

  None
}

// Derives the window label from the app id and the conductor's number
fn derive_window_label(conductor_nr: i32, app_id: String) -> String {
  // constraint by tauri that window labels can only contain alphanumeric characters, `-`, `/`, `:` and `_`
  let sanitized_app_id = sanitize_app_id(app_id);
  return format!("Conductor-{}-{}", conductor_nr, sanitized_app_id);
}


fn sanitize_app_id(app_id: String) -> String {
  return app_id.replace("-", "--").replace(" ", "-").replace(".", "_");
}