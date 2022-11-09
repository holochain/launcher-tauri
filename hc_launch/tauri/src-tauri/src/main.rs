#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use holochain_client::AdminWebsocket;
use serde_json::value::Value;
use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::mpsc;
use tauri::Window;
use tauri::{AppHandle, Manager};
use lair_keystore_api::{LairClient, ipc_keystore_connect};
use url::Url;
mod utils;
mod commands;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use commands::sign_zome_call::sign_zome_call;

fn main() {
  tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![sign_zome_call]) // uncomment when testing with right version
    .setup(|app| {
      let cli_matches = app.get_cli_matches()?;

      let pwd = std::env::current_dir().unwrap();
      // let assets_path: PathBuf = pwd.parent().unwrap().parent().unwrap().join(".hc_launch").join("ui").into();
      let assets_path: PathBuf = match cli_matches.args.get("ui-path") {
        Some(data) => match data.value.clone() {
          Value::String(path) => path.into(),
          _ => {
            println!("ERROR: Value passed to --ui-path option could not be interpreted as string.");
            panic!("Value passed to --ui-path option could not be interpreted as string.")
          }
        },
        None => pwd.join(".hc_launch").join("ui").into(),
      };

      println!("Does path exist? {}", assets_path.exists());
      if !assets_path.exists() {
        println!("ERROR: Specified UI path does not exist.");
        panic!("Specified UI path does not exist.");
      }

      println!("current working directory: {:?}", pwd);

      println!("path to assets: {:?}", assets_path);

      // read the .hc file to get the number of apps
      // let dot_hc_path = pwd.parent().unwrap().parent().unwrap().join(".hc");
      let dot_hc_path = pwd.join(".hc");

      println!("path to .hc: {:?}", dot_hc_path);

      let dot_hc_content = std::fs::read_to_string(dot_hc_path).unwrap();

      println!("content of .hc: {:?}", dot_hc_content);

      // need to get lair client for each of the windows
      // window calls tauri command sign_zome_call() --> I need to identify which window did the call (can get this from window label)
      // need a hashmap of lair clients with window labels as keys




      // open a tauri window for each app instance and create a lair client instance for each window
      let mut windows: Vec<Window> = vec![];
      let mut lair_clients: HashMap<String, LairClient> = HashMap::new();

      let mut app_counter = 0;

      tauri::async_runtime::block_on( async move {

        for tmp_directory_path in dot_hc_content.lines() {
          // let app_id = format!("Agent-{}", app_counter);
          let app_id = String::from("test-app");

          // let dot_hc_live_path: PathBuf = pwd.parent().unwrap().parent().unwrap().join(format!(".hc_live_{}", app_counter)).into();
          let dot_hc_live_path: PathBuf = pwd.join(format!(".hc_live_{}", app_counter)).into();

          println!(
            "path to .hc_live_{} file: {:?}",
            app_counter, dot_hc_live_path
          );

          let admin_port = std::fs::read_to_string(dot_hc_live_path).unwrap();

          println!("admin port: {:?}", admin_port);

          let admin_port_clone = admin_port.clone();

          println!("trying to get app port");
          let app_port = tauri::async_runtime::block_on(async move {
            match get_app_websocket(admin_port_clone).await {
              Ok(ws) => ws,
              Err(e) => {
                println!("ERROR! Error getting app websocket port: {}", e);
                panic!("Failed to get app websocket port.");
              }
            }
          });

          println!("app port: {:?}", app_port);

          // extract the number of lines of the .hc file to know the number of sandboxes
          // read all the hc_live_X files to retrieve the admin ports

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

          println!("Starting to build window.");

          let window_label = format!("Agent-{}", app_counter);

          let window = match utils::generate_window(
            &app.handle(),
            &app_id,
            window_label.clone(),
            assets_path.clone().join("index.html"),
            assets_path.clone(),
            launcher_env,
          ) {
            Ok(window) => window,
            Err(e) => {
              println!("ERROR! Failed to build window: {}", e);
              panic!("Failed to build Window: {:?}", e);
            }
          };

          let a = app.handle().clone();

          window.on_menu_event(move |_| {
            if let Some(w) = a.get_window(window_label.as_str()) {
              w.open_devtools();
            }
          });

          println!("App window created.");
          windows.push(window);

          // create lair client and add it to hashmap
          let connection_url = Url::parse(
            PathBuf::from(tmp_directory_path).join("keystore")
              .as_os_str()
              .to_str()
              .expect("Failed to convert OsString to &str")
          ).expect("Failed to parse lair url");

          let client =   ipc_keystore_connect(connection_url, String::from("pass").into_bytes())
            .await
            .expect(format!("Failed to connect to lair client at url: {:?}", connection_url).as_str());

          lair_clients.insert(window_label.to_string(), client);

          app_counter += 1;
        }
      });


      // watch for file changes in the UI folder if requested
      match cli_matches.args.get("watch") {
        Some(data) => {
          match data.value.clone() {
            Value::Bool(true) => {
              println!(
                "Watching file changes in folder {:?}",
                assets_path.as_path()
              );

              let watch_handle = std::thread::spawn(move || {
                let (tx_watcher, rx_watcher) = std::sync::mpsc::channel();

                let mut watcher = match RecommendedWatcher::new(tx_watcher, Config::default()) {
                  Ok(w) => w,
                  Err(e) => panic!("Failed to create file system watcher: {:?}", e),
                };

                match watcher.watch(assets_path.as_path(), RecursiveMode::Recursive) {
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

              Some(watch_handle)
            }
            _ => None,
          }
        }
        _ => None,
      };

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
