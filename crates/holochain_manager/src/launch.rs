use log;
use std::{collections::HashMap, path::PathBuf, sync::Arc, sync::Mutex};
use tauri::api::process::{Command, CommandChild, CommandEvent};

use lair_keystore_manager::error::LaunchChildError;

use crate::{error::LaunchHolochainError, versions::HolochainVersion};


enum LaunchHolochainProcessState {
  Pending,
  DatabaseFileTypeError,
  Success,
}

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


  let launch_state = Arc::new(Mutex::new(LaunchHolochainProcessState::Pending));
  let launch_state_clone = Arc::clone(&launch_state);




  let stdout_handle = tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = holochain_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => {
          log::info!("[HOLOCHAIN v{}] {}", version, line);
          if line == String::from("Conductor ready.") {
            let mut launch_state_ref = launch_state_clone.lock().unwrap();
            *launch_state_ref = LaunchHolochainProcessState::Success;
            break;
          }
        },
        CommandEvent::Stderr(line) => {
          match line.contains("DatabaseError(SqliteError(SqliteFailure(Error { code: NotADatabase, extended_code: 26 }, Some(\"file is not a database\"))))") {
            true => {
              log::info!("[HOLOCHAIN v{}] {}", version, line);
              let mut launch_state_ref = launch_state_clone.lock().unwrap();
              *launch_state_ref = LaunchHolochainProcessState::DatabaseFileTypeError;
              break;
              },
            false => {
              log::info!("[HOLOCHAIN v{}] {}", version, line);
            },
          }
        },
        _ => {
          log::info!("[HOLOCHAIN v{}] {:?}", version, event);
        },
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


  // await for the stdout_handle to exit either with success or error
  stdout_handle.await.unwrap();

  let launch_state_ref = launch_state.lock().unwrap();

  match *launch_state_ref {
    LaunchHolochainProcessState::Success => {
      log::info!("LaunchHolochainProcessState::Success");
      Ok(holochain_child)
    },
    LaunchHolochainProcessState::DatabaseFileTypeError => {
      log::info!("LaunchHolochainProcessState::DatabaseFileTypeError");
      Err(LaunchHolochainError::SqliteError(String::from("Database file is not of the correct type.")))
    },
    LaunchHolochainProcessState::Pending => {
      log::info!("LaunchHolochainProcessState::Pending");
      Err(LaunchHolochainError::ImpossibleError("LaunchHolochainProcessState still pending after launching the holochain process.".into()))
    }
  }

}

