use log::LevelFilter;
use log4rs::{
  self,
  append::file::FileAppender,
  config::{Appender, Root},
  encode::pattern::PatternEncoder,
  Config,
};

use crate::setup::config::logs_path;

use super::config::{logs_folder_path};

pub fn setup_logs() -> Result<(), String> {
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("[{d}] {l} - {m}\n")))
    .build(logs_path())
    .map_err(|err| format!("Could not build log config: {:?}", err))?;

  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder().appender("logfile").build(LevelFilter::Info))
    .map_err(|err| format!("Could not init log config: {:?}", err))?;

  log4rs::init_config(config).map_err(|err| format!("Could not init log config: {:?}", err))?;

  Ok(())
}

#[tauri::command]
pub fn log(log: String) -> Result<(), String> {
  log::info!("[UI] {}", log);
  Ok(())
}

pub fn open_logs_folder() {
  if let Err(err) = opener::open(logs_folder_path()) {
    log::error!("Error opening logs folder: {}", err);
  }
}
