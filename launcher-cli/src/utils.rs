use std::fs;
use std::path::PathBuf;
use std::process::{Stdio, Command};
use std::thread::JoinHandle;
use std::fs::OpenOptions;
use std::io::prelude::*;


use holochain_types::web_app::WebAppBundle;
use lair_keystore_manager::utils::{path_exists, create_dir_if_necessary};


pub fn spawn_app_instance(app_id: String) -> JoinHandle<()> {

  // write app_id to .hc_launcher file
  let maybe_file = OpenOptions::new()
    .write(true)
    .append(true)
    .open(".hc_launcher");

  match maybe_file {
    Ok(mut file) => {
      if let Err(e) = writeln!(file, "{}", app_id) {
        eprintln!("Couldn't write to file: {}", e);
      }
    },
    Err(_) => {
      println!("Creating new .hc_launcher file.");
      std::fs::write(".hc_launcher", format!("{}\n", app_id)).unwrap();
    }
  }



  // spawn hc sandbox thread
  std::thread::spawn(move ||  {
    let mut _sandbox_handle = Command::new("hc")
      .args(["s", "--piped", "generate", ".launcher-cli/happ.happ", "--run", "-a", app_id.as_str(), "network", "mdns"])
      .stdout(Stdio::inherit())
      .output()
      .expect("failed to execute process");

    // waiting for child (only when using .spawn())
    // sandbox_handle.wait().expect("failed to wait on sandbox child.");

    println!("sandbox_handle finished.");
  })
}


pub async fn read_and_prepare_webhapp(web_happ_path: &String) -> () {

  // 1. read the .webhapp file
  println!("Reading .webhapp file");
  let bytes = match fs::read(&web_happ_path) {
    Ok(bytes) => bytes,
    Err(e) => panic!("Failed to read .webhapp file: {:?}", e),
  };

  println!("decoding .webhapp file");
  let web_app_bundle = match WebAppBundle::decode(&bytes) {
    Ok(bundle) => bundle,
    Err(e) => panic!("Failed to read webhapp bundle file: {:?}", e)
  };

  println!("extracting happ bundle");
  let app_bundle = match web_app_bundle.happ_bundle().await {
    Ok(bundle) => bundle,
    Err(e) => panic!("Failed to extract app bundle from file: {:?}", e),
  };

  println!("extracting ui.zip bytes");
  let web_ui_zip_bytes = match web_app_bundle.web_ui_zip_bytes().await {
    Ok(bytes) => bytes,
    Err(e) => panic!("Failed to extract ui zip bytes: {:?}", e)
  };

  println!("creating .launcher-cli directory if necessary");
  // 2. store the .happ and the unzipped UI assets in respective folders
  match create_dir_if_necessary(&PathBuf::from(".launcher-cli")) {
    Ok(()) => (),
    Err(e) => panic!("Failed to create temporary directory .launcher-cli: {:?}", e),
  }

  println!("removing existing assets");
  let ui_folder_path = PathBuf::from(".launcher-cli").join("ui");
  // remove existing assets first
  if path_exists(&ui_folder_path) {
    fs::remove_dir_all(ui_folder_path.clone()).unwrap();
  }

  match fs::create_dir(ui_folder_path.clone()) {
    Ok(()) => (),
    Err(e) => panic!("Failed to create ui directory: {:?}", e),
  }

  println!("writing ui.zip");
  let ui_zip_path = ui_folder_path.join("ui.zip");
  match fs::write(ui_zip_path.clone(), web_ui_zip_bytes.to_vec()) {
    Ok(()) => (),
    Err(e) => panic!("Error writing ui.zip: {:?}", e),
  }

  println!("opening ui.zip");
  let file = match fs::File::open(ui_zip_path.clone()) {
    Ok(file) => file,
    Err(e) => panic!("Error opening ui.zip: {:?}", e),
  };

  println!("Unzipping ui.zip");
  match unzip_file(file, ui_folder_path) {
    Ok(()) => (),
    Err(e) => panic!("Could not unzip ui.zip: {:?}", e),
  }

  println!("Removing ui.zip");
  // remove ui.zip after extraction
  match fs::remove_file(ui_zip_path) {
    Ok(()) => (),
    Err(e) => panic!("Failed to remove ui.zip: {:?}", e),
  }

  println!("Writing .happ file");
  match app_bundle.write_to_file(&PathBuf::from(".launcher-cli").join("happ.happ")).await {
    Ok(()) => (),
    Err(e) => panic!("Failed to write .happ file: {:?}", e),
  }

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
