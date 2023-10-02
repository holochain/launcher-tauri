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

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const invokeTauriCommand = async (command: string, params: any) => {
  try {
    await invoke(command, params);
  } catch (e) {
    console.error(`Failed to invoke tauri command '${command}': ${e}`);
  }
};

window.__HC_LAUNCHER_API__ = {
  notify: async (notifications: Array<HappNotification>) => {
    // store notifications to unread notifications of the app
    const appId = window.__HC_LAUNCHER_ENV__.INSTALLED_APP_ID;
    await invokeTauriCommand("notify_tauri", { notifications, appId });
  },
  resetNotificationCount: async (notificationIds: Array<NotificationId>) => {
    const appId = window.__HC_LAUNCHER_ENV__.INSTALLED_APP_ID;
    await invokeTauriCommand("reset_happ_notification_count", {
      appId,
      notificationIds,
    });
  },
};

window.addEventListener("focus", async () => {
  await invoke("clear_happ_notifications", {
    appId: window.__HC_LAUNCHER_ENV__.INSTALLED_APP_ID,
  });
});
