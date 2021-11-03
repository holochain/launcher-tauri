import { AppRoleManifest } from "@holochain/conductor-api";

export interface WebAppInfo {
  app_name: string;
  roles_to_create: Array<AppRoleManifest>;
}

export type ConnectionStatus =
  | {
      type: "Connected";
      admin_interface_port: number;
      caddy_admin_port: number;
    }
  | { type: "Error"; error: string }
  | {
      type: "AlreadyRunning";
    };
