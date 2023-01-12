use log::LevelFilter;
use log4rs::{
  self,
  append::file::FileAppender,
  config::{Appender, Root},
  encode::pattern::PatternEncoder,
  Config,
};

use crate::file_system::{profile_logs_dir, profile_logs_path, Profile};

pub fn setup_logs(profile: Profile) -> Result<(), String> {
  let logs_path = profile_logs_path(profile)
    .map_err(|e| format!("Failed to get path to profile's log file: {}", e))?;

  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("[{d}] {l} - {m}\n")))
    .build(logs_path)
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
  log::info!("[Launcher UI] {}", log);
  Ok(())
}


/// Opens the folder where the logs are stored for the given profile
pub fn open_logs_folder(profile: Profile) -> () {

  match profile_logs_dir(profile) {
    Ok(dir) => {
      if let Err(err) = opener::open(dir) {
        log::error!("Error opening logs folder: {}", err);
      }
    },
    Err(e) => {
      log::error!("Error opening logs folder: {}", e);
    }
  };
}
