import { ActionHash, AgentPubKey, DnaHash, EntryHash, ZomeCallCapGrant } from "@holochain/client"


export interface Entity<T> {
  id: ActionHash,
  action: ActionHash,
  address: EntryHash,
  ctype: string,
  content: T,
}

export interface DevHubResponse<T> {
  metadata: any,
  payload: T,
}

export type EntityId = ActionHash;

export interface AppEntry {
  name: string,
  description: string,
  icon: EntryHash,
  publisher: ActionHash, // alias EntityId
  devhub_address: WebHappConfig,
  editors: Array<AgentPubKey>,

  author: AgentPubKey,
  published_at: number,
  last_updated: number,
  metadata: any,
}


export interface HostEntry {
  dna: DnaHash,
  capabilities: ZomeCallCapGrant,
  author: AgentPubKey,
  published_at: number,
  last_updated: number,
  metadata: any,
}


export interface WebHappConfig {
  dna: DnaHash,
  happ: EntryHash,
  gui: EntryHash | undefined,
}


export interface CustomRemoteCallInput {
  host: AgentPubKey,
  call: RemoteCallInput,
}

export interface RemoteCallInput {
  dna: DnaHash,
  zome: string,
  function: string,
  payload: any,
}

export interface GetWebHappPackageInput {
  name: string,
  happ_release_id: EntryHash,
  gui_release_id: EntryHash,
}


export interface HappReleaseEntry {
  name: string,
  description: string,
  for_happ: EntryHash,
  ordering: number,
  published_at: number,
  last_updated: number,
  manifest: any,
  dna_hash : string,
  hdk_version: string,
  dnas: Array<DnaReference>,
  metadata: Record<string, any>, // BTreeMap<String, serde_yaml::Value> in Rust

  // Optional fields
  official_gui: EntryHash | undefined,
}

export interface DnaReference {
  role_name: string,
  dna : EntryHash, // Dna ID
  version : EntryHash, // Version ID
  wasm_hash : string,
}
