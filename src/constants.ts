import { DnaHash, DnaHashB64, decodeHashFromBase64 } from "@holochain/client";


export const APPSTORE_APP_ID = "AppStore";
export const DEVHUB_APP_ID = "DevHub";

// hard coded dna hash of the DevHub in use

// production environment hash
export const DEVHUB_HAPP_LIBRARY_DNA_HASH_B64: DnaHashB64 = "uhC0kco40cQ_lAEoz_rNB8ruwhPlAlmzLpL5H18dEohyUSXldWSiX";
export const DEVHUB_HAPP_LIBRARY_DNA_HASH: DnaHash = decodeHashFromBase64(DEVHUB_HAPP_LIBRARY_DNA_HASH_B64);

// dev environment hash:
// export const DEVHUB_HAPP_LIBRARY_DNA_HASH_B64: DnaHashB64 = "uhC0k4_LyUyi9kuDIVdmCtHoaeIcVuMYb4ZYRq982qYnzmxMkxKyN";


