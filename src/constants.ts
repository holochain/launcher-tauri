import { DnaHash, DnaHashB64, decodeHashFromBase64 } from "@holochain/client";




export const APP_STORE_ID = "Appstore";

// hard coded dna hash of the DevHub in use
export const DEVHUB_HAPP_LIBRARY_DNA_HASH_B64: DnaHashB64 = "uhC0kenEh_slR59FCLNSzhsO3KxphMNy3Be30pBd454jyRLr6IsHY";
export const DEVHUB_HAPP_LIBRARY_DNA_HASH: DnaHash = decodeHashFromBase64(DEVHUB_HAPP_LIBRARY_DNA_HASH_B64);




