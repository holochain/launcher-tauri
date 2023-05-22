import { DnaHash, DnaHashB64, decodeHashFromBase64 } from "@holochain/client";


export const APPSTORE_APP_ID = "AppStore";
export const DEVHUB_APP_ID = "DevHub";

// hard coded dna hash of the DevHub in use

// production environment hash

// export const DEVHUB_HAPP_LIBRARY_DNA_HASH_B64: DnaHashB64 = "uhC0kOkT0l0srUiAq-LUywFMjIfD4Kx1pzBtGtDMu418KZLGytXZz";
export const DEVHUB_HAPP_LIBRARY_DNA_HASH_B64: DnaHashB64 = "uhC0ktLhtxeQNXCDFL9dMazx5PkgBQVoK-5-XZk7ISPCpW8E9aT9w";
export const DEVHUB_HAPP_LIBRARY_DNA_HASH: DnaHash = decodeHashFromBase64(DEVHUB_HAPP_LIBRARY_DNA_HASH_B64);

// dev environment hash:


