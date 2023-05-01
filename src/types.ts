import {
  AgentPubKey,
  AnyDhtHash,
  AppRoleManifest,
  AppInfo,
  EntryHash,
  DnaHash,
  DnaHashB64,
  AnyDhtHashB64,
} from "@holochain/client";
import { Entity, GUIReleaseEntry, HappReleaseEntry } from "./appstore/types";

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
  hdi_version: HdiVersion;
  hdk_version: HdkVersion;
}

export type HolochainState = RunningState<HolochainInfo, string>;

export type HolochainVersion = string;
export type HdkVersion = string;
export type HdiVersion = string;

export type HolochainId =
  | {
      type: "HolochainVersion";
      content: HolochainVersion;
    }
  | {
      type: "CustomBinary";
      content: undefined;
    };

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

export interface RunningHolochainsStateInfo {
  versions: Record<HolochainVersion, HolochainState>;
  custom_binary: HolochainState | undefined;
}

export interface LauncherStateInfo {
  state: RunningState<
    RunningState<RunningHolochainsStateInfo, KeystoreStatus>,
    RunLauncherError
  >;
  config: LauncherConfig;
  default_version: HolochainVersion;
}

export type LogLevel = "Error" | "Warn" | "Info" | "Debug" | "Trace";

export interface LauncherConfig {
  log_level: LogLevel;
  running_versions: HolochainVersion[];
  custom_binary_path: string | undefined;
}

export type WebUiInfo =
  | {
      type: "Headless";
    }
  | {
      type: "WebApp";
      path_to_web_app: string;
      app_ui_port: number;
      gui_release_info: ReleaseInfo | undefined;
    };


export interface Hrl {
  dna_hash: DnaHash,
  resource_hash: AnyDhtHash,
}

export interface HrlB64 {
  dna_hash: DnaHashB64,
  resource_hash: AnyDhtHashB64,
}

export interface InstalledWebAppInfo {
  installed_app_info: AppInfo;
  happ_release_info: ReleaseInfo | undefined;
  web_uis: Record<string, WebUiInfo>;
}

export interface HolochainAppInfo {
  webAppInfo: InstalledWebAppInfo;
  holochainId: HolochainId;
  holochainVersion: HolochainVersion;
}

export interface HolochainAppInfoExtended {
  webAppInfo: InstalledWebAppInfo;
  holochainId: HolochainId;
  holochainVersion: HolochainVersion;
  guiUpdateAvailable: Hrl | undefined; // gui release entry hash if there is known to be a new gui release available in the DevHub
}


export interface GossipProgress {
  expectedBytes: number;
  actualBytes: number;
}

export interface StorageInfo {
  uis: number;
  authored: number;
  cached: number;
  conductor: number;
  dht: number;
  p2p: number;
  wasm: number;
}


export interface ReleaseData {
  devhubDnaHash: DnaHash,
  happRelease: Entity<HappReleaseEntry>,
  guiRelease: Entity<GUIReleaseEntry> | undefined,
}

export interface ReleaseInfo {
  hrl: HrlB64,
  version: string | undefined,
}