import {
  AgentPubKey,
  AppRoleManifest,
  InstalledAppInfo,
} from "@holochain/client";

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

export interface HolochainInfo {
  installed_apps: InstalledWebAppInfo[];
  app_interface_port: number;
  admin_interface_port: number;
}

export type HolochainState = RunningState<HolochainInfo, string>;

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
    RunningState<Record<HolochainVersion, HolochainState>, KeystoreStatus>,
    RunLauncherError
  >;
  config: LauncherConfig;
}

export type LogLevel = "Error" | "Warn" | "Info" | "Debug" | "Trace";

export interface LauncherConfig {
  log_level: LogLevel;
  running_versions: HolochainVersion[];
  version_for_devhub: HolochainVersion;
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

export interface AppSetup {
  appId: string;
  uid: string | undefined;
  reuseAgentPubKey: AgentPubKey | undefined;
  holochainVersion: HolochainVersion;
  membraneProofs: { [key: string]: Array<number> } | undefined;
}
