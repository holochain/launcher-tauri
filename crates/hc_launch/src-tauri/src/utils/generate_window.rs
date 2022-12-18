use tauri::{ WindowUrl, window::{ WindowBuilder, Window } };
use tauri::{CustomMenuItem, Menu, Submenu};
use tauri::Result;
use std::fs::read;
use std::path::PathBuf;


pub fn generate_window(
  app_handle: &tauri::AppHandle,
  _app_id: &String,
  label: String,
  index_path: PathBuf,
  assets_path: PathBuf,
  local_storage_path: PathBuf,
  launcher_env_command: String
) -> Result<Window> {

    WindowBuilder::new(
      app_handle,
      label.clone(),
      WindowUrl::App("index.html".into())
    )
    .on_web_resource_request(move |request, response| {

      let uri = request.uri();
      match uri {
        "tauri://localhost" => {
          let mutable_response = response.body_mut();
          match read(index_path.clone()) {
            Ok(index_html) => *mutable_response = index_html, // TODO! Check if there are better ways of dealing with errors here
            Err(e) => log::error!("Error reading the path of the UI's index.html: {:?}", e),
          }
        },
        _ => {
          if uri.starts_with("tauri://localhost/") {

            let mut asset_file = &uri[18..]; // TODO! proper error handling. index may be out of bounds?

            // if uri is exactly "tauri://localhost/" redirect to index.html
            // (otherwise it will try to redirect to the admin window's index.html)
            if asset_file == "" {
              asset_file = "index.html";
            }

            let mime_guess = mime_guess::from_path(asset_file);

            let mime_type = match mime_guess.first() {
              Some(mime) => Some(mime.essence_str().to_string()),
              None => {
                None
              }
            };

            let asset_path = assets_path.join(asset_file);
            match read(asset_path.clone()) {
              Ok(asset) => {
                let mutable_response = response.body_mut();
                *mutable_response = asset;
                response.set_mimetype(mime_type.clone());
                println!("\nRequested file: {}", asset_file);
                println!("Detected mime type: {:?}\n", mime_type);
              },
              Err(e) => {
                println!("### ERROR ### Error reading asset file from path '{:?}'. Redirecting to 'index.html'. Error: {:?}.\nThis may be expected in case of push state routing.", asset_path, e);
                let mutable_response = response.body_mut();
                match read(index_path.clone()) {
                  Ok(index_html) =>  {
                    *mutable_response = index_html;
                    response.set_mimetype(Some(String::from("text/html")));
                  },
                  Err(e) => {
                    println!("### ERROR ### Error reading the path of the UI's index.html: {:?}\n", e);
                  },
                }
              },
            }
          }
        }
      }


    })
    .disable_file_drop_handler()
    .data_directory(local_storage_path)
    .initialization_script(launcher_env_command.as_str())
    .inner_size(1000.0, 700.0)
    .title(label)
    .menu(Menu::new().add_submenu(Submenu::new(
      "Settings",
      Menu::new().add_item(CustomMenuItem::new("show-devtools", "Show DevTools")),
    )))
    .build()

}