import { AppSlotManifest } from "@holochain/conductor-api";

export interface WebAppInfo {
  app_name: string;
  slots_to_create: Array<AppSlotManifest>;
}
