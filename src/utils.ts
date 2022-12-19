import { CellInfo } from "@holochain/client";
import { CellId } from "@holochain/client";
import {
  DisabledAppReason,
  NetworkInfo,
  AppInfo,
} from "@holochain/client";
import prettyBytes from "pretty-bytes";

import { GossipProgress } from "./types";

export function isAppRunning(app: AppInfo): boolean {
  return (app.status as any) === "running";
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

    if ((reason as any) === "never_started") {
      return "App was never started";
    } else if ((reason as any) === "user") {
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
    return (
      app.status as unknown as {
        paused: { reason: { error: string } };
      }
    ).paused.reason.error;
  }
}

export function flattenCells(cell_info: Record<string, CellInfo[]>): [string, CellInfo][] {
  return Object.entries(cell_info).map(([roleName, cellInfos]) => {
    return cellInfos.map((CellInfo) => [roleName, CellInfo])
  }).flat() as any
}

export function getCellId(cellInfo: CellInfo): CellId | undefined {
  if ("Provisioned" in cellInfo) {
    return cellInfo.Provisioned.cell_id;
  }
  if ("Cloned" in cellInfo) {
    return cellInfo.Cloned.cell_id;
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
