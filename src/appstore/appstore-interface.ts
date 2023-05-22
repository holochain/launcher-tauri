import { AgentPubKey, AppInfo, AppWebsocket, DnaHash, encodeHashToBase64, EntryHash } from "@holochain/client";
import { getCellId } from "../utils";
import { AppEntry, CustomRemoteCallInput, DevHubResponse, Entity, GetWebHappPackageInput, HappReleaseEntry, HostEntry, Response, MemoryEntry, MemoryBlockEntry, GUIReleaseEntry, FilePackage, HostAvailability, PublisherEntry } from "./types";
import { ResourceLocator } from "../types";



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
 * IMPORTANT: EntryHashes all need to be part of the same DHT
 *
 * @param happReleaseEntryHash
 */
export async function getHappReleasesByEntryHashes(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  happReleaseEntryHashes: Array<EntryHash | undefined>
): Promise<Array<HappReleaseEntry | undefined>> {

  console.log("@getHappReleasesByEntryHashes: getting happ releases by entry hashes");
  console.log("@getHappReleasesByEntryHashes: devhubDna: ", encodeHashToBase64(devhubDna));
  // Find an online host
  const host: AgentPubKey = await getAvailableHostForZomeFunction(
    appWebsocket,
    appStoreApp,
    devhubDna,
    "happ_library",
    "get_happ_releases",
  );

  console.log("@getHappReleasesByEntryHashes: found host: ", host);


  // make zome calls for each EntryHash to this one host
  const happReleases = await Promise.all(happReleaseEntryHashes.map( async (entryHash) => {
    if (entryHash) {
      return getHappReleaseFromHost(appWebsocket, appStoreApp, devhubDna, host, entryHash);
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
 * Get the the HappEntry of an entry hash from a specific DevHub host
 * @param appWebsocket
 * @param appStoreApp
 * @param host
 * @param entryHash
 */
async function getHappEntryFromHost (
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  host: AgentPubKey,
  entryHash: EntryHash, // EntryHash of the HappEntry
): Promise<Entity<HappReleaseEntry>> {

  const payload = {
    id: entryHash,
  };

  return remoteCallToDevHubHost<Entity<HappReleaseEntry>>(
    appWebsocket,
    appStoreApp,
    devhubDna,
    host,
    "happ_library",
    "get_happ",
    payload
  );
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
  devhubDna: DnaHash,
  host: AgentPubKey,
  entryHash: EntryHash, // EntryHash of the HappReleaseEntry
): Promise<Entity<HappReleaseEntry>> {

  const payload = {
    id: entryHash,
  };

  return remoteCallToDevHubHost<Entity<HappReleaseEntry>>(
    appWebsocket,
    appStoreApp,
    devhubDna,
    host,
    "happ_library",
    "get_happ_release",
    payload
  );
}


/**
 * Get happ releases for a happ. Searches for an online DevHub host first.
 *
 * ATTENTION: This returns just an empty array if there is no happ entry found at
 * the requested entry hash
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param forHapp
 * @returns
 */
export async function getHappReleases(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  forHapp: ResourceLocator,
): Promise<Array<Entity<HappReleaseEntry>>> {

  // console.log("@getHappReleases: trying to get host.");

  try {
    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      forHapp.dna_hash,
      "happ_library",
      "get_happ_releases",
    );

    console.log("@getHappReleases: found host: ", encodeHashToBase64(host), host);

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
export async function getHappReleasesFromHost (
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  host: AgentPubKey,
  forHapp: ResourceLocator,
): Promise<Array<Entity<HappReleaseEntry>>> {

  const payload = {
    for_happ: forHapp.resource_hash,
  };

  return remoteCallToDevHubHost<Array<Entity<HappReleaseEntry>>>(
    appWebsocket,
    appStoreApp,
    forHapp.dna_hash,
    host,
    "happ_library",
    "get_happ_releases",
    payload,
  );
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
  guiReleaseLocator: ResourceLocator,
) {

  const payload = {
    id: guiReleaseLocator.resource_hash,
  };

  return remoteCallToAvailableHost<Entity<GUIReleaseEntry>>(
    appWebsocket,
    appStoreApp,
    guiReleaseLocator.dna_hash,
    "happ_library",
    "get_gui_release",
    payload,
  );
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
  happReleaseLocator: ResourceLocator,
  guiReleaseLocator: ResourceLocator,
): Promise<Uint8Array> {

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);
  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    const host: AgentPubKey = await getAvailableHostForZomeFunction(
      appWebsocket,
      appStoreApp,
      happReleaseLocator.dna_hash,
      "happ_library",
      "get_webhapp_package",
    );

    const payload: GetWebHappPackageInput = {
      name,
      happ_release_id: happReleaseLocator.resource_hash,
      gui_release_id: guiReleaseLocator.resource_hash,
    };

    return remoteCallToDevHubHost<Uint8Array>(
      appWebsocket,
      appStoreApp,
      happReleaseLocator.dna_hash,
      host,
      "happ_library",
      "get_webhapp_package",
      payload
    );
  }
}


export async function fetchGui(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  guiReleaseHash: EntryHash,
): Promise<Entity<FilePackage>> {

  const host: AgentPubKey = await getAvailableHostForZomeFunction(
    appWebsocket,
    appStoreApp,
    devhubDna,
    "happ_library",
    "get_webasset",
  );

  const payload = {
    id: guiReleaseHash,
  };

  return remoteCallToDevHubHost<Entity<FilePackage>>(
    appWebsocket,
    appStoreApp,
    devhubDna,
    host,
    "happ_library",
    "get_webasset",
    payload
  );
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
  devhubDna: DnaHash,
  zome_name: string,
  fn_name: string,
): Promise<AgentPubKey> {

    const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

    if (!portalCell) {
      return Promise.reject("Failed to get hosts for zome function: portal cell not found.");
    }

    try {
      const hosts = await getHostsForZomeFunction(appWebsocket, appStoreApp, devhubDna, zome_name, fn_name)

      // 2. ping each of them and take the first one that responds
      try {
        const availableHost = await Promise.any(hosts.map(async (hostEntry) => {
          const hostPubKey = hostEntry.author;
          // console.log("@getAvailableHostForZomeFunction: trying to ping host: ", encodeHashToBase64(hostPubKey));

          try{
            const result: Response<any> = await appWebsocket.callZome({
              fn_name: "ping",
              zome_name: "portal_api",
              cell_id: getCellId(portalCell)!,
              payload: hostPubKey,
              provenance: getCellId(portalCell)![1],
            });

            if (result.type === "failure") {
              return Promise.reject(`Failed to ping host: ${result.payload}`);
            }
          } catch (e) {
            // console.error("Failed to ping host: ", e);
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
      return Promise.reject(`Failed to get available host for zome ${zome_name} and function ${fn_name}: ${JSON.stringify(e)}`);
    }
}



export async function getVisibleHostsForZomeFunction(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  zome_name: string,
  fn_name: string,
  timeoutMs: number = 4000,
): Promise<HostAvailability> {

    const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);

    if (!portalCell) {
      return Promise.reject("Failed to get hosts for zome function: portal cell not found.");
    }

    let responded: AgentPubKey[] = [];

    const pingTimestamp = Date.now();

    try {

      const hosts = await getHostsForZomeFunction(appWebsocket, appStoreApp, devhubDna, zome_name, fn_name)

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
          }, timeoutMs);

          if (result.type === "failure") {
              return Promise.reject(`Failed to ping host: ${result.payload}`);
          }

          responded.push(hostEntry.author)

        } catch (e) {
          return Promise.reject(`Failed to ping host: ${e}`);
        };

      }))

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
  devhubDna: DnaHash,
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
    // console.log("@getHostsForZomeFunction: searching hosts.");

    // 1. get all registered hosts for this zome function
    const hosts: DevHubResponse<Array<Entity<HostEntry>>> = await appWebsocket.callZome({
      fn_name: "get_hosts_for_zome_function",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: {
        dna: devhubDna,
        zome: zome_name,
        function: fn_name,
      },
      provenance: getCellId(portalCell)![1],
    })

    // console.log("@getHostsForZomeFunction: found hosts: ", hosts);
    // let b64Hosts = hosts.payload.map((entity) => encodeHashToBase64(entity.content.author));
    // console.log("@getHostsForZomeFunction: b64 hosts: ", b64Hosts);

    // if (hosts.payload.length === 0) {
    //   return Promise.reject(`Found no registered hosts for zome ${zome_name} and function ${fn_name}.`);
    // }

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


/**
 * Makes a remote call to a specified DevHub host for the specified function and zome and with the
 * specified payload
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param devhubDna
 * @param host
 * @param fn_name
 * @param zome_name
 * @param payload
 * @returns
 */
export async function remoteCallToDevHubHost<T>(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  host: AgentPubKey, // public key of the devhub host to call to
  zome_name: string,
  fn_name: string,
  payload: any,
): Promise<T> {

  const portalCell = appStoreApp.cell_info["portal"].find((c) => "provisioned" in c);
  if (!portalCell) {
    throw new Error("portal cell not found.")
  } else {

    const input: CustomRemoteCallInput = {
      host,
      call: {
        dna: devhubDna,
        zome: zome_name,
        function: fn_name,
        payload,
      }
    }

    const response: DevHubResponse<DevHubResponse<T>> = await appWebsocket.callZome({
      fn_name: "custom_remote_call",
      zome_name: "portal_api",
      cell_id: getCellId(portalCell)!,
      payload: input,
      provenance: getCellId(portalCell)![1],
    });

    if (response.type === "success") {
      if (response.payload.type === "success") {
        return response.payload.payload;
      } else {
        return Promise.reject(`remote call for function '${fn_name}' of zome '${zome_name}' failed: ${JSON.stringify(response.payload.payload)}`);
      }
    } else {
      return Promise.reject(`remote call for function '${fn_name}' of zome '${zome_name}' failed: ${JSON.stringify(response.payload)}`);
    }
  }
}



/**
 * Helper function to make a remote call to first responsive host. It is possible
 * that this host does not have the app synchronized and thus is unable to deliver it.
 * In that case, other hosts should be tried which is not supported by this function.
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param devhubDna
 * @param zome_name
 * @param fn_name
 * @param payload
 */
export async function remoteCallToAvailableHost<T>(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  zome_name: string,
  fn_name: string,
  payload: any,
): Promise<T> {

  const host: AgentPubKey = await getAvailableHostForZomeFunction(
    appWebsocket,
    appStoreApp,
    devhubDna,
    zome_name,
    fn_name,
  );

  return remoteCallToDevHubHost<T>(
    appWebsocket,
    appStoreApp,
    devhubDna,
    host,
    zome_name,
    fn_name,
    payload
  );
}



/**
 * Helper function to make a remote call to hosts in a cascading manner, i.e. if the first
 * responsive host fails to deliver the promise, go on to proceeding hosts etc.
 *
 * WARNING: Untested.
 *
 * @param appWebsocket
 * @param appStoreApp
 * @param devhubDna
 * @param zome_name
 * @param fn_name
 * @param payload
 */
export async function remoteCallCascadeToAvailableHosts<T>(
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  zome_name: string,
  fn_name: string,
  payload: any,
  pingTimeout: number = 3000, // hosts that do not respond to the ping quicker than this are ignored
): Promise<T> {

  const pingResult = await getVisibleHostsForZomeFunction(
    appWebsocket,
    appStoreApp,
    devhubDna,
    zome_name,
    fn_name,
    pingTimeout,
  );

  const availableHosts = pingResult.responded;

  let result: T | undefined = undefined;

  let errors = [];

  // for each host, try to get stuff and if it succeeded, return,
  // otherwise go to next host
  for (const host of availableHosts) {
    try {
      const response = await remoteCallToDevHubHost<T>(
        appWebsocket,
        appStoreApp,
        devhubDna,
        host,
        zome_name,
        fn_name,
        payload,
      );

      return response;

    } catch (e) {
      errors.push(e);
    }
  }

  return Promise.reject(`Failed to do remote call for function '${fn_name}' of zome '${zome_name}' for all available hosts.\nErrors: ${errors}`);

}


export async function tryWithHosts<T>(
  fn: (host: AgentPubKey) => T,
  appWebsocket: AppWebsocket,
  appStoreApp: AppInfo,
  devhubDna: DnaHash,
  zome_name: string,
  fn_name: string,
  pingTimeout: number = 3000,
  ): Promise<T>{

  // try with first responding host
  const host: AgentPubKey = await getAvailableHostForZomeFunction(
    appWebsocket,
    appStoreApp,
    devhubDna,
    zome_name,
    fn_name,
  );

  try {
    // console.log("@tryWithHosts: trying with first responding host: ", encodeHashToBase64(host));
    const result = await fn(host);
    return result;
  } catch (e) {
    // console.log("@tryWithHosts: Failed with first host: ", JSON.stringify(e));
    // if it fails with the first host, try other hosts
    const pingResult = await getVisibleHostsForZomeFunction(
      appWebsocket,
      appStoreApp,
      devhubDna,
      zome_name,
      fn_name,
      pingTimeout,
    );

    const availableHosts = pingResult.responded;

    // console.log("@tryWithHosts: other available hosts: ", availableHosts.map((hash) => encodeHashToBase64(hash)));

    let result: T | undefined = undefined;

    let errors = [];

    // for each host, try to get stuff and if it succeeded, return,
    // otherwise go to next host
    for (const otherHost of availableHosts) {
      try {
        // console.log("@tryWithHosts: retrying with other host: ", encodeHashToBase64(otherHost));
        const response = await fn(otherHost);
        return response;
      } catch (e) {
        errors.push(e);
      }
    }

    return Promise.reject(`Callback for function '${fn_name}' of zome '${zome_name}' failed for all available hosts.\nErrors: ${JSON.stringify(errors)}`);

  }

}




