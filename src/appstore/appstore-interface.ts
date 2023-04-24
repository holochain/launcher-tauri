import { AgentPubKey, AppInfo, AppWebsocket, decodeHashFromBase64, DnaHash, DnaHashB64, encodeHashToBase64, EntryHash } from "@holochain/client";
import { getCellId } from "../utils";
import { AppEntry, CustomRemoteCallInput, DevHubResponse, Entity, GetWebHappPackageInput, HappReleaseEntry, HostEntry, Response, MemoryEntry, MemoryBlockEntry, GUIReleaseEntry, FilePackage, HostAvailability, PublisherEntry } from "./types";
import { DEVHUB_HAPP_LIBRARY_DNA_HASH } from "../constants";



export function appstoreCells(appstoreHapp: AppInfo) {
  const appstore = appstoreHapp.cell_info["appstore"];
  const portal = appstoreHapp.cell_info["portal"];

  if (!appstore || !portal ) throw new Error("Bad app info");

  return {
    appstore,
    portal,
  };
}


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


/**
 * Gets a publisher entry
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param publisherEntryHash
 * @returns
 */
export async function getPublisher(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  publisherEntryHash: EntryHash,
): Promise<PublisherEntry> {

  const appstoreCell = appStoreApp.cell_info["appstore"].find((c) => "provisioned" in c);

  if (!appstoreCell) {
    throw new Error("appstore cell not found.")
  }

  try {
    const response: Response<Entity<PublisherEntry>> = await appWebsocket.callZome({
      fn_name: "get_publisher",
      zome_name: "appstore_api",
      cell_id: getCellId(appstoreCell)!,
      payload: {
        id: publisherEntryHash,
      },
      provenance: getCellId(appstoreCell)![1],
    })

    if (response.type !== "success") {
      return Promise.reject(`Failed to get publisher entry: ${response.payload}`)
    }

    return response.payload.content

  } catch (e) {
    return Promise.reject(`Failed to get publisher entry: ${e}`)
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
): Promise<Array<HappReleaseEntry | undefined>> {

  console.log("@getHappReleasesByEntryHashes: getting happ releases by entry hashes");

  // Find an online host
  const host: AgentPubKey = await getAvailableHostForZomeFunction(
    appWebsocket,
    appStoreApp,
    "happ_library",
    "get_happ_releases",
  );

  console.log("@getHappReleasesByEntryHashes: found host: ", host);


  // make zome calls for each EntryHash to this one host
  const happReleases = await Promise.all(happReleaseEntryHashes.map( async (entryHash) => {
    if (entryHash) {
      return getHappReleaseFromHost(appWebsocket, appStoreApp, host, entryHash);
    } else {
      return undefined;
    }
  }));


  console.log("@getHappReleasesByEntryHashes: Found happReleases: ", happReleases);

  return happReleases.map((response) => {
    if (response) {
      return response.content;
    } else {
      return undefined;
    }
  });
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

    const happReleaseResponse: DevHubResponse<Response<Entity<HappReleaseEntry>>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    // maybe it needs to be entity.payload.content instead...
    return happReleaseResponse.payload.payload;
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

  // console.log("@getHappReleases: trying to get host.");

  try {
    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      "happ_library",
      "get_happ_releases",
    );

    console.log("@getHappReleases: found host: ", host);

    return getHappReleasesFromHost(appWebsocket, appStoreApp, host, forHapp);
  } catch (e) {
    if (e instanceof AggregateError) {
      return Promise.reject(`No available peer host found.`);
    }
    console.error(`Failed to get happ releases: ${JSON.stringify(e)}`);
    return Promise.reject(`Failed to get happ releases: ${JSON.stringify(e)}`);
  }
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

    const happReleaseEntities: DevHubResponse<DevHubResponse<Array<Entity<HappReleaseEntry>>>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    // maybe it needs to be entity.payload.content instead...
    return happReleaseEntities.payload.payload;
  }
}


/**
 * Fetching GuiRelease Entry from a DevHub host
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param guiReleaseEntryHash
 */
export async function fetchGuiReleaseEntry(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  guiReleaseEntryHash: EntryHash,
) {

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);
  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    console.log("@fetchGuiReleaseEntry: trying to fetch GUI release entry...");

    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      "happ_library",
      "get_gui_release",
    );

    console.log("@fetchGuiReleaseEntry: got host: ", encodeHashToBase64(host));

    const input: CustomRemoteCallInput = {
      host,
      call: {
        dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
        zome: "happ_library",
        function: "get_gui_release",
        payload: {
          id: guiReleaseEntryHash,
        },
      }
    }


    const guiReleaseEntryResponse: DevHubResponse<DevHubResponse<Entity<GUIReleaseEntry>>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });


    return guiReleaseEntryResponse.payload.payload;
  }
}


/**
 * Fetching the webhapp bytes from a DevHub host.
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param name
 * @param happReleaseEntryHash
 * @param guiReleaseEntryHash
 * @returns
 */
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

    console.log("@fetchWebHapp: trying to fetch webhapp...");

    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      "happ_library",
      "get_webhapp_package",
    );

    console.log("@fetchWebHapp: got host: ", encodeHashToBase64(host));

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

    console.log("@fetchWebHapp: calling portal API with payload: ", input);

    const webHappBytesResponse: DevHubResponse<DevHubResponse<Uint8Array>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    return webHappBytesResponse.payload.payload;
  }
}


export async function fetchGui(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  webAssetEntryHash: EntryHash,
): Promise<Uint8Array | undefined> {

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

    const response: DevHubResponse<DevHubResponse<Entity<FilePackage>>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    if (response.type === "success") {
      if (response.payload.type === "success") {
        return response.payload.payload.content.bytes;
      } else {
        return Promise.reject(`Failed to fetch UI: ${response.payload.payload}`);
      }
    } else {
      return Promise.reject(`Failed to fetch UI: ${response.payload}`);
    }
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

    const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

    if (!portalCell) {
      return Promise.reject("Failed to get hosts for zome function: portal cell not found.");
    }

    try {
      const hosts = await getHostsForZomeFunction(appWebsocket, appStoreApp, zome_name, fn_name)

      // 2. ping each of them and take the first one that responds
      try {
        const availableHost = await Promise.any(hosts.map(async (hostEntry) => {
          const hostPubKey = hostEntry.author;
          console.log("@getAvailableHostForZomeFunction: trying to ping host: ", encodeHashToBase64(hostPubKey));

          try{
            const result: Response<any> = await appWebsocket.callZome({
              fn_name: "ping",
              zome_name: "portal_api",
              cell_id: getCellId(portalCell)!,
              payload: hostPubKey,
              provenance: getCellId(portalCell)![1],
            });

            console.log("@getAvailableHostForZomeFunction Sent ping to host and got result: ", result);

            if (result.type === "failure") {
              return Promise.reject(`Failed to ping host: ${result.payload}`);
            }
          } catch (e) {
            console.error("Failed to ping host: ", e);
            console.log("Failed to ping host: stringified error: ", JSON.stringify(e));
            return Promise.reject("Failed to ping host.");
          }
          // what happens in the "false" case? Can this happen?


          return hostPubKey;
        }))

        return availableHost;
      } catch (e) {
        return Promise.reject("No available peer host found.")
      }

    } catch (e) {
      return Promise.reject(`Failed to get available host for zome ${zome_name} and function ${fn_name}: ${e}`);
    }
}



export async function getVisibleHostsForZomeFunction(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  zome_name: string,
  fn_name: string,
): Promise<HostAvailability> {

    const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

    if (!portalCell) {
      return Promise.reject("Failed to get hosts for zome function: portal cell not found.");
    }

    let responded = 0;

    const pingTimestamp = Date.now();

    try {
      const hosts = await getHostsForZomeFunction(appWebsocket, appStoreApp, zome_name, fn_name)

      // 2. ping each of them and take the first one that responds
      await Promise.allSettled(hosts.map(async (hostEntry) => {

        try {
          // consider hosts that do not respond after 6 seconds as offline
          const result = await appWebsocket.callZome({
            fn_name: "ping",
            zome_name: "portal_api",
            cell_id: getCellId(portalCell)!,
            payload: hostEntry.author,
            provenance: getCellId(portalCell)![1],
          }, 6000);

          if (result.type === "failure") {
              return Promise.reject(`Failed to ping host: ${result.payload}`);
          }

          responded += 1;

        } catch (e) {
          return Promise.reject(`Failed to ping host: ${e}`);
        };

      }))

      console.log("@getVisibleHostsForZomeFunction: all Promises settled: result: ", {
        responded,
        totalHosts: hosts.length,
        pingTimestamp,
      });

      return {
        responded,
        totalHosts: hosts.length,
        pingTimestamp,
      };

    } catch (e) {
      return Promise.reject(`Failed to get visible hosts for zome ${zome_name} and function ${fn_name}: ${e}`);
    }
}


/**
 * Gets all the hosts that registered to grant remote calls for the given zome function
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param zome_name
 * @param fn_name
 * @returns
 */
export async function getHostsForZomeFunction(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  zome_name: string,
  fn_name: string,
): Promise<Array<HostEntry>> {

  // console.log("@getHostsForZomeFunction: being called...");
  // console.log("@getHostsForZomeFunction: appStoreApp: ", appStoreApp);


  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

  // console.log("@getHostsForZomeFunction: appstoreCell", appstoreCell);
  // console.log("@getHostsForZomeFunction: portalCell", portalCell);

  if (!portalCell) {
    return Promise.reject("Failed to get hosts for zome function: portal cell not found.");
  } else {
    console.log("@getHostsForZomeFunction: searching hosts.");

    const registeredHosts: DevHubResponse<Array<Entity<HostEntry>>> = await appWebsocket.callZome({
      fn_name: "get_registered_hosts",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: {
        dna: DEVHUB_HAPP_LIBRARY_DNA_HASH,
      },
      provenance: getCellId(portalCell)![1],
    });

    // console.log("Registered Hosts overall: ", registeredHosts);

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

    // console.log("@getHostsForZomeFunction: found hosts: ", hosts);
    let b64Hosts = hosts.payload.map((entity) => encodeHashToBase64(entity.content.author));
    console.log("@getHostsForZomeFunction: b64 hosts: ", b64Hosts);

    if (hosts.payload.length === 0) {
      return Promise.reject(`Found no registered hosts for zome ${zome_name} and function ${fn_name}.`);
    }

    return hosts.payload.map((host) => host.content);
  }
}




/**
 * Gets bytes from the mere_memory zome
 *
 * @param appWebsocket
 * @param appStoreApp
 * @returns
 */
export async function collectBytes(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  entryHash: EntryHash,
): Promise<Uint8Array> {

  console.log("@collectBytes");
  const appstoreCell = appStoreApp.cell_info["appstore"].find((c) => "provisioned" in c);

  console.log("@collectBytes: appstoreCell", appstoreCell);

  if (!appstoreCell) {
    throw new Error("appstore cell not found.")
  } else {

    const response: Response<MemoryEntry> = await appWebsocket.callZome({
      fn_name: "get_memory",
      zome_name: "mere_memory_api",
      cell_id: getCellId(appstoreCell)!,
      payload: entryHash,
      provenance: getCellId(appstoreCell)![1],
    })

    console.log("@collectBytes RECEIVED RESPONSE: ", response);

    if (!(response.type === "success")) {
      return Promise.reject(`Failed to get MemoryEntry: ${response.payload}`)
    }

    let memoryEntry = response.payload;
    let blockAddresses = memoryEntry.block_addresses;

    let chunks: Array<MemoryBlockEntry> = [];
    // for each block address get the bytes
    try {
      await Promise.all(blockAddresses.map(async (entryHash) => {
        const memoryBlockResponse: Response<MemoryBlockEntry> = await appWebsocket.callZome({
          fn_name: "get_memory_block",
          zome_name: "mere_memory_api",
          cell_id: getCellId(appstoreCell)!,
          payload: entryHash,
          provenance: getCellId(appstoreCell)![1],
        })

        if (!(memoryBlockResponse.type === "success")) {
          return Promise.reject(`Failed to get MemoryBlockEntry: ${response.payload}`)
        }

        chunks.push(memoryBlockResponse.payload)

      }))

      // sort chunks and plug them together to one array
      chunks.sort((a, b) => a.sequence.position - b.sequence.position);

      let combinedBytes: Array<number> = [];
      chunks.forEach((chunk) => combinedBytes = [...combinedBytes, ...chunk.bytes]);

      return Uint8Array.from(combinedBytes);

    } catch (e) {
      return Promise.reject(`Failed to collect bytes from mere_memory zome: ${e}`);
    }


    // console.log("@getImage: image", image);


    // return allApps.payload.map((appEntity) => appEntity.content)
  }

}