use log::LevelFilter;
use log4rs::{
  self,
  append::file::FileAppender,
  config::{Appender, Root},
  encode::pattern::PatternEncoder,
  Config,
};
use tauri::{AppHandle, Manager};
use tauri::api::shell::open;

use crate::file_system::{logs_folder_path, logs_path};

pub fn setup_logs(custom_path: Option<String>) -> Result<(), String> {
  let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("[{d}] {l} - {m}\n")))
    .build(logs_path(custom_path))
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

pub fn open_logs_folder(app: &AppHandle, custom_path: Option<String>) {

  let logs_path = logs_folder_path(custom_path);

  match logs_path.as_os_str().to_str() {
    Some(logs_path_str) => {
      if let Err(err) = open(
        &app.shell_scope(),
        logs_path_str,
        None
        ){
        log::error!("Error opening logs folder: {}", err);
      }
    },
    None => log::error!("Failed to convert logs path from OsStr to str")
  }
}
