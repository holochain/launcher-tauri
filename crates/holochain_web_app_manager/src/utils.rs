use std::{
  fs::{self, File},
  io,
  path::PathBuf,
};

pub fn unzip_file(reader: File, outpath: PathBuf) -> Result<(), String> {
  let mut archive = match zip::ZipArchive::new(reader) {
    Ok(a) => a,
    Err(e) => return Err(format!("Failed to unpack zip archive: {}", e)),
  };

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
      io::copy(&mut file, &mut outfile).unwrap();
    }
  }

  Ok(())
}
