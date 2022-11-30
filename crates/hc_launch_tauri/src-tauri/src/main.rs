#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use holochain_client::AdminWebsocket;
use serde_json::value::Value;
use std::path::PathBuf;
use std::collections::HashMap;
use tauri::Window;
use tauri::Manager;
use lair_keystore_api::{LairClient, ipc_keystore_connect};
use url::Url;
mod utils;
mod commands;
mod error;
mod cli;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use commands::sign_zome_call::sign_zome_call;

use structopt::StructOpt;



fn main() {
  // run sandboxes

  // build tauri windows
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![sign_zome_call]) // uncomment when testing with right version
    .setup(|app| {

      // clean existing sandboxes
      holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

      let args = tauri::async_runtime::block_on(async move {

        if std::env::var_os("RUST_LOG").is_some() {
            observability::init_fmt(observability::Output::Log).ok();
        }
        let opt = cli::HcLaunch::from_args();

        let args = opt.args().unwrap();
        match opt.run().await {
          Ok(()) => (),
          Err(e) => {
            println!("Failed to run HcLaunch: {:?}", e);
            panic!("Failed to run HcLaunch: {:?}", e);
          }
        }

        args
      });

      println!("@tauri main: conductors launched.");

      println!("waiting a few seconds before starting tauri windows...");
      std::thread::sleep(std::time::Duration::from_millis(5000));

      let pwd = std::env::current_dir().unwrap();

      println!("@tauri main: matching ui_path.");

      let ui_path = match args.ui_path {
        Some(p) => p,
        None => pwd.join(".hc_launch").join("ui").into(), // TODO! switch to tmp directory for ui and .happ
      };

      println!("@tauri main: B.");


      // read the .hc file to get the number of apps
      let dot_hc_path = pwd.join(".hc");

      println!("@tauri main: C");

      let dot_hc_content = std::fs::read_to_string(dot_hc_path).unwrap();

      println!("@tauri main: D");

      // open a tauri window for each app instance and create a lair client instance for each window
      let mut windows: Vec<Window> = vec![];
      let mut lair_clients: HashMap<String, LairClient> = HashMap::new();

      let mut app_counter = 0;

      println!("@tauri main: E");


      let (windows, ui_path, lair_clients, app) = tauri::async_runtime::block_on( async move {

        for tmp_directory_path in dot_hc_content.lines() {

          let app_id = String::from("test-app");
          let dot_hc_live_path: PathBuf = pwd.join(format!(".hc_live_{}", app_counter)).into();
          println!("@tauri main: F");

          let admin_port = std::fs::read_to_string(dot_hc_live_path).unwrap();

          println!("@tauri main: G");

          let admin_port_clone = admin_port.clone();

          let app_port = match get_app_websocket(admin_port_clone).await {
            Ok(ws) => ws,
            Err(e) => {
              println!("ERROR! Error getting app websocket port: {}", e);
              panic!("Failed to get app websocket port.");
            }
          };

          println!("@tauri main: H");


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
          let client = match ipc_keystore_connect(connection_url.clone(), "".as_bytes()).await {
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
      });

      app.manage(lair_clients);


      // watch for file changes in the UI folder if requested
      match args.watch {
        true => {
          println!(
            "Watching file changes in folder {:?}",
            ui_path.as_path()
          );

          let watch_handle = std::thread::spawn(move || {
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

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
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