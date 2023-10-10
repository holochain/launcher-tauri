declare global {
  interface Window {
    __HC_LAUNCHER_API__: {
      notify: (notifications: Array<HappNotification>) => void;
      resetNotificationCount: (notificationIds: Array<NotificationId>) => void;
    };
  }
}

export async function launcherApiAvailable() {
  return !!window.__HC_LAUNCHER_API__;
}

/**
 * Sends notifications to the Holochain Launcher
 *
 * @param notifications Array of notifications to send
 */
export async function notifyLauncher(notifications: Array<HappNotification>) {
  window.__HC_LAUNCHER_API__.notify(notifications);
}

/**
 * Resets the notifications counts for the notifications with the specified ids
 *
 * @param notificationIds Ids of the notifications whose counts shall be reset
 */
export async function resetNotificationCount(
  notificationIds: Array<NotificationId>
) {
  window.__HC_LAUNCHER_API__.resetNotificationCount(notificationIds);
}

export type NotificationId = string;

export interface HappNotification {
  /**
   * Title of the notification.
   */
  title: string;
  /**
   * content of the notification
   */
  body: string;
  /**
   * type of notification, in a chat app e.g. "message" or "@mention". May potentially be used to show
   * notifications aggregated by type in notification feeds
   */
  notification_type: string;
  /**
   * Icon for the message type. The Holochain Launcher looks for a file with this
   * name in the icons/ directory of the app's UI assets.
   */
  icon_file_name: string | undefined;
  /**
   * urgency level "low" only shows up in the Launcher UI when opened
   * urgency level "medium" also shows up as a dot in the system tray icon
   * urgency level "high" shows up as an urgent dot in the system tray icon and triggers an OS notification
   */
  urgency: "low" | "medium" | "high";
  /**
   * Timestamp **in milliseconds** of when the event that the notification is about
   * has occured.
   * Ideally the timestamp of the DHT Action associated to the notification.
   * It may be displayed by Launcher in notification feeds and will be used to determine
   * whether an event is "fresh" or has occurred while the user was offline.
   * In the latter case, Launcher will not show an OS notification for
   * that notification on startup.
   */
  timestamp: number;
  /**
   * If not provided, Launcher resets the notification count (used for
   * dots on app icons and similar) for this message automatically when
   * the user opens the app (default). Otherwise, the app is assumed
   * to take care of clearing the notification count for this message via
   * use of resetNotificationCount() and based on applet-internal logic.
   *
   * CAUTION: If handled improperly by the app, this can lead to accumulation
   * of stale notifications.
   */
  custom_count_reset?: NotificationId;
}
