use log;
use std::{collections::HashMap, path::PathBuf, sync::Arc, sync::Mutex};
use tauri::api::process::{Command, CommandChild, CommandEvent};

use lair_keystore_manager::error::LaunchChildError;

use crate::{error::LaunchHolochainError, versions::HolochainVersion};

pub async fn launch_holochain_process(
  log_level: log::Level,
  version: HolochainVersion,
  command: Command,
  conductor_config_path: PathBuf,
  password: String,
) -> Result<CommandChild, LaunchHolochainError> {
  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));
  envs.insert(String::from("WASM_LOG"), String::from(log_level.as_str()));

  let (mut holochain_rx, mut holochain_child) = command
    .args(&[
      "-c",
      conductor_config_path.into_os_string().to_str().unwrap(),
      "-p",
    ])
    .envs(envs)
    .spawn()
    .map_err(|err| {
      LaunchHolochainError::LaunchChildError(LaunchChildError::FailedToExecute(format!("{}", err)))
    })?;



  let db_error = Arc::new(Mutex::new(false));
  let db_err_clone = Arc::clone(&db_error);

  let cmd_handle = tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = holochain_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[HOLOCHAIN v{}] {}", version, line),
        CommandEvent::Stderr(line) => { match line.contains("DatabaseError(SqliteError(SqliteFailure(Error { code: NotADatabase, extended_code: 26 }, Some(\"file is not a database\"))))") {
          true => {
            log::info!("[HOLOCHAIN v{}] {}", version, line);
            let mut db_err = db_err_clone.lock().unwrap();
            *db_err = true;
            // err_tx.send(true).unwrap();
            },
          false => log::info!("[HOLOCHAIN v{}] {}", version, line),
          }
        },
        _ => log::info!("[HOLOCHAIN v{}] {:?}", version, event),
      };
    };
  });
  log::info!("Launched holochain");


  holochain_child
    .write(password.as_bytes())
    .map_err(|err| LaunchHolochainError::ErrorWritingPassword(format!("{:?}", err)))?;
  holochain_child
    .write("\n".as_bytes())
    .map_err(|err| LaunchHolochainError::ErrorWritingPassword(format!("{:?}", err)))?;

  cmd_handle.await.unwrap();
  let db_err = *db_error.lock().unwrap();
  match db_err {
    true => {
      Err(LaunchHolochainError::SqliteError(String::from("Database file is not of the correct type.")))
    },
    false => {
      Ok(holochain_child)
    }
  }

}
