// use crate::launcher::{state::LauncherState, manager::HolochainId};

// #[tauri::command]
// pub async fn start_app(
//   window: tauri::Window,
//   state: tauri::State<'_, LauncherState>,
//   app_id: String,
//   holochain_id: HolochainId,
// ) -> Result<(), String> {
//   if window.label() != "admin" {
//     return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (O)"))
//   }

//   let mut mutex = (*state).lock().await;
//   let manager = mutex.get_running()?;

//   manager
//     .get_web_happ_manager(holochain_id)?
//     .start_app(app_id)
//     .await?;

//   manager.on_apps_changed().await?;

//   Ok(())
// }
