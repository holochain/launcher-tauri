import { AppRoleManifest, InstalledAppInfo } from "@holochain/client";

export interface WebAppInfo {
  app_name: string;
  roles_to_create: Array<AppRoleManifest>;
}

export type RunningState<T, E> =
  | {
      type: "Running";
      content: T;
    }
  | {
      type: "Error";
      content: E;
    };

export type HolochainStateInfo = RunningState<InstalledAppInfo[], string>;

export type HolochainVersion = string;

export type LaunchTauriSidecarError =
  | {
      type: "BinaryNotFound";
    }
  | {
      type: "FailedToExecute";
      content: string;
    };

export type LairKeystoreError =
  | {
      type: "LaunchTauriSidecarError";
      content: LaunchTauriSidecarError;
    }
  | {
      type: "ErrorWritingPassword";
      content: string;
    };
export type KeystoreStatus =
  | { type: "InitNecessary" }
  | {
      type: "PasswordNecessary";
    }
  | {
      type: "LaunchKeystoreError";
      content: LairKeystoreError;
    };

export type RunLauncherError =
  | { type: "AnotherInstanceIsAlreadyRunning" }
  | {
      type: "OldFilesExist";
    }
  | {
      type: "FileSystemError";
      content: string;
    }
  | {
      type: "ErrorLaunching";
      content: string;
    };

export interface LauncherStateInfo {
  state: RunningState<
    RunningState<Record<HolochainVersion, HolochainStateInfo>, KeystoreStatus>,
    RunLauncherError
  >;
  config: LauncherConfig;
}

export type LogLevel = "Error" | "Warn" | "Info" | "Debug" | "Trace";

export interface LauncherConfig {
  log_level: LogLevel;
}

export type WebUiInfo =
  | {
      type: "Headless";
    }
  | {
      type: "WebApp";
      path_to_web_app: string;
      app_ui_port: number;
    };

export interface InstalledWebAppInfo {
  installed_app_info: InstalledAppInfo;
  web_ui_info: WebUiInfo;
}
