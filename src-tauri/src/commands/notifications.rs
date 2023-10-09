use futures::lock::Mutex;
use holochain_client::InstalledAppId;
use holochain_web_app_manager::derive_window_label;
use serde::{Deserialize, Serialize};
use tauri::{api::notification::Notification, Manager, Icon};

pub type NotificationId = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HappNotification {
    title: String,
    body: String,
    notification_type: String,
    icon_file_name: Option<String>,
    urgency: String,
    timestamp: u64,
    custom_count_reset: Option<NotificationId>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotificationPayload {
    notifications: Vec<HappNotification>,
    app_id: InstalledAppId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IconState {
    Clean,
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysTrayIconState {
    pub icon_state: IconState
}

impl SysTrayIconState {
    pub fn get_icon_state(&self) -> IconState {
        self.icon_state.clone()
    }
}


#[tauri::command]
pub async fn notify_tauri(
    // window: tauri::Window,
    app_handle: tauri::AppHandle,
    notifications: Vec<HappNotification>,
    app_id: InstalledAppId,
) -> tauri::Result<()> {
    // This tauri command is allowed for any window.

    // Send notifications to admin window to store to localStorage and check
    // OS notification settings for this app.
    if let Some(admin_window) = app_handle.get_window("admin") {
        admin_window.emit("happ-notifications", NotificationPayload {
            notifications,
            app_id,
        })
    } else {
        // The admin window must always be running invisibly in the background
        // so this case should not occur
        Ok(())
    }
}


#[tauri::command]
pub async fn notify_os(
    window: tauri::Window,
    app_handle: tauri::AppHandle,
    // profile: tauri::State<'_, Profile>,
    notifications: Vec<HappNotification>,
    app_id: InstalledAppId,
    systray: bool,
    os: bool,
) -> Result<(), String> {
    if window.label() != "admin" {
        return Err(String::from("Unauthorized: Attempted to call tauri command 'notify_os' which is not allowed in this window."))
    }

    // Only notify the OS if the happ's window is not focused,
    let happ_window_focused = match app_handle.get_window(derive_window_label(&app_id).as_str()) {
        Some(happ_window) => happ_window.is_focused()
            .map_err(|e| format!("Failed to get focus state of happ window: {}", e))?,
        None => false,
    };
    let admin_window_focused = window.is_focused()
        .map_err(|e| format!("Failed to get focus state of admin window: {}", e))?;

    // if both happ window and admin window are not focused, change the system tray icon state and send an os notification
    if !admin_window_focused && !happ_window_focused {
        for message in notifications {
            if systray {
                change_systray_icon_state(&app_handle, &message.urgency).await
                    .map_err(|e| format!("Failed to change systray icon state: {}", e))?;
            }
            if os && message.urgency == "high" {
                let os_notification =  Notification::new(&app_handle.config().tauri.bundle.identifier)
                    .body(message.body)
                    .title(format!("{}: {}", app_id, message.title));

                // TODO add icon by deriving [app assets dir].join("icons").join(message.file_name)
                // --> probably requires webhapp manager for the correct holochain ID (web_happ_manager.get_app_assets_dir())
                // if let Some(file_name) = message.icon_file_name {
                //     let file_path =
                // }

                os_notification.show()
                    .map_err(|e| format!("Failed to to show OS notification: {}", e))?;

            }
        }
    }

    Ok(())
}


#[tauri::command]
pub async fn clear_happ_notifications(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    app_id: InstalledAppId
) -> Result<(), String> {
    // This tauri command is allowed to be called only by the window of the corresponding app:
    if window.label() != derive_window_label(&app_id) {
        return Err(String::from("Unauthorized: Attempted to notifications for app that this tauri window is not associated to."))
    }

    // Send notifications to admin window to store to localStorage and check
    // OS notification settings for this app.
    if let Some(admin_window) = app_handle.get_window("admin") {
        admin_window.emit("clear-happ-notifications", app_id)
            .map_err(|e| format!("Failed to emit event to admin window: {}", e))
    } else {
        // The admin window must always be running invisibly in the background
        // so this case should not occur
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ResetHappNotificationPayload {
    app_id: InstalledAppId,
    notification_ids: Vec<NotificationId>,
}

#[tauri::command]
pub async fn reset_happ_notification_count(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    app_id: InstalledAppId,
    notification_ids: Vec<NotificationId>,
) -> Result<(), String> {
    // This tauri command is allowed to be called only by the window of the corresponding app:
    if window.label() != derive_window_label(&app_id) {
        return Err(String::from("Unauthorized: Attempted to notifications for app that this tauri window is not associated to."))
    }

    // Send notifications to admin window to store to localStorage and check
    // OS notification settings for this app.
    if let Some(admin_window) = app_handle.get_window("admin") {
        admin_window.emit("reset-happ-notification-count", ResetHappNotificationPayload { app_id, notification_ids })
            .map_err(|e| format!("Failed to emit event to admin window: {}", e))
    } else {
        // The admin window must always be running invisibly in the background
        // so this case should not occur
        Ok(())
    }
}


async fn change_systray_icon_state(
  app_handle: &tauri::AppHandle,
  urgency: &String,
) -> tauri::Result<()> {

    let mutex = app_handle.state::<Mutex<SysTrayIconState>>();

    match urgency.as_str() {
        "low" => (),
        "medium" => {
            let systray_icon_state = mutex.lock().await.get_icon_state();
            match systray_icon_state {
                IconState::Clean | IconState::Low => {
                    let icon_path_option = app_handle.path_resolver().resolve_resource("icons/icon_priority_medium_32x32.png");
                    if let Some(icon_path) = icon_path_option {
                        app_handle.tray_handle().set_icon(Icon::File(icon_path))?;
                    }
                    *mutex.lock().await = SysTrayIconState { icon_state: IconState::Medium };
                },
                _ => (),
            }
        },
        "high" => {
            let icon_path_option = app_handle.path_resolver().resolve_resource("icons/icon_priority_high_32x32.png");
            if let Some(icon_path) = icon_path_option {
                app_handle.tray_handle().set_icon(Icon::File(icon_path))?;
            }
            *mutex.lock().await = SysTrayIconState { icon_state: IconState::High };
        },
        _ => log::error!("Got invalid notification urgency level: {}", urgency),
    }

    Ok(())
}

#[tauri::command]
pub async fn clear_systray_icon(
    app_handle: tauri::AppHandle,
  ) -> tauri::Result<()> {
    let mutex = app_handle.state::<Mutex<SysTrayIconState>>();
    let icon_path_option = app_handle.path_resolver().resolve_resource("icons/32x32.png");
    if let Some(icon_path) = icon_path_option {
        app_handle.tray_handle().set_icon(Icon::File(icon_path))?;
    }
    *mutex.lock().await = SysTrayIconState { icon_state: IconState::Clean };
    Ok(())
  }




