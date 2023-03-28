import { AgentPubKey, AppInfo, AppWebsocket, decodeHashFromBase64, DnaHash, DnaHashB64, encodeHashToBase64, EntryHash } from "@holochain/client";
import { getCellId } from "../utils";
import { AppEntry, CustomRemoteCallInput, DevHubResponse, Entity, GetWebHappPackageInput, HappReleaseEntry, HostEntry } from "./types";



// hard coded dna hash of the DevHub in use
const DEVHUB_HAPP_LIBRARY_DNA_HASH_B64: DnaHashB64 = "uhC0ke1JijHM0tAVTy3OH3-i1fuZzB7FlCBi2oLiD96p-NW97ueuK";
const DEVHUB_HAPP_LIBRARY_DNA_HASH: DnaHash = decodeHashFromBase64(DEVHUB_HAPP_LIBRARY_DNA_HASH_B64);




export async function getAllApps(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
): Promise<Array<AppEntry>> {
  console.log("@getAllApps");
  const appstoreCell = appStoreApp.cell_info["appstore"].find((c) => "provisioned" in c);

  console.log("@getAllApps: appstoreCell", appstoreCell);

  if (!appstoreCell) {
    throw new Error("appstore cell not found.")
  } else {
    const allApps: DevHubResponse<Array<Entity<AppEntry>>> = await appWebsocket.callZome({
      fn_name: "get_all_apps",
      zome_name: "appstore_api",
      cell_id: getCellId(appstoreCell)!,
      payload: null,
      provenance: getCellId(appstoreCell)![1],
    })

    console.log("@getAllApps: allApps", allApps);


    return allApps.payload.map((appEntity) => appEntity.content)
  }

}


// =========== Bridge Calls via Portal API =================

/**
 * Gets the happ releases corresponding to the passed entry hashes
 *
 * @param happReleaseEntryHash
 */
export async function getHappReleasesByEntryHashes(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  happReleaseEntryHashes: Array<EntryHash | undefined>
) {

  // Find an online host
  const host: AgentPubKey = await getAvailableHostForZomeFunction(
    appWebsocket,
    appStoreApp,
    "happ_library",
    "get_happ_releases",
  );

  // make zome calls for each EntryHash to this one host
  const happReleases = await Promise.all(happReleaseEntryHashes.map( async (entryHash) => {
    if (entryHash) {
      return getHappReleaseFromHost(appWebsocket, appStoreApp, host, entryHash);
    } else {
      return undefined;
    }
  }));

  return happReleases;
}

/**
 * Get the happ released associated to a happ release entry hash from a specific DevHub host
 * @param appWebsocket
 * @param appStoreApp
 * @param host
 * @param entryHash
 */
async function getHappReleaseFromHost (
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  host: AgentPubKey,
  entryHash: EntryHash, // EntryHash of the HappReleaseEntry
): Promise<Entity<HappReleaseEntry>> {

  const input: CustomRemoteCallInput = {
    host,
    call: {
      dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
      zome: "happ_library",
      function: "get_happ_release",
      payload: {
        id: entryHash,
      }
    }
  }

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    const happReleaseResponse: DevHubResponse<Entity<HappReleaseEntry>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    // maybe it needs to be entity.payload.content instead...
    return happReleaseResponse.payload;
  }
}


/**
 * Get happ releases for a happ. Searches for an online DevHub host first.
 * @param appWebsocket
 * @param appStoreApp
 * @param forHapp
 * @returns
 */
export async function getHappReleases(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  forHapp: EntryHash,
): Promise<Array<Entity<HappReleaseEntry>>> {

  console.log("@getHappReleases: trying to get host.");

  const host: AgentPubKey = await getAvailableHostForZomeFunction(
    appWebsocket,
    appStoreApp,
    "happ_library",
    "get_happ_releases",
  );

  console.log("@getHappReleases: found host: ", host);

  return getHappReleasesFromHost(appWebsocket, appStoreApp, host, forHapp);
}

/**
 * Get happ releases for a happ from a specific DevHub host
 * @param appWebsocket
 * @param appStoreApp
 * @param host
 * @param forHapp
 */
async function getHappReleasesFromHost (
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  host: AgentPubKey,
  forHapp: EntryHash,
): Promise<Array<Entity<HappReleaseEntry>>> {

  const input: CustomRemoteCallInput = {
    host,
    call: {
      dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
      zome: "happ_library",
      function: "get_happ_releases",
      payload: {
        for_happ: forHapp,
      }
    }
  }

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    const happReleaseEntities: DevHubResponse<Array<Entity<HappReleaseEntry>>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    // maybe it needs to be entity.payload.content instead...
    return happReleaseEntities.payload;
  }
}


export async function fetchWebHapp(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  name: string,
  happReleaseEntryHash: EntryHash,
  guiReleaseEntryHash: EntryHash,
): Promise<Uint8Array> {

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);
  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      "happ_library",
      "get_webhapp_package",
    );

    const payload: GetWebHappPackageInput = {
      name,
      happ_release_id: happReleaseEntryHash,
      gui_release_id: guiReleaseEntryHash,
    };

    const input: CustomRemoteCallInput = {
      host,
      call: {
        dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
        zome: "happ_library",
        function: "get_webhapp_package",
        payload,
      }
    }

    const webHappBytes: Uint8Array = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    return webHappBytes;
  }
}


export async function fetchGui(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  webAssetEntryHash: EntryHash,
): Promise<Uint8Array> {

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);
  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      "happ_library",
      "get_webasset",
    );


    const input: CustomRemoteCallInput = {
      host,
      call: {
        dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
        zome: "happ_library",
        function: "get_webasset",
        payload: {
          id: webAssetEntryHash,
        },
      }
    }

    const guiBytes: Uint8Array = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    return guiBytes;
  }
}





/**
 * Remote call to DevHub host
 *
 * 1. get all registered hosts for the given zome function via the get_hosts_for_zome_function zome call
 *
 * 2. for each of those hosts, send a ping via portal_api/ping zome function, with Promise.any()
 *
 * 3. return the first that responds to the ping
 *
 */
export async function getAvailableHostForZomeFunction(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  zome_name: string,
  fn_name: string,
): Promise<AgentPubKey> {

  const appstoreCell = appStoreApp.cell_info["appstore"].find((c) => "provisioned" in c);
  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

  if (!appstoreCell) {
    throw new Error("appstore cell not found.")
  } else if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {
    console.log("@getAvailableHostForZomeFunction: searching hosts.");

    // 1. get all registered hosts for this zome function
    const hosts: DevHubResponse<Array<Entity<HostEntry>>> = await appWebsocket.callZome({
      fn_name: "get_hosts_for_zome_function",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: {
        dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
        zome: zome_name,
        function: fn_name,
      },
      provenance: getCellId(portalCell)![1],
    })

    console.log("@getAvailableHostForZomeFunction: found hosts: ", hosts);
    let b64Hosts = hosts.payload.map((entity) => encodeHashToBase64(entity.content.author));
    console.log("@getAvailableHostForZomeFunction: b64 hosts: ", b64Hosts);

    // 2. ping each of them and take the first one that responds
    return Promise.any(hosts.payload.map(async (hostEntity) => {
      const hostPubKey = hostEntity.content.author;
      console.log("@getAvailableHostForZomeFunction: trying to ping host: ", hostPubKey);

      const success: boolean = await appWebsocket.callZome({
        fn_name: "ping",
        zome_name: "portal_api",
        cell_id: getCellId(portalCell)!,
        payload: {
          dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
          zome: zome_name,
          function: fn_name,
        },
        provenance: getCellId(portalCell)![1],
      });
      // what happens in the "false" case? Can this happen?

      console.log("@getAvailableHostForZomeFunction Sent ping to host and got result: ", success);

      return hostPubKey;
    }))
  }
}



