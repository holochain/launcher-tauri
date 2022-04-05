import { HdkVersion } from "@/hdk";
import {
  AppWebsocket,
  EntryHash,
  InstalledAppInfo,
  InstalledCell,
} from "@holochain/client";
import { Happ, HappRelease } from "./types";

export interface ContentAddress<C> {
  address: EntryHash;
  content: C;
}

export interface AppWithReleases {
  app: ContentAddress<Happ>;
  releases: Array<ContentAddress<HappRelease>>;
}

export async function getAppsForHdk(
  appWebsocket: AppWebsocket,
  devhubHappCell: InstalledCell,
  hdkVersion: HdkVersion
): Promise<Array<AppWithReleases>> {
  const allAppsOutput = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: devhubHappCell.cell_id,
    fn_name: "get_all_happs",
    zome_name: "happ_library",
    payload: null,
    provenance: devhubHappCell.cell_id[1],
  });

  const allApps: Array<ContentAddress<Happ>> = allAppsOutput.payload.items;

  console.log(allApps);

  const promises = allApps.map((app) =>
    getAppsReleases(appWebsocket, devhubHappCell, app)
  );

  return Promise.all(promises);
}

export async function getAppsReleases(
  appWebsocket: AppWebsocket,
  devhubHappCell: InstalledCell,
  app: ContentAddress<Happ>
): Promise<AppWithReleases> {
  const appReleasesOutput = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: devhubHappCell.cell_id,
    fn_name: "get_all_happs",
    zome_name: "happ_library",
    payload: {
      for_happ: app.address,
    },
    provenance: devhubHappCell.cell_id[1],
  });

  const releases = appReleasesOutput.payload.items;

  return {
    app,
    releases,
  };
}

export async function getWebHapp(
  appWebsocket: AppWebsocket,
  devhubHappCell: InstalledCell,
  devhubDnaCell: InstalledCell,
  devhubWebAssetsCell: InstalledCell,
  happReleaseEntryHash: EntryHash
): Promise<Uint8Array> {
  return appWebsocket.callZome({
    cap_secret: null,
    cell_id: devhubHappCell.cell_id,
    fn_name: "get_webhapp_package",
    zome_name: "happ_library",
    payload: {
      name: String,
      id: happReleaseEntryHash,
      dnarepo_dna_hash: devhubDnaCell.cell_id[0],
      webassets_dna_hash: devhubWebAssetsCell.cell_id[0],
    },
    provenance: devhubHappCell.cell_id[1],
  });
}
