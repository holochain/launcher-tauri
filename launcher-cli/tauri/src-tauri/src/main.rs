#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;
use holochain_client::AdminWebsocket;
use tauri::Window;
mod utils;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![greet])
		.setup(|app| {

				let pwd = std::env::current_dir().unwrap();
				let assets_path: PathBuf = pwd.parent().unwrap().parent().unwrap().join(".launcher-cli").join("ui").into();

				println!("path to assets: {:?}", assets_path);

				// read the .hc file to get the number of apps
				let dot_hc_path = pwd.parent().unwrap().parent().unwrap().join(".hc");
				let dot_hc_content = std::fs::read_to_string(dot_hc_path).unwrap();


				// open a tauri window for each app instance
				let mut windows: Vec<Window> = vec![];
				let mut app_counter = 0;
				for _ in dot_hc_content.lines() {

					let app_id = format!("Agent-{}", app_counter);

					let dot_hc_live_path: PathBuf = pwd.parent().unwrap().parent().unwrap().join(format!(".hc_live_{}", app_counter)).into();

					println!("path to .hc_live_{} file: {:?}", app_counter, dot_hc_live_path);

					let admin_port = std::fs::read_to_string(dot_hc_live_path).unwrap();

					println!("admin port: {:?}", admin_port);

					let admin_port_clone = admin_port.clone();

					println!("trying to get app port");
					let app_port = tauri::async_runtime::block_on(async move { match get_app_websocket(admin_port_clone).await {
						Ok(ws) => ws,
						Err(e) => {
							println!("ERROR! Error getting app websocket port: {}", e);
							panic!("Failed to get app websocket port.");
						},
					} });

					println!("app port: {:?}", app_port);

					// extract the number of lines of the .hc file to know the number of sandboxes
					// read all the hc_live_X files to retrieve the admin ports



					let launcher_env = format!(r#"{{
						"APP_INTERFACE_PORT": {},
						"ADMIN_INTERFACE_PORT": {},
						"INSTALLED_APP_ID": "{}"
					}}"#,
					app_port,
					admin_port,
					app_id.clone(),
					);

					println!("Starting to build window.");

					let window = match utils::generate_window(
						&app.handle(),
						&app_id,
						app_id.clone(),
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

					println!("App window created.");
					windows.push(window);
					app_counter += 1;

				}




			Ok(())

		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");


}



async fn get_app_websocket(admin_port: String) ->  Result<u16, String> {
	// Try to connect twice. This fixes the os(111) error for now that occurs when the conducor is not ready yet.
	let mut ws = match AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
		.await
	{
		Ok(ws) => ws,
		Err(e) => return Err(format!("Could not connect to admin websocket: {:?}", e))
	};

	let app_interface_port = {
		let app_interfaces = ws.list_app_interfaces().await.or(Err(
			format!("Could not list app websocket interfaces."),
		))?;

		if app_interfaces.len() > 0 {
			app_interfaces[0]
		} else {
			let free_port = portpicker::pick_unused_port().expect("No ports free");

			ws.attach_app_interface(free_port).await.or(Err(
				format!("Could not attach app websocket interface."),
			))?;
			free_port
		}
	};

	Ok(app_interface_port)
}
