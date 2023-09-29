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

// const windowVisible = await appWindow.isVisible();
// const now = Date.now();
// await Promise.all(
//   notifications.map(async (notification) => {
//     // check whether it's actually a new event or not. Events older than 5 minutes won't trigger an OS notification
//     // because it is assumed that they are emitted by the Applet UI upon startup of We and occurred while the
//     // user was offline
//     if (now - notification.timestamp < 300000) {
//       await notifyTauri(
//         notification,
//         appNotificationSettings.showInSystray && !windowVisible,
//         appNotificationSettings.allowOSNotification &&
//           notification.urgency === "high"
//       );
//     }
//   })
// );

// /**
//  * Stores happ notifications to localStorage - to the array of unread notifications
//  * as well as to a persistent (deduplicated) log of all received notifications
//  *
//  * @param notifications
//  * @param appletId
//  * @returns
//  */
// export function storeHappNotifications(
//   notifications: Array<HappNotification>,
//   appId: string
// ): Array<HappNotification> {
//   // store them to unread messages
//   const unreadNotificationsJson: string | null = window.localStorage.getItem(
//     `happNotificationsUnread#${appId}`
//   );
//   let unreadNotifications: Array<HappNotification>;

//   if (unreadNotificationsJson) {
//     unreadNotifications = JSON.parse(unreadNotificationsJson);
//     unreadNotifications = [
//       ...new Set([...unreadNotifications, ...notifications]),
//     ]; // dedpulicated array
//   } else {
//     unreadNotifications = [...notifications];
//   }

//   window.localStorage.setItem(
//     `happNotificationsUnread#${appId}`,
//     JSON.stringify(unreadNotifications)
//   );

//   // store to persistend time-indexed notifications log
//   notifications.forEach((notification) => {
//     const timestamp = notification.timestamp;
//     const daysSinceEpoch = Math.floor(timestamp / 8.64e7);
//     const notificationsOfSameDateJSON: string | null =
//       window.localStorage.getItem(
//         `happNotifications#${daysSinceEpoch}#${appId}`
//       );
//     let notificationsOfSameDate: Array<HappNotification>;
//     if (notificationsOfSameDateJSON) {
//       notificationsOfSameDate = JSON.parse(notificationsOfSameDateJSON);
//       notificationsOfSameDate = [
//         ...new Set([...notificationsOfSameDate, notification]),
//       ];
//     } else {
//       notificationsOfSameDate = [notification];
//     }
//     window.localStorage.setItem(
//       `happNotifications#${daysSinceEpoch}#${appId}`,
//       JSON.stringify(notificationsOfSameDate)
//     );
//   });

//   return unreadNotifications;
// }
