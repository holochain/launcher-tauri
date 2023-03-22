import { AgentPubKey, AppInfo, AppWebsocket, decodeHashFromBase64, DnaHash, DnaHashB64, EntryHash } from "@holochain/client";
import { getCellId } from "../utils";
import { AppEntry, CustomRemoteCallInput, Entity, GetWebHappPackageInput, HappReleaseEntry, HostEntry } from "./types";



// hard coded dna hash of the DevHub in use
const DEVHUB_DNA_HASH_B64: DnaHashB64 = "dummy93f0938f09f";
const DEVHUB_DNA_HASH: DnaHash = decodeHashFromBase64(DEVHUB_DNA_HASH_B64);




export async function getAllApps(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
): Promise<Array<AppEntry>> {
  const appstoreCell = appStoreApp.cell_info["appstore"].find((c) => "provisioned" in c);

  if (!appstoreCell) {
    throw new Error("appstore cell not found.")
  } else {
    const allApps: Array<Entity<AppEntry>> = await appWebsocket.callZome({
      fn_name: "get_all_apps",
      zome_name: "appstore_api",
      cell_id: getCellId(appstoreCell)!,
      payload: null,
      provenance: getCellId(appstoreCell)![1],
    })

    return allApps.map((appEntity) => appEntity.content)
  }

}


// =========== Bridge Calls via Portal API =================


export async function getHappReleases(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  forHapp: EntryHash,
): Promise<Array<HappReleaseEntry>> {

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);
  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      "happ_library",
      "get_happ_releases",
    );

    const input: CustomRemoteCallInput = {
      host,
      call: {
        dna: DEVHUB_DNA_HASH,
        zome: "happ_library",
        function: "get_happ_releases",
        payload: {
          for_happ: forHapp,
        }
      }
    }

    const happReleaseEntities: Array<Entity<HappReleaseEntry>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    return happReleaseEntities.map((entity) => entity.content);
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
        dna: DEVHUB_DNA_HASH,
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
        dna: DEVHUB_DNA_HASH,
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
    // 1. get all registered hosts for this zome function
    const hosts: Array<Entity<HostEntry>> = await appWebsocket.callZome({
      fn_name: "get_hosts_for_zome_function",
      zome_name: "appstore_api",
      cell_id: getCellId(appstoreCell)!,
      payload: {
        dna: DEVHUB_DNA_HASH,
        zome: zome_name,
        function: fn_name,
      },
      provenance: getCellId(appstoreCell)![1],
    })

    // 2. ping each of them and take the first one that responds
    return Promise.any(hosts.map(async (hostEntity) => {
      const hostPubKey = hostEntity.content.author;
      const success: boolean = await appWebsocket.callZome({
        fn_name: "ping",
        zome_name: "portal_api",
        cell_id: getCellId(portalCell)!,
        payload: {
          dna: DEVHUB_DNA_HASH,
          zome: zome_name,
          function: fn_name,
        },
        provenance: getCellId(portalCell)![1],
      });
      // what happens in the "false" case? Can this happen?

      return hostPubKey;
    }))
  }
}



