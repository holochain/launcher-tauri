import { invoke } from "@tauri-apps/api";
import { HappNotification, NotificationId } from "@holochain/launcher-api";
declare global {
  interface Window {
    __HC_LAUNCHER_API__: {
      notify: (notifications: Array<HappNotification>) => void;
      resetNotificationCount: (notificationIds: Array<NotificationId>) => void;
    };
    __HC_LAUNCHER_ENV__: {
      APP_INTERFACE_PORT: number;
      ADMIN_INTERFACE_PORT: number;
      INSTALLED_APP_ID: string;
    };
  }
}

window.__HC_LAUNCHER_API__ = {
  notify: async (notifications: Array<HappNotification>) => {
    // store notifications to unread notifications of the app
    const appId = window.__HC_LAUNCHER_ENV__.INSTALLED_APP_ID;
    try {
      await invoke("notify_tauri", { notifications, appId });
    } catch (e) {
      console.error("Failed to invoke tauri command 'notify_tauri': ", e);
    }
  },
  resetNotificationCount: async (notificationIds: Array<NotificationId>) => {
    const appId = window.__HC_LAUNCHER_ENV__.INSTALLED_APP_ID;
    try {
      await invoke("reset_happ_notification_count", { appId, notificationIds });
    } catch (e) {
      console.error(
        "Failed to invoke tauri command 'reset_happ_notification_count': ",
        e
      );
    }
  },
};

window.addEventListener("focus", async () => {
  await invoke("clear_happ_notifications", {
    appId: window.__HC_LAUNCHER_ENV__.INSTALLED_APP_ID,
  });
});
