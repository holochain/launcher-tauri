use super::port_mapping::app_ui_folder_path;
use crate::{
  config::uis_data_path,
  uis::{activate::activate_app_ui, port_mapping::PortMapping},
};
use std::{
  fs::{self, File},
  io,
  path::PathBuf,
};

#[tauri::command]
pub fn install_ui(app_id: String, base64_bytes: String) -> Result<u16, String> {
  let mut port_mapping = PortMapping::read_port_mapping()?;

  if let Some(_) = port_mapping.get_ui_port_for_app(&app_id) {
    return Err(String::from("App is already installed"));
  }

  let bytes = base64::decode(base64_bytes).or(Err("Failed to decode base64"))?;

  let ui_folder_path = app_ui_folder_path(app_id.clone());
  let ui_zip_path = uis_data_path().join(format!("{}.zip", app_id));

  fs::write(ui_zip_path.clone(), bytes).or(Err("Could not write the UI file"))?;

  unzip_file(
    File::open(ui_zip_path).or(Err("Failed to read file"))?,
    ui_folder_path,
  )?;

  let port = port_mapping.set_available_ui_port_for_app(app_id.clone())?;

  activate_app_ui(app_id, port)?;

  Ok(port)
}

fn unzip_file(file: File, outpath: PathBuf) -> Result<(), String> {
  let mut archive = zip::ZipArchive::new(file).unwrap();

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    let outpath = match file.enclosed_name() {
      Some(path) => outpath.join(path).to_owned(),
      None => continue,
    };

    {
      let comment = file.comment();
      if !comment.is_empty() {
        println!("File {} comment: {}", i, comment);
      }
    }

    if (&*file.name()).ends_with('/') {
      println!("File {} extracted to \"{}\"", i, outpath.display());
      fs::create_dir_all(&outpath).unwrap();
    } else {
      println!(
        "File {} extracted to \"{}\" ({} bytes)",
        i,
        outpath.display(),
        file.size()
      );
      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(&p).unwrap();
        }
      }
      let mut outfile = fs::File::create(&outpath).unwrap();
      io::copy(&mut file, &mut outfile).unwrap();
    }
  }

  Ok(())
}
