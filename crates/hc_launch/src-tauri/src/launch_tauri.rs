#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use holochain_client::AdminWebsocket;
use lair_keystore_api::dependencies::sodoken;
use tauri::RunEvent;
use std::path::PathBuf;
use std::collections::HashMap;
use tauri::Window;
use tauri::WindowEvent;
use tauri::Manager;
use lair_keystore_api::{LairClient, ipc_keystore_connect};
use url::Url;

use crate::utils;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};


pub fn launch_tauri(ui_path: PathBuf, watch: bool, passphrase: sodoken::BufRead) -> () {

  // tauri::async_runtime::set(tokio::runtime::Handle::current());

  // build tauri windows
  let builder_result = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![crate::commands::sign_zome_call::sign_zome_call]) // uncomment when testing with right version
    .setup(move |app| {

      let pwd = std::env::current_dir().unwrap();

      // launch tauri windows
      println!("Waiting a few seconds before starting tauri windows...");
      std::thread::sleep(std::time::Duration::from_millis(8000));


      // read the .hc file to get the number of apps
      let dot_hc_path = pwd.join(".hc");

      let dot_hc_content = match std::fs::read_to_string(dot_hc_path) {
        Ok(p) => p,
        Err(e) => {
          println!("Failed to read content of .hc file: {}", e);
          panic!("Failed to read content of .hc file: {}", e);
        }
      };

      // open a tauri window for each app instance and create a lair client instance for each window
      let mut windows: Vec<Window> = vec![];
      let mut lair_clients: HashMap<String, LairClient> = HashMap::new();

      let mut app_counter = 0;

      let (windows, ui_path, lair_clients, app) =
        tokio::task::block_in_place( || {
          tauri::async_runtime::block_on( async move {

          for tmp_directory_path in dot_hc_content.lines() {

            let app_id = String::from("test-app");
            let dot_hc_live_path: PathBuf = pwd.join(format!(".hc_live_{}", app_counter)).into();

            let admin_port = match std::fs::read_to_string(dot_hc_live_path) {
              Ok(p) => p,
              Err(e) => {
                println!("Failed to read content of .hc_live file: {}", e);
                panic!("Failed to read content of .hc_live file: {}", e);
              }
            };

            let admin_port_clone = admin_port.clone();

            let app_port = match get_app_websocket(admin_port_clone).await {
              Ok(ws) => ws,
              Err(e) => {
                println!("ERROR! Error getting app websocket port: {}", e);
                panic!("Failed to get app websocket port.");
              }
            };

            // TODO! implement writing it to window object instead
            let launcher_env = format!(
              r#"{{
                "APP_INTERFACE_PORT": {},
                "ADMIN_INTERFACE_PORT": {},
                "INSTALLED_APP_ID": "{}"
              }}"#,
              app_port,
              admin_port,
              app_id.clone(),
            );

            let window_label = format!("Agent-{}", app_counter);

            let window = match utils::generate_window::generate_window(
              &app.handle(),
              &app_id,
              window_label.clone(),
              ui_path.clone().join("index.html"),
              ui_path.clone(),
              launcher_env,
            ) {
              Ok(window) => window,
              Err(e) => {
                println!("ERROR! Failed to build window: {}", e);
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

            window.on_window_event(|event| {
              match event {
                WindowEvent::CloseRequested { api: _, .. } => (),
                _ => (),
              }
            });

            windows.push(window);

            // read lair-keystore-config.yaml file
            let connection_url = match read_lair_yaml(PathBuf::from(tmp_directory_path)) {
              Some(url) => url,
              None => {
                println!("ERROR: No connectionUrl found in lair-keystore-config.yaml");
                panic!("No connectionUrl found in lair-keystore-config.yaml")
              }
            };

            let connection_url = Url::parse(connection_url.as_str()).unwrap();

            // create lair client and add it to hashmap
            let client = match ipc_keystore_connect(connection_url.clone(), passphrase.clone()).await {
              Ok(client) => client,
              Err(e) => {
                println!("Failed to connect to lair client: {:?}", e);
                panic!("Failed to connect to lair client");
              }
            };

            lair_clients.insert(window_label.to_string(), client);

            app_counter += 1;
          }

          (windows, ui_path, lair_clients, app)
        })
      });

      app.manage(lair_clients);


      // watch for file changes in the UI folder if requested
      match watch {
        true => {
          println!(
            "Watching file changes in folder {:?}",
            ui_path.as_path()
          );

          let _watch_handle = std::thread::spawn(move || {
            let (tx_watcher, rx_watcher) = std::sync::mpsc::channel();

            let mut watcher = match RecommendedWatcher::new(tx_watcher, Config::default()) {
              Ok(w) => w,
              Err(e) => panic!("Failed to create file system watcher: {:?}", e),
            };

            match watcher.watch(ui_path.as_path(), RecursiveMode::Recursive) {
              Ok(()) => (),
              Err(e) => {
                println!("Failed to watch: {:?}", e);
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
                Err(e) => println!("watch error: {:?}", e),
              }
            }
          });
        },

        false => (),
      }


      // // release app ports
      // holochain_cli_sandbox::save::release_ports(std::env::current_dir()?).await?;

      Ok(())
    })
    .build(tauri::generate_context!());

    match builder_result {
      Ok(builder) => {
        builder.run(move |_app_handle, event| {
          if let RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
          }
        });
      },
      Err(e) => eprintln!("Error building tauri windows: {:?}", e)
    }

    // .run(tauri::generate_context!())
    // .expect("error while running tauri application");
}


async fn get_app_websocket(admin_port: String) -> Result<u16, String> {
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
    PathBuf::from(path).join("keystore").join("lair-keystore-config.yaml")) {
      Ok(content) => content,
      Err(e) => {
        println!("Failed to read lair-keystore-config.yaml: {:?}", e);
        panic!("Failed to read lair-keystore-config.yaml");
      }
    };

    for line in yaml_content.lines() {
      match line.contains("connectionUrl") {
        true => return Some(line[15..].to_string()),
        false => ()
      }
    }

    None
}