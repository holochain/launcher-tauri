use std::fs;
use std::path::PathBuf;


use holochain_types::web_app::WebAppBundle;
use lair_keystore_manager::utils::{path_exists, create_dir_if_necessary};


pub async fn read_and_prepare_webapp(web_happ_path: &PathBuf) -> Result<(), String> {

  // 1. read the .webhapp file
  let bytes = fs::read(web_happ_path)
    .map_err(|e| format!("Failed to read .webhapp file: {}", e))?;

  // decoding .webhapp file
  let web_app_bundle = WebAppBundle::decode(&bytes)
    .map_err(|e| format!("Failed to read webhapp bundle file: {}", e))?;

  // extracting happ bundle
  let app_bundle = web_app_bundle.happ_bundle().await
    .map_err(|e| format!("Failed to extract app bundle from file: {:?}", e))?;

  // extracting ui.zip bytes
  let web_ui_zip_bytes = web_app_bundle.web_ui_zip_bytes().await
    .map_err(|e| format!("Failed to extract ui zip bytes: {:?}", e))?;


  // TODO! Add to tmp directory instead
  // creating .hc_launch directory if necessary
  // 2. store the .happ and the unzipped UI assets in respective folders
  create_dir_if_necessary(&PathBuf::from(".hc_launch"))
    .map_err(|e| format!("Failed to create temporary directory .hc_launch: {:?}", e))?;


  let ui_folder_path = PathBuf::from(".hc_launch").join("ui");
  // remove existing assets first
  if path_exists(&ui_folder_path) {
    fs::remove_dir_all(ui_folder_path.clone()).unwrap();
  }

  fs::create_dir(ui_folder_path.clone())
    .map_err(|e| format!("Failed to create ui directory: {:?}", e))?;


  // writing ui.zip
  let ui_zip_path = ui_folder_path.join("ui.zip");
  fs::write(ui_zip_path.clone(), web_ui_zip_bytes.to_vec())
    .map_err(|e| format!("Error writing ui.zip: {:?}", e))?;

  // opening ui.zip
  let file = fs::File::open(ui_zip_path.clone())
    .map_err(|e| format!("Error opening ui.zip: {:?}", e))?;

  // Unzipping ui.zip
  unzip_file(file, ui_folder_path)
    .map_err(|e| format!("Could not unzip ui.zip: {:?}", e))?;

  // remove ui.zip after extraction
  fs::remove_file(ui_zip_path)
    .map_err(|e| format!("Failed to remove ui.zip: {:?}", e))?;

  // Writing .happ file
  app_bundle.write_to_file(&PathBuf::from(".hc_launch").join("happ.happ")).await
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

