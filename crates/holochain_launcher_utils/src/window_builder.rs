use tauri::{ WindowUrl, window::WindowBuilder };
use std::fs::read;
use std::path::PathBuf;


/// Creates a tauri WindowBuilder object with all the methods shared between
/// holochain launcher and hc_launch already applied to it.
pub fn happ_window_builder<'a>(
  app_handle: &'a tauri::AppHandle,
  app_id: String,
  window_label: String, // label used by tauri internally to distinguish different windows
  window_title: String, // label shown on the top bar of the window
  index_path: PathBuf, // path to the index.html of the happ
  assets_path: PathBuf, // path
  local_storage_path: PathBuf,
  app_port: u16,
  admin_port: u16,
  width: f64,
  height: f64,
) -> WindowBuilder<'a> {

  let launcher_env_command = format!(r#"window.__HC_LAUNCHER_ENV__ = {{
    "APP_INTERFACE_PORT": {},
    "ADMIN_INTERFACE_PORT": {},
    "INSTALLED_APP_ID": "{}"
  }}"#,
    app_port,
    admin_port,
    app_id
  );

  // listen for anchor clicks to route them to the open_url_cmd command for sanitization and
  // opennig in system default browser
  let anchor_event_listener = r#"window.addEventListener("click", (e) => {
    if ((e.target.tagName.toLowerCase() === 'a') && (e.target.href.startsWith('http://') || e.target.href.startsWith('https://'))) {
      e.preventDefault();
      window.__TAURI_INVOKE__('open_url_cmd', { 'url': e.target.href } )
    }
  });
  "#;

  WindowBuilder::new(
    app_handle,
    window_label.clone(),
    WindowUrl::App("".into())
  )
  .on_web_resource_request(move |request, response| {
    let uri = request.uri();
    match uri {
      "tauri://localhost" => {
        let mutable_response = response.body_mut();
        match read(index_path.clone()) {
          Ok(index_html) => {
            *mutable_response = index_html;
            response.set_mimetype(Some(String::from("text/html")));
          }, // TODO! Check if there are better ways of dealing with errors here
          Err(_e) => {
            ()
            // println!("\n### ERROR ### Error reading the path of the UI's index.html: {:?}\n", e);
            // log::error!("Error reading the path of the UI's index.html: {:?}", e);
          },
        }
      },
      _ => {
        if uri.starts_with("tauri://localhost/") {

          let mut asset_file = &uri[18..]; // TODO! error handling: can index be out of bounds?

          // if uri is exactly "tauri://localhost/" redirect to index.html (otherwise it will try to redirect to the admin window's index.html)
          if asset_file == "" {
            asset_file = "index.html";
          }

          let mime_guess = mime_guess::from_path(asset_file);

          let mime_type = match mime_guess.first() {
            Some(mime) => Some(mime.essence_str().to_string()),
            None => {
              log::info!("Could not determine MIME Type of file '{:?}'", asset_file);
              // println!("\n### ERROR ### Could not determine MIME Type of file '{:?}'\n", asset_file);
              None
            }
          };

          // TODO! if files in subfolders are requested, additional logic may be required here to get paths right across platforms
          let asset_path = assets_path.join(asset_file);

          match read(asset_path.clone()) {
            Ok(asset) => {
              let mutable_response = response.body_mut();
              *mutable_response = asset;
              response.set_mimetype(mime_type.clone());
              // println!("\nRequested file: {}", asset_file);
              // println!("Detected mime type: {:?}\n", mime_type);
            },
            Err(e) => {
              // println!("\n### ERROR ### Error reading asset file from path '{:?}'. Redirecting to 'index.html'. Error: {:?}.\nThis may be expected in case of push state routing.\n", asset_path, e);
              let mutable_response = response.body_mut();
              match read(index_path.clone()) {
                Ok(index_html) =>  {
                  *mutable_response = index_html;
                  response.set_mimetype(Some(String::from("text/html")));
                },
                Err(e) => {
                  // println!("\n### ERROR ### Error reading the path of the UI's index.html: {:?}\n", e);
                  // log::error!("Error reading the path of the UI's index.html: {:?}", e);
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
  .initialization_script(anchor_event_listener)
  .inner_size(width, height)
  .title(window_title)

}






