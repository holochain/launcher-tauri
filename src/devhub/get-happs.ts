import { HdkVersion } from "../hdk";
import {
  AppWebsocket,
  EntryHash,
  ActionHash,
  AppInfo,
  EntryHashB64,
  decodeHashFromBase64,
} from "@holochain/client";
import { GUIReleaseEntry, HappEntry, HappReleaseEntry } from "./types";
import { getCellId } from "../utils";

// corresponds to https://docs.rs/hc_crud_ceps/0.55.0/hc_crud/struct.Entity.html
export interface Entity<T> {
  id: EntryHash;
  action: ActionHash;
  address: EntryHash;
  ctype: EntityType;
  content: T;
}

// corresponds to https://docs.rs/hc_crud_ceps/0.55.0/hc_crud/struct.EntityType.html
export interface EntityType {
  name: string;
  model: string;
}

export interface GUIReleaseResponsePayload {
  action: ActionHash;
  address: EntryHash; // The address of the current entry
  content: GUIReleaseEntry;
  id: EntryHash; // The address of the original created entry
  type: any;
}

export interface ContentAddress<C> {
  id: EntryHash;
  address: EntryHash;
  content: C;
}

export interface AppWithReleases {
  app: ContentAddress<HappEntry>;
  happReleases: Array<ContentAddress<HappReleaseEntry>>;
  guiReleases: Array<ContentAddress<GUIReleaseEntry>>;
}

export function filterByHdkVersion(
  hdkVersions: HdkVersion[],
  apps: Array<AppWithReleases>
): Array<AppWithReleases> {
  const filteredReleases: Array<AppWithReleases> = apps.map((app) => ({
    app: app.app,
    happReleases: app.happReleases.filter((r) =>
      hdkVersions.includes(r.content.hdk_version)
    ),
    guiReleases: app.guiReleases
  }));

  return filteredReleases.filter((app) => app.happReleases.length > 0);
}

// filtered by apps with GUI's
export async function getAllAppsWithGui(
  appWebsocket: AppWebsocket,
  devhubHapp: AppInfo
): Promise<Array<AppWithReleases>> {
  const cells = devhubCells(devhubHapp);
  const allAppsOutput = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: getCellId(cells.happs.find((c) => "Provisioned" in c )!)!,
    fn_name: "get_happs_by_tags",
    zome_name: "happ_library",
    payload: ["app-store-ready"],
    provenance: getCellId(cells.happs.find((c) => "Provisioned" in c )!)![1],
  });
  // console.log("@getAllAppsWithGui: ", allAppsOutput);
  const allApps: Array<ContentAddress<HappEntry>> = allAppsOutput.payload;
  const promises = allApps.map((app) =>
    getAppsReleasesWithGui(appWebsocket, devhubHapp, app)
  );

  return Promise.all(promises);
}

export async function getAppsReleasesWithGui(
  appWebsocket: AppWebsocket,
  devhubHapp: AppInfo,
  app: ContentAddress<HappEntry>
): Promise<AppWithReleases> {
  const cells = devhubCells(devhubHapp);

  const appReleasesOutput = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: getCellId(cells.happs.find((c) => "Provisioned" in c )!)!,
    fn_name: "get_happ_releases",
    zome_name: "happ_library",
    payload: {
      for_happ: app.id,
    },
    provenance: getCellId(cells.happs.find((c) => "Provisioned" in c )!)![1],
  });

  // console.log("@getAppsReleases: appReleasesOutput:", appReleasesOutput);

  const allReleases: Array<Entity<HappReleaseEntry>> = appReleasesOutput.payload;

  const releases: Array<ContentAddress<HappReleaseEntry>> = allReleases.map(
    (entity) => {
      return {
        id: entity.id,
        address: entity.address,
        content: entity.content,
      };
    }
  );

  // console.log("@getAppsReleases: releases: ", releases);

  const filteredReleases = releases.filter((r) => !!r.content.official_gui);

  let guiReleases: Array<GUIReleaseResponsePayload> = []

  await Promise.all(
    filteredReleases.map(async (release) => {
      const guiRelease = await appWebsocket.callZome({
        cap_secret: null,
        cell_id: getCellId(cells.happs.find((c) => "Provisioned" in c )!)!,
        fn_name: "get_gui_release",
        zome_name: "happ_library",
        payload: {
          id: release.content.official_gui
        },
        provenance: getCellId(cells.happs.find((c) => "Provisioned" in c )!)![1],
      })
      guiReleases.push(guiRelease.payload)
    })
  )


  // console.log("@getAppsReleases: filteredReleases: ", filteredReleases);
  return {
    app,
    happReleases: filteredReleases,
    guiReleases,
  };
}

export function getLatestRelease(
  apps: AppWithReleases
): ContentAddress<HappReleaseEntry> {
  return apps.happReleases.sort(
    (r1, r2) => r2.content.last_updated - r1.content.last_updated
  )[0];
}


const sleep = (ms: number) => new Promise((r) => setTimeout(() => r(null), ms));

export async function fetchWebHapp(
  appWebsocket: AppWebsocket,
  devhubHapp: AppInfo,
  name: string,
  happReleaseEntryHash: EntryHash,
  guiReleaseEntryHash: EntryHash,
  retryCount = 3
): Promise<Uint8Array> {
  const cells = devhubCells(devhubHapp);

  const result = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: getCellId(cells.happs.find((c) => "Provisioned" in c )!)!,
    fn_name: "get_webhapp_package",
    zome_name: "happ_library",
    payload: {
      name,
      happ_release_id: happReleaseEntryHash,
      gui_release_id: guiReleaseEntryHash,
    },
    provenance: getCellId(cells.happs.find((c) => "Provisioned" in c )!)![1],
  });

  if (result.payload.error) {
    if (retryCount === 0) {
      throw new Error(result.payload.error);
    } else {
      await sleep(1000);
      return fetchWebHapp(
        appWebsocket,
        devhubHapp,
        name,
        happReleaseEntryHash,
        guiReleaseEntryHash,
        retryCount - 1
      );
    }
  }

  return result.payload;
}

export function devhubCells(devhubHapp: AppInfo) {
  const happs = devhubHapp.cell_info["happs"];
  const dnarepo = devhubHapp.cell_info["dnarepo"];
  const webassets = devhubHapp.cell_info["web_assets"];

  if (!happs || !dnarepo || !webassets) throw new Error("Bad app info");

  return {
    happs,
    dnarepo,
    webassets,
  };
}



/**
 * Gets the happ releases corresponding to the passed entry hashes
 *
 * @param happReleaseEntryHash
 */
export async function getHappReleasesByEntryHashes(
  appWebsocket: AppWebsocket,
  devhubHapp: AppInfo,
  happReleaseEntryHashes: Array<EntryHash | undefined>
) {

  const cells = devhubCells(devhubHapp);
  const happReleases = await Promise.all(happReleaseEntryHashes.map( async (entryHash) => {
    if (entryHash) {
      return appWebsocket.callZome({
        cap_secret: null,
        cell_id: getCellId(cells.happs.find((c) => "Provisioned" in c )!)!,
        fn_name: "get_happ_release",
        zome_name: "happ_library",
        payload: {
          id: entryHash,
        },
        provenance: getCellId(cells.happs.find((c) => "Provisioned" in c )!)![1],
      })
    } else {
      return undefined;
    }
  }));

  return happReleases.map((response) => {
    if (response) {
      return response.payload.content;
    } else {
      return undefined;
    }
  });
}


/**
 * Fetches a GUI from the DevHub
 * @param appWebsocket
 * @param devhubHapp
 * @param name
 * @param happReleaseEntryHash
 * @param guiReleaseEntryHash
 * @param retryCount
 * @returns
 */
export async function fetchGui(
  appWebsocket: AppWebsocket,
  devhubHapp: AppInfo,
  webAssetEntryHash: EntryHash,
  retryCount = 3
): Promise<Uint8Array> {
  const cells = devhubCells(devhubHapp);

  const result = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: getCellId(cells.happs.find((c) => "Provisioned" in c )!)!,
    fn_name: "get_webasset",
    zome_name: "happ_library",
    payload: {
      id: webAssetEntryHash,
    },
    provenance: getCellId(cells.happs.find((c) => "Provisioned" in c )!)![1],
  });

  if (result.payload.error) {
    if (retryCount === 0) {
      throw new Error(result.payload.error);
    } else {
      await sleep(1000);
      return fetchGui(
        appWebsocket,
        devhubHapp,
        webAssetEntryHash,
        retryCount - 1
      );
    }
  }

  return result.payload.content.bytes;
}
