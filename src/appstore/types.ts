import {
  ActionHash,
  AgentPubKey,
  DnaHash,
  EntryHash,
  ZomeCallCapGrant,
} from "@holochain/client";

export interface Entity<T> {
  id: ActionHash;
  action: ActionHash;
  address: EntryHash;
  ctype: string;
  content: T;
}

export interface DevHubResponse<T> {
  type: "success" | "failure";
  metadata: any;
  payload: T;
}

export type EntityId = ActionHash;

export interface AppEntry {
  title: string;
  subtitle: string;
  description: string;
  icon: EntryHash;
  publisher: ActionHash; // alias EntityId
  devhub_address: WebHappConfig;
  editors: Array<AgentPubKey>;

  author: AgentPubKey;
  published_at: number;
  last_updated: number;
  metadata: any;
}

export interface HostEntry {
  dna: DnaHash;
  capabilities: ZomeCallCapGrant;
  author: AgentPubKey;
  published_at: number;
  last_updated: number;
  metadata: any;
}

export interface PublisherEntry {
  name: string;
  location: LocationTriplet;
  website: WebAddress;
  icon: EntryHash;
  editors: Array<AgentPubKey>;

  // common fields
  author: AgentPubKey;
  published_at: number;
  last_updated: number;
  metadata: any;

  // optional
  description: string | undefined;
  email: string | undefined;
  deprecation: any;
}

export interface WebAddress {
  url: string;
  context: string | undefined;
}

export interface LocationTriplet {
  country: string;
  region: string;
  city: string;
}

export interface WebHappConfig {
  dna: DnaHash;
  happ: EntryHash;
  gui: EntryHash | undefined;
}

export interface CustomRemoteCallInput {
  host: AgentPubKey;
  call: RemoteCallInput;
}

export interface RemoteCallInput {
  dna: DnaHash;
  zome: string;
  function: string;
  payload: any;
}

export interface GetWebHappPackageInput {
  name: string;
  happ_release_id: EntryHash;
  gui_release_id: EntryHash;
}

export interface HappReleaseEntry {
  version: string;
  description: string;
  for_happ: EntryHash;
  ordering: number;
  published_at: number;
  last_updated: number;
  manifest: any;
  dna_hash: string;
  hdk_version: string;
  dnas: Array<DnaReference>;
  metadata: Record<string, any>; // BTreeMap<String, serde_yaml::Value> in Rust

  // Optional fields
  official_gui: EntryHash | undefined;
}

export interface DnaReference {
  role_name: string;
  dna: EntryHash; // Dna ID
  version: EntryHash; // Version ID
  wasm_hash: string;
}

export interface GUIReleaseEntry {
  version: string;
  changelog: string;
  for_gui: EntryHash;
  for_happ_releases: Array<EntryHash>;
  web_asset_id: EntryHash;
  published_at: number;
  last_updated: number;
  metadata: Record<string, any>;

  // Optional fields
  screenshots: Array<EntryHash> | undefined;
  // pub dna_support: Option<Vec<EntryHash>>, // list of DnaEntry IDs of intended support, does not mean they are guaranteed to work for all those DNA's versions
}

export interface Response<T> {
  type: "success" | "failure";
  payload: T;
}

export interface MemoryEntry {
  author: AgentPubKey;
  published_at: number;
  hash: string;
  memory_size: number;
  block_addresses: Array<EntryHash>;
}

export interface MemoryBlockEntry {
  sequence: SequencePosition;
  bytes: Uint8Array;
}

export interface SequencePosition {
  position: number;
  length: number;
}

export interface FilePackage {
  author: AgentPubKey;
  published_at: number;
  last_updated: number;
  file_size: number;
  mere_memory_addr: EntryHash;
  mere_memory_hash: String;

  // optional
  bytes: Uint8Array | undefined;
  name: string | undefined;
  metadata: any;
}

export interface HostAvailability {
  responded: AgentPubKey[];
  totalHosts: number;
  pingTimestamp: number;
}
