import { AgentPubKey, EntryHash } from "@holochain/client";

export interface DeprecationNotice {
  message: string;

  // optional
  recommended_alternatives: Array<EntryHash> | undefined;
}

export interface Happ {
  title: string;
  subtitle: string;
  description: string;
  designer: AgentPubKey;
  published_at: number;
  last_updated: number;
  metadata: Record<string, any>;

  // optional
  tags: Array<string> | undefined;
  icon: Uint8Array | undefined;
  deprecation: DeprecationNotice | undefined;
}

export interface HappRelease {
  name: string;
  description: string;
  for_happ: EntryHash;
  published_at: number;
  last_updated: number;
  manifest: any;
  dna_hash: string;
  hdk_version: string;
  dnas: Array<any>;
  gui: any | undefined;
  metadata: Record<string, any>;
}
