use std::fs;
use std::path::PathBuf;
use std::process::{Stdio, Command};
use std::thread::JoinHandle;
use std::fs::OpenOptions;
use std::io::prelude::*;


use holochain_types::web_app::WebAppBundle;
use lair_keystore_manager::utils::{path_exists, create_dir_if_necessary};

/// Spawns an app instance in a new conductor, i.e. for a new agent.
pub fn spawn_agent_app_instance(
  app_id: String, // app id
  sandbox_identifier: Option<String>, // if provided, the sandbox gets the name [sandbox_identifier]_[app_id]
  log_app_id: bool, // whether to append the app id to a .hc_launch file
) -> JoinHandle<()> {

  if log_app_id == true {
    // write app_id to .hc_launch file
    let maybe_file = OpenOptions::new()
      .write(true)
      .append(true)
      .open(".hc_launch");

    match maybe_file {
      Ok(mut file) => {
        if let Err(e) = writeln!(file, "{}", app_id) {
          eprintln!("Couldn't write to file: {}", e);
        }
      },
      Err(_) => {
        println!("Creating new .hc_launcher file.");
        std::fs::write(".hc_launch", format!("{}\n", app_id)).unwrap();
      }
    }
  }



  // spawn hc sandbox thread
  match sandbox_identifier {
    Some(id) => {
      std::thread::spawn(move ||  {
        let mut _sandbox_handle = Command::new("hc")
          .args(["s", "--piped", "generate", ".launcher-cli/happ.happ", "--run", "-a", app_id.as_str(), "-d", format!("{}_{}", id, app_id).as_str(),"network", "mdns"])
          .stdout(Stdio::inherit())
          .output()
          .expect("failed to execute process");
      })
    },
    None => {
      std::thread::spawn(move ||  {
        let mut _sandbox_handle = Command::new("hc")
          .args(["s", "--piped", "generate", ".launcher-cli/happ.happ", "--run", "-a", app_id.as_str(),"network", "mdns"])
          .stdout(Stdio::inherit())
          .output()
          .expect("failed to execute process");
      })
    }
  }

}


pub async fn read_and_prepare_webhapp(web_happ_path: PathBuf) -> Result<(), String> {

  // 1. read the .webhapp file
  println!("Reading .webhapp file");
  let bytes = fs::read(web_happ_path)
    .map_err(|e| format!("Failed to read .webhapp file: {}", e))?;

  println!("decoding .webhapp file");
  let web_app_bundle = WebAppBundle::decode(&bytes)
    .map_err(|e| format!("Failed to read webhapp bundle file: {}", e))?;

  println!("extracting happ bundle");
  let app_bundle = web_app_bundle.happ_bundle().await
    .map_err(|e| format!("Failed to extract app bundle from file: {:?}", e))?;

  println!("extracting ui.zip bytes");
  let web_ui_zip_bytes = web_app_bundle.web_ui_zip_bytes().await
    .map_err(|e| format!("Failed to extract ui zip bytes: {:?}", e))?;


  println!("creating .launcher-cli directory if necessary");
  // 2. store the .happ and the unzipped UI assets in respective folders
  create_dir_if_necessary(&PathBuf::from(".launcher-cli"))
    .map_err(|e| format!("Failed to create temporary directory .launcher-cli: {:?}", e))?;


  println!("removing existing assets");
  let ui_folder_path = PathBuf::from(".launcher-cli").join("ui");
  // remove existing assets first
  if path_exists(&ui_folder_path) {
    fs::remove_dir_all(ui_folder_path.clone()).unwrap();
  }

  fs::create_dir(ui_folder_path.clone())
    .map_err(|e| format!("Failed to create ui directory: {:?}", e))?;


  println!("writing ui.zip");
  let ui_zip_path = ui_folder_path.join("ui.zip");
  fs::write(ui_zip_path.clone(), web_ui_zip_bytes.to_vec())
    .map_err(|e| format!("Error writing ui.zip: {:?}", e))?;

  println!("opening ui.zip");
  let file = fs::File::open(ui_zip_path.clone())
    .map_err(|e| format!("Error opening ui.zip: {:?}", e))?;

  println!("Unzipping ui.zip");
  unzip_file(file, ui_folder_path)
    .map_err(|e| format!("Could not unzip ui.zip: {:?}", e))?;


  println!("Removing ui.zip");
  // remove ui.zip after extraction
  fs::remove_file(ui_zip_path)
    .map_err(|e| format!("Failed to remove ui.zip: {:?}", e))?;

  println!("Writing .happ file");
  app_bundle.write_to_file(&PathBuf::from(".launcher-cli").join("happ.happ")).await
    .map_err(|e| format!("Failed to write .happ file: {:?}", e))?;


  Ok(())

}



pub fn unzip_file(reader: fs::File, outpath: PathBuf) -> Result<(), String> {
  let mut archive = zip::ZipArchive::new(reader).unwrap();

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    let outpath = match file.enclosed_name() {
      Some(path) => outpath.join(path).to_owned(),
      None => continue,
    };

    if (&*file.name()).ends_with('/') {
      fs::create_dir_all(&outpath).unwrap();
    } else {
      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(&p).unwrap();
        }
      }
      let mut outfile = fs::File::create(&outpath).unwrap();
      std::io::copy(&mut file, &mut outfile).unwrap();
    }
  }

  Ok(())
}
