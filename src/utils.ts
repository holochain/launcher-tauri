import { DisabledAppReason, InstalledAppInfo } from "@holochain/client";

export function isAppRunning(app: InstalledAppInfo): boolean {
  return (app.status as any) === "running";
}
export function isAppDisabled(app: InstalledAppInfo): boolean {
  return Object.keys(app.status).includes("disabled");
}
export function isAppPaused(app: InstalledAppInfo): boolean {
  return Object.keys(app.status).includes("paused");
}
export function getReason(app: InstalledAppInfo): string | undefined {
  if (isAppRunning(app)) return undefined;
  if (isAppDisabled(app)) {
    const reason = (
      app.status as unknown as {
        disabled: {
          reason: DisabledAppReason;
        };
      }
    ).disabled.reason;

    if (Object.keys(reason).includes("never_started")) {
      return "App was never started";
    } else if (Object.keys(reason).includes("user")) {
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
