use std::{fs, path::{PathBuf, Path}};

use crate::error::FileSystemError;

pub fn create_dir_if_necessary(path: &PathBuf) -> Result<(), FileSystemError> {
  if !path_exists(path) {
    fs::create_dir(path)?;
  }

  Ok(())
}

pub fn path_exists(path: &PathBuf) -> bool {
  Path::new(path).exists()
}
