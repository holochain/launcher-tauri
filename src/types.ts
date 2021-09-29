import { AppSlotManifest } from "@holochain/conductor-api";

export interface WebAppInfo {
  app_name: string;
  slots_to_create: Array<AppSlotManifest>;
}

export type ConnectionStatus =
  | {
      type: "Connected";
      admin_interface_port: number;
    }
  | { type: "Error"; error: string }
  | {
      type: "AlreadyRunning";
    };
