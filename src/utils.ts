import { CellInfo, encodeHashToBase64 } from "@holochain/client";
import { CellId } from "@holochain/client";
import { DisabledAppReason, AppInfo } from "@holochain/client";
import prettyBytes from "pretty-bytes";
import { Base64 } from "js-base64";

import { GossipProgress, ResourceLocator, ResourceLocatorB64 } from "./types";
import { HappNotification } from "@holochain-launcher/api";

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
  const unreadNotificationsJson: string | null = window.localStorage.getItem(
    `happNotificationsUnread#${appId}`
  );
  let unreadNotifications: Array<HappNotification>;

  if (unreadNotificationsJson) {
    unreadNotifications = JSON.parse(unreadNotificationsJson);
    unreadNotifications = [
      ...new Set([...unreadNotifications, ...notifications]),
    ]; // dedpulicated array
  } else {
    unreadNotifications = [...notifications];
  }

  window.localStorage.setItem(
    `happNotificationsUnread#${appId}`,
    JSON.stringify(unreadNotifications)
  );

  // store to persistend time-indexed notifications log
  notifications.forEach((notification) => {
    const timestamp = notification.timestamp;
    const daysSinceEpoch = Math.floor(timestamp / 8.64e7);
    const notificationsOfSameDateJSON: string | null =
      window.localStorage.getItem(
        `happNotifications#${daysSinceEpoch}#${appId}`
      );
    let notificationsOfSameDate: Array<HappNotification>;
    if (notificationsOfSameDateJSON) {
      notificationsOfSameDate = JSON.parse(notificationsOfSameDateJSON);
      notificationsOfSameDate = [
        ...new Set([...notificationsOfSameDate, notification]),
      ];
    } else {
      notificationsOfSameDate = [notification];
    }
    window.localStorage.setItem(
      `happNotifications#${daysSinceEpoch}#${appId}`,
      JSON.stringify(notificationsOfSameDate)
    );
  });

  return unreadNotifications;
}

export function readUnreadHappNotifications(
  appId: string
): Array<HappNotification> {
  const unreadNotificationsJson: string | null = window.localStorage.getItem(
    `happNotificationsUnread#${appId}`
  );

  if (unreadNotificationsJson) {
    return JSON.parse(unreadNotificationsJson);
  } else {
    return [];
  }
}

export function clearHappNotifications(appId: string) {
  // clear all happ notifications without customCountReset id
  const unreadNotificationsJson: string | null = window.localStorage.getItem(
    `happNotificationsUnread#${appId}`
  );

  if (unreadNotificationsJson) {
    const unreadNotifications: Array<HappNotification> = JSON.parse(
      unreadNotificationsJson
    );
    const unreadNotificationsCleared = unreadNotifications.filter(
      (notification) => !!notification.customCountReset
    );
    window.localStorage.setItem(
      `happNotificationsUnread#${appId}`,
      JSON.stringify(unreadNotificationsCleared)
    );
  } else {
    window.localStorage.setItem(
      `happNotificationsUnread#${appId}`,
      JSON.stringify([])
    );
  }
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
