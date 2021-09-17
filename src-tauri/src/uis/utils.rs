use std::{
  fs::{self, File},
  io,
  path::PathBuf,
};

pub fn unzip_file(reader: File, outpath: PathBuf) -> Result<(), String> {
  let mut archive = zip::ZipArchive::new(reader).unwrap();

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    let outpath = match file.enclosed_name() {
      Some(path) => outpath.join(path).to_owned(),
      None => continue,
    };

    {
      let comment = file.comment();
      if !comment.is_empty() {
        log::trace!("File {} comment: {}", i, comment);
      }
    }

    if (&*file.name()).ends_with('/') {
      log::trace!("File {} extracted to \"{}\"", i, outpath.display());
      fs::create_dir_all(&outpath).unwrap();
    } else {
      log::trace!(
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
