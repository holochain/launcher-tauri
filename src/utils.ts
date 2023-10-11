import {
  CellInfo,
  InstalledAppId,
  encodeHashToBase64,
} from "@holochain/client";
import { CellId } from "@holochain/client";
import { DisabledAppReason, AppInfo } from "@holochain/client";
import prettyBytes from "pretty-bytes";
import { Base64 } from "js-base64";

import {
  GossipProgress,
  HappNotificationSettings,
  ResourceLocator,
  ResourceLocatorB64,
} from "./types";
import { HappNotification, NotificationId } from "@holochain/launcher-api";

export function locatorToLocatorB64(
  locator: ResourceLocator
): ResourceLocatorB64 {
  return {
    dna_hash: encodeHashToBase64(locator.dna_hash),
    resource_hash: encodeHashToBase64(locator.resource_hash),
  };
}

export function isAppRunning(app: AppInfo): boolean {
  return (app.status as unknown) === "running";
}

export function capitalizeFirstLetter(str: string) {
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
}

export function isAppDisabled(app: AppInfo): boolean {
  return Object.keys(app.status).includes("disabled");
}
export function isAppPaused(app: AppInfo): boolean {
  return Object.keys(app.status).includes("paused");
}
export function getReason(app: AppInfo): string | undefined {
  if (isAppRunning(app)) return undefined;
  if (isAppDisabled(app)) {
    const reason = (
      app.status as unknown as {
        disabled: {
          reason: DisabledAppReason;
        };
      }
    ).disabled.reason;

    if ((reason as unknown) === "never_started") {
      return "App was never started";
    } else if ((reason as unknown) === "user") {
      return "App was disabled by the user";
    } else {
      return `There was an error with this app: ${
        (
          reason as {
            error: string;
          }
        ).error
      }`;
    }
  } else {
    return `You may be offline:\n\n${
      (
        app.status as unknown as {
          paused: { reason: { error: string } };
        }
      ).paused.reason.error
    }`;
  }
}

export function flattenCells(
  cell_info: Record<string, CellInfo[]>
): [string, CellInfo][] {
  return Object.entries(cell_info)
    .map(([roleName, cellInfos]) => {
      return cellInfos.map((CellInfo) => [roleName, CellInfo]);
    })
    .flat() as [string, CellInfo][];
}

export function getCellId(cellInfo: CellInfo): CellId | undefined {
  if ("provisioned" in cellInfo) {
    return cellInfo.provisioned.cell_id;
  }
  if ("cloned" in cellInfo) {
    return cellInfo.cloned.cell_id;
  }
  return undefined;
}

export function getCellName(cellInfo: CellInfo): string | undefined {
  if ("provisioned" in cellInfo) {
    return cellInfo.provisioned.name;
  }
  if ("cloned" in cellInfo) {
    return cellInfo.cloned.name;
  }
  if ("stem" in cellInfo) {
    return cellInfo.stem.name;
  }
}

export function getCellNetworkSeed(cellInfo: CellInfo): string | undefined {
  if ("provisioned" in cellInfo) {
    return cellInfo.provisioned.dna_modifiers.network_seed;
  }
  if ("cloned" in cellInfo) {
    return cellInfo.cloned.dna_modifiers.network_seed;
  }
  return undefined;
}

// GossipProgress will only return anticipated bytes soon so these methods will become obsolete
export function gossipProgressPercent(progress: GossipProgress | undefined) {
  if (!progress) {
    return undefined;
  }
  const ratio = 100 * (progress.actualBytes / progress.expectedBytes);
  return ratio > 100 ? 100 : ratio;
}

export function gossipProgressString(progress: GossipProgress | undefined) {
  if (!progress) {
    return "- / -";
  }
  return `${prettyBytes(progress.actualBytes)} / ${prettyBytes(
    progress.expectedBytes
  )}`;
}

export function toSrc(
  data: Uint8Array | undefined,
  mimeType?: string
): string | undefined {
  if (data) {
    const base64Data = Base64.fromUint8Array(data);
    return `data:${mimeType ? mimeType : "image/png"};base64,${base64Data}`;
  }

  return undefined;
}

function getLocalStorageItem<T>(key: string): T | undefined {
  const item: string | null = window.localStorage.getItem(key);
  return item ? JSON.parse(item) : undefined;
}

function setLocalStorageItem<T>(key: string, value: T): void {
  window.localStorage.setItem(key, JSON.stringify(value));
}

/**
 * Stores happ notifications to localStorage - to the array of unread notifications
 * as well as to a persistent (deduplicated) log of all received notifications
 *
 * @param notifications
 * @param appId
 * @returns
 */
export function storeHappNotifications(
  notifications: Array<HappNotification>,
  appId: string
): Array<HappNotification> {
  // store them to unread messages
  let unreadNotifications = getLocalStorageItem<Array<HappNotification>>(
    `happNotificationsUnread#${appId}`
  );

  unreadNotifications = unreadNotifications
    ? [...new Set([...unreadNotifications, ...notifications])]
    : [...notifications];

  setLocalStorageItem<Array<HappNotification>>(
    `happNotificationsUnread#${appId}`,
    unreadNotifications
  );

  // store to persistend time-indexed notifications log
  notifications.forEach((notification) => {
    const timestamp = notification.timestamp;
    const daysSinceEpoch = Math.floor(timestamp / 8.64e7);
    let notificationsOfSameDate = getLocalStorageItem<Array<HappNotification>>(
      `happNotifications#${daysSinceEpoch}#${appId}`
    );
    notificationsOfSameDate = notificationsOfSameDate
      ? [...new Set([...notificationsOfSameDate, notification])]
      : [notification];

    setLocalStorageItem<Array<HappNotification>>(
      `happNotifications#${daysSinceEpoch}#${appId}`,
      notificationsOfSameDate
    );
  });

  return unreadNotifications;
}

export function readUnreadHappNotifications(
  appId: string
): Array<HappNotification> {
  const unreadNotifications = getLocalStorageItem<Array<HappNotification>>(
    `happNotificationsUnread#${appId}`
  );
  return unreadNotifications ? unreadNotifications : [];
}

export function clearHappNotifications(appId: InstalledAppId) {
  // clear all happ notifications without custom_count_reset id
  const unreadNotifications = getLocalStorageItem<Array<HappNotification>>(
    `happNotificationsUnread#${appId}`
  );

  setLocalStorageItem(
    `happNotificationsUnread#${appId}`,
    unreadNotifications
      ? unreadNotifications.filter(
          (notification) => !!notification.custom_count_reset
        )
      : []
  );
}

export function resetHappNotificationCount(
  appId: InstalledAppId,
  notificationIds: Array<NotificationId>
) {
  // clear all happ notifications **with** custom_count_reset id
  const urneadNotifications = getLocalStorageItem<Array<HappNotification>>(
    `happNotificationsUnread#${appId}`
  );

  setLocalStorageItem(
    `happNotificationsUnread#${appId}`,
    urneadNotifications
      ? urneadNotifications.filter((notification) => {
          if (!notification.custom_count_reset) return true;
          return !notificationIds.includes(notification.custom_count_reset);
        })
      : []
  );
}

export function validateNotifications(
  notifications: Array<HappNotification>
): Array<HappNotification> {
  notifications.forEach((notification) => {
    if (typeof notification.title !== "string") {
      throw new Error(
        "Received a notification with a title that's not of type string."
      );
    }
    if (typeof notification.body !== "string") {
      throw new Error(
        "Received a notification with a body that's not of type string."
      );
    }
    if (!["low", "medium", "high"].includes(notification.urgency)) {
      throw new Error(
        "Received a notification with an invalid urgency level. Valid urgency levels are ['low', 'medium', 'high']."
      );
    }
    if (
      notification.icon_file_name &&
      typeof notification.icon_file_name !== "string"
    ) {
      throw new Error(
        "Received a notification an invalid icon_src attribute. Must be either of type string or undefined."
      );
    }
    // validate timestamp
    if (typeof notification.timestamp !== "number") {
      throw new Error(
        `Received a notification with a timestamp that's not a number: ${notification.timestamp}`
      );
    } else if (!isMillisecondTimestamp(notification.timestamp)) {
      throw new Error(
        `Received a notification with a timestamp that's not in millisecond format: ${notification.timestamp}`
      );
    }
  });
  return notifications;
}

function isMillisecondTimestamp(timestamp: number): boolean {
  const reference = 1690803917545;
  if (timestamp / reference > 10 || reference / timestamp > 1.5) {
    return false;
  }
  return true;
}

/**
 * Gets the user-defined notification settings for the specified applet Id from localStorage
 * @param installedAppId
 * @returns
 */
export function getHappNotificationSettings(
  installedAppId: InstalledAppId
): HappNotificationSettings {
  const happNotificationSettings =
    getLocalStorageItem<HappNotificationSettings>(
      `happNotificationSettings#${installedAppId}`
    );
  return happNotificationSettings
    ? happNotificationSettings
    : {
        allowOSNotification: true,
        showInSystray: true,
        showInLauncherView: true,
        showInFeed: true,
      };
}

/**
 * Gets the user-defined notification settings for the specified applet Id from localStorage
 * @param installedAppId
 * @returns
 */
export function setHappNotificationSettings(
  installedAppId: InstalledAppId,
  settings: HappNotificationSettings
): void {
  setLocalStorageItem(`happNotificationSettings#${installedAppId}`, settings);
}

/**
 * Deletes all notification storage related to an app. To be called upon unsinstalling the app
 */
export function deleteNotificationStorage(
  installedAppId: InstalledAppId
): void {
  window.localStorage.removeItem(`happNotificationSettings#${installedAppId}`);
  window.localStorage.removeItem(`happNotificationsUnread#${installedAppId}`);
  // Remove all historical notificationss
  Object.keys(window.localStorage).forEach((key) => {
    if (
      key.includes("happNotifications#") &&
      key.includes(`#${installedAppId}`)
    ) {
      window.localStorage.removeItem(key);
    }
  });
}
