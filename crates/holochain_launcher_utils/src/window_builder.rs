use tauri::{ WindowUrl, window::WindowBuilder };
use std::fs::read;
use std::path::PathBuf;


pub enum UISource {
  Path(PathBuf),
  Port(u16),
}

impl Clone for UISource{
  fn clone(&self) -> Self {
    match self {
      UISource::Path(p) => UISource::Path(p.clone()),
      UISource::Port(p) => UISource::Port(p.clone()),
    }
  }
}

/// Creates a tauri WindowBuilder object with all the methods shared between
/// holochain launcher and hc_launch already applied to it.
pub fn happ_window_builder<'a>(
  app_handle: &'a tauri::AppHandle,
  app_id: String,
  window_label: String, // label used by tauri internally to distinguish different windows
  window_title: String, // label shown on the top bar of the window
  ui_source: UISource, // source to the UI
  local_storage_path: PathBuf,
  app_port: u16,
  admin_port: u16,
  show_404: bool, // whether to show a 404 message (true) if the index.html cannot be found or default to tauri's index.html (false)
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
  // openig in system default browser. For macOS additionaly display a message when data is being
  // attempted to be downloaded via an anchor tag
  #[cfg(target_os = "macos")]
  let anchor_event_listener = r#"window.addEventListener("click", (e) => {
    const maybeHref = e.composedPath()[0].href;

    if (maybeHref) {
      // alert(`Got composed path with href: ${maybeHref}`);
      if ( (maybeHref.startsWith('http://') || maybeHref.startsWith('https://')) && !(maybeHref.includes("tauri.localhost")) ) {
        e.preventDefault();
        window.__TAURI_INVOKE__('open_url_cmd', { 'url': maybeHref } )
      }

      if (maybeHref.startsWith('data:')) {
        e.preventDefault();
        alert("We use Tauri to securely display Holochain apps. For macOS, downloading files is currently not supported. For more information, visit https://github.com/tauri-apps/tauri/issues/4633");
      }
    }
  });
  "#;

  #[cfg(not(target_os = "macos"))]
  let anchor_event_listener = r#"window.addEventListener("click", (e) => {
    const maybeHref = e.composedPath()[0].href;

    if (maybeHref) {
      // alert(`Got composed path with href: ${maybeHref}`);
      if ( (maybeHref.startsWith('http://') || maybeHref.startsWith('https://')) && !(maybeHref.includes("tauri.localhost")) ) {
        e.preventDefault();
        window.__TAURI_INVOKE__('open_url_cmd', { 'url': maybeHref } )
      }
    }
  });
  "#;

  let zoom_on_scroll = r#"
    // Adding event listeners to adjust zoom level on Ctrl + scroll
    function increaseZoomLevel(amount) {
      const percentageString = document.body.style.zoom;
      let num = percentageString === "" ? 100 : parseInt(percentageString.slice(0, percentageString.length-1));
      let newVal = num + Math.round(amount) < 500 ? num + Math.round(amount) : 500;
      document.body.style.zoom = `${newVal}%`
    }
    function decreaseZoomLevel(amount) {
      const percentageString = document.body.style.zoom;
      let num = percentageString === "" ? 100 : parseInt(percentageString.slice(0, percentageString.length-1));
      let newVal = num - Math.round(amount) > 30 ? num - Math.round(amount) : 30;
      document.body.style.zoom = `${newVal}%`
    }
    window.onkeydown = (ev) => {
      if (ev.key === "Control") {
        window.onwheel = (ev) => {
          if (ev.deltaY > 0) {
            decreaseZoomLevel(10);
          } else if (ev.deltaY < 0) {
            increaseZoomLevel(10);
          }
        }
      }
    };
    window.onkeyup = (ev) => {
      if (ev.key === "Control") {
        window.onwheel = null;
      }
    }
  "#;

  let message_404 = r#"
  <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100vh;">
    <h1>404 Not Found.</h1>
    <h3>Looks like this UI has no index.html</h3>
  </div>
  "#.as_bytes().to_vec();

  let url = match ui_source.clone() {
    UISource::Path(_path) => WindowUrl::App("".into()),
    UISource::Port(port) => WindowUrl::External(format!("http://localhost:{}", port).parse().unwrap()),
  };

  let mut window_builder = WindowBuilder::new(
    app_handle,
    window_label.clone(),
    url
  );

  // In the "real" launcher case, i.e. not served via localhost:
  if let UISource::Path(assets_path) = ui_source {
    window_builder = window_builder.on_web_resource_request(move |request, response| {
      let uri = request.uri();
      let index_path = assets_path.join("index.html");
      match uri {
        "tauri://localhost" => {
          let mutable_response = response.body_mut();
          match read(index_path.clone()) {
            Ok(index_html) => {
              *mutable_response = index_html;
              response.set_mimetype(Some(String::from("text/html")));
            }, // TODO! Check if there are better ways of dealing with errors here
            Err(e) => {
              if show_404 {
                *mutable_response = message_404.clone();
                response.set_mimetype(Some(String::from("text/html")));
              }
              log::error!("Error reading the path of the UI's index.html: {:?}", e);
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
              Err(_e) => {
                // println!("\n### ERROR ### Error reading asset file from path '{:?}'. Redirecting to 'index.html'. Error: {:?}.\nThis may be expected in case of push state routing.\n", asset_path, e);
                let mutable_response = response.body_mut();
                match read(index_path.clone()) {
                  Ok(index_html) =>  {
                    *mutable_response = index_html;
                    response.set_mimetype(Some(String::from("text/html")));
                  },
                  Err(e) => {
                    if show_404 {
                      *mutable_response = message_404.clone();
                      response.set_mimetype(Some(String::from("text/html")));
                    }
                    log::error!("Error reading the path of the UI's index.html: {:?}", e);
                  },
                }
              },
            }
          }
        }
      }
    });
  }

  window_builder
    .disable_file_drop_handler()
    .data_directory(local_storage_path)
    .initialization_script(launcher_env_command.as_str())
    .initialization_script(anchor_event_listener)
    .initialization_script(zoom_on_scroll)
    .title(window_title)

}




