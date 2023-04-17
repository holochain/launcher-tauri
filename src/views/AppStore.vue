<template>
  <HCLoading ref="downloading" :text="loadingText" />

  <HCDialog ref="appLibraryFirstEnter">
    <div
      class="column"
      style="padding: 30px; align-items: center; max-width: 600px"
    >
      <div style="font-weight: 600; font-size: 27px; margin-bottom: 25px">
        Note
      </div>
      <div>
        Holochain is <span @click="peerToPeer" title="https://developer.holochain.org/glossary/#peer-to-peer" style="color: #0000EE; text-decoration: underline; cursor: pointer;">peer-to-peer</span>
         and the <b>App Library is not optimized for download speed yet.</b><br><br>
        Before you can download your first app, <b>the App Library needs to get synchronized with other peers in the background.</b>
        This can take up to 10-20 Minutes depending on the number and size of apps available and the bandwidth of your internet connection.
        <br><br>
        You can see the progress of ongoing App Library Synchronizations in the bottom right corner.
      </div>

      <HCButton style="margin-top: 20px; width: 100px;" @click="closeNote">Ok</HCButton>
    </div>

  </HCDialog>


  <div class="row center-content top-bar" style="position: sticky; top: 0; z-index: 1; width: 100%;">
    <mwc-icon-button
      icon="arrow_back"
      @click="$emit('go-back')"
      :title="$t('buttons.back')"
      style="margin-left: 8px;"
    ></mwc-icon-button>

    <span
      style="
        flex: 1;
        font-size: 1.5em;
        margin-left: 4px;
      "
      >{{ $t("appStore.appStore") }}</span
    >
    <HCButton
      outlined
      @click="howToPublish()"
      style="height: 36px; border-radius: 8px; padding: 0 20px"
      :title="howToPublishUrl"
      >{{ $t("appStore.howToPublishAnApp") }}
    </HCButton>
    <HCButton
      style="
        margin-left: 8px;
        margin-right: 8px;
        height: 40px;
        border-radius: 8px;
        padding: 0 20px;
      "
      @click="selectFromFileSystem()"
    >
      <div class="row center-content">
        <mwc-icon>folder</mwc-icon>
        <span style="margin-left: 5px">{{
          $t("appStore.selectAppFromFileSystem")
        }}</span>
      </div>
    </HCButton>
  </div>


  <div v-if="loading" class="column center-content" style="flex: 1; min-height: calc(100vh - 64px);">
    <LoadingDots style="--radius: 15px; --dim-color: #e8e8eb; --fill-color: #b5b5b5;"></LoadingDots>
  </div>

  <div
    v-else-if="installableApps.length === 0"
    class="column center-content"
    style="flex: 1; min-height: calc(100vh - 64px);"
  >
    <span>{{ $t("appStore.noAppsInStore") }}</span>
    <span style="margin-top: 8px"
      ><span
        style="cursor: pointer; text-decoration: underline;"
        :title="howToPublishUrl"
        @click="howToPublish()"
        @keydown.enter="howToPublish()"
        tabindex="0"
        >{{ $t("appStore.readThis") }}</span
      >
      {{ $t("appStore.readThisToPublish") }}</span
    >
    <HCButton
      outlined
      @click="fetchApps()"
      class="refresh-button"
      >{{ $t("main.refresh") }}
    </HCButton>
  </div>

  <div v-else class="row" style="flex-wrap: wrap; margin: 16px; min-height: calc(100vh - 64px); margin-bottom: 200px;">
    <div
      v-for="(app, i) of installableApps"
      :key="i"
      class="column"
      style="margin-right: 16px; margin-bottom: 16px"
    >
      <AppPreviewCard :app="app" :appIcon="app.icon" @installApp="saveApp(app)" />
    </div>
  </div>

  <div class="progress-indicator" :class="{ highlighted: downloadFailed }">
    <div
      style="margin-bottom: 10px; font-weight: 600; margin-left: 10px"
      :title="$t('appStore.fullSynchronizationRequired')"
    >
      {{ $t('appStore.appLibrarySynchronization') }}:
    </div>
    <div>
      <div v-for="(cell, idx) in provisionedCells" :key="cell[0]" class="column">
        <div class="row" style="align-items: center">
          <div
            style="
              width: 20%;
              margin-left: 20px;
              font-size: 0.95em;
              text-align: right;
            "
          >
            {{ cell[0] }}
          </div>
          <div style="width: 50%; margin: 0 30px">
            <HCProgressBar
              v-if="(networkStates[idx] || networkStates[idx] === 0) && cachedMaxExpected[idx]"
              title="currently ongoing data exchanges with peers"
              :progress="progressRatio(idx)"
              :style="`--height: 10px; --hc-primary-color:${
                idleStates[idx] ? '#6B6B6B' : '#482edf'
              };`"
            />
            <span
              v-else
              style="
                opacity: 0.7;
                font-size: 0.8em;
                display: flex;
                justify-content: center;
              "
              title="currently ongoing data exchanges with peers"
            >
            {{ $t('appStore.noOngoingPeerSynchronization') }}</span
            >
          </div>
          <div
            :style="`width: 30%; text-align: center; ${
              idleStates[idx] ? 'opacity: 0.7;' : ''
            }`"
            title="received bytes | expected bytes"
          >
            {{ prettyBytesLocal(byteDiff(idx)) }}
            | <span :class="{ highglightedText: maxExceeded[idx] }">
            {{ prettyBytesLocal(cachedMaxExpected[idx]) }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>

  <InstallAppDialog
    v-if="selectedAppBundlePath"
    :appBundlePath="selectedAppBundlePath"
    :holochainSelection="holochainSelection"
    :happReleaseHash="selectedHappReleaseHash"
    :guiReleaseHash="selectedGuiReleaseHash"
    @app-installed="
      holochainSelection = true;
      installClosed();
      $emit('go-back');
    "
    @closing-dialog="installClosed()"
    @error="(e) => showError(e)"
    ref="install-app-dialog"
  ></InstallAppDialog>
  <HCSnackbar
    :labelText="errorText"
    ref="snackbar"
  ></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "@material/mwc-circular-progress";
import "@material/mwc-icon";
import "@material/mwc-icon-button";
import { AppWebsocket, NetworkInfo, CellInfo, EntryHashB64, encodeHashToBase64, AgentPubKey } from "@holochain/client";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { toSrc } from "../utils";

import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import HCProgressBar from "../components/subcomponents/HCProgressBar.vue";
import LoadingDots from "../components/subcomponents/LoadingDots.vue";

import { getHappReleases, fetchWebHapp, getAvailableHostForZomeFunction, DEVHUB_HAPP_LIBRARY_DNA_HASH } from "../appstore/appstore-interface";
import InstallAppDialog from "../components/InstallAppDialog.vue";
import HCButton from "../components/subcomponents/HCButton.vue";
import AppPreviewCard from "../components/AppPreviewCard.vue";
import HCLoading from "../components/subcomponents/HCLoading.vue";
import HCDialog from "../components/subcomponents/HCDialog.vue";

import { HolochainId } from "../types";
import prettyBytes from "pretty-bytes";
import { getCellId } from "../utils";
import { i18n } from "../locale";
import { AppEntry } from "../appstore/types";
import { getAllApps } from "../appstore/appstore-interface";

export default defineComponent({
  name: "AppStore",
  components: {
    InstallAppDialog,
    HCButton,
    AppPreviewCard,
    HCLoading,
    HCSnackbar,
    HCProgressBar,
    HCDialog,
    LoadingDots,
  },
  data(): {
    loadingText: string;
    loading: boolean;
    installableApps: Array<AppEntry>;
    selectedAppBundlePath: string | undefined;
    howToPublishUrl: string;
    holochainId: HolochainId | undefined;
    holochainSelection: boolean;
    pollInterval: number | null;
    provisionedCells: [string, CellInfo | undefined][] | undefined;
    networkStates: (number | undefined)[];
    cachedMaxExpected: (number | undefined)[];
    latestNetworkUpdates: number[]; // timestamps of the latest non-zero gossipInfo update
    idleStates: boolean[];
    maxExceeded: boolean[];
    showProgressIndicator: boolean;
    downloadFailed: boolean;
    errorText: string;
    appWebsocket: AppWebsocket | undefined;
    selectedHappReleaseHash: EntryHashB64 | undefined;
    selectedGuiReleaseHash: EntryHashB64 | undefined;
  } {
    return {
      loadingText: "",
      loading: true,
      installableApps: [],
      selectedAppBundlePath: undefined,
      howToPublishUrl:
        "https://github.com/holochain/launcher#publishing-and-updating-an-app-in-the-devhub",
      holochainId: undefined,
      holochainSelection: true,
      pollInterval: null,
      provisionedCells: undefined,
      networkStates: [undefined, undefined, undefined],
      cachedMaxExpected: [undefined, undefined, undefined],
      latestNetworkUpdates: [0, 0, 0],
      idleStates: [true, true, true],
      maxExceeded: [false, false, false],
      showProgressIndicator: false,
      downloadFailed: false,
      errorText: "Unknown error occured.",
      appWebsocket: undefined,
      selectedHappReleaseHash: undefined,
      selectedGuiReleaseHash: undefined,
    };
  },
  beforeUnmount() {
    window.clearInterval(this.pollInterval!);
  },
  async mounted() {

    if (!window.localStorage.getItem("appLibraryWarningShown")) {
      (this.$refs.appLibraryFirstEnter as typeof HCDialog).open();
      window.localStorage.setItem("appLibraryWarningShown", "true");
    }

    try {
      await this.fetchApps();
    } catch (e) {
      console.error(`Failed to fetch apps in mounted() hook: ${e}`);
    }

    // set up polling loop to periodically get gossip progress, global scope (window) seems to
    // be required to clear it again on beforeUnmount()
    // try {
    //   await this.getNetworkState();
    // } catch (e) {
    //   console.error(`Failed to get NetworkState: ${JSON.stringify(e)}`);
    // }

    // this.pollInterval = window.setInterval(
    //   async () => await this.getNetworkState(),
    //   2000
    // );
  },
  methods: {
    toSrc,
    async connectAppWebsocket() {
      // const _hdiOfDevhub = this.$store.getters["hdiOfDevhub"]; // currently not used
      const holochainId = this.$store.getters["holochainIdForDevhub"];
      this.holochainId = holochainId;
      // connect to AppWebsocket
      const port = this.$store.getters["appInterfacePort"](holochainId);
      this.appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      // console.log("connected to AppWebsocket.");
    },
    async fetchApps() {
      console.log("LOADING...");
      this.loading = true;

      console.log("@fetchApps: about to call appInfo...");
      if (!this.appWebsocket) {
        await this.connectAppWebsocket();
      }

      console.log("@fetchApps: about to call appInfo...");
      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: `Appstore`,
      });

      console.log("@fetchApps: appStoreInfo: ", appStoreInfo);
      const allCells = appStoreInfo.cell_info;
      console.log("@fetchApps: allCells: ", allCells);

      const provisionedCells: [string, CellInfo | undefined][] = Object.entries(allCells).map(([roleName, cellInfos]) => {
        return [roleName, cellInfos.find((cellInfo) => "provisioned" in cellInfo)]
      });

      console.log("@fetchApps: provisionedCells: ", provisionedCells);

      this.provisionedCells = provisionedCells.sort(([roleName_a, _cellInfo_a], [roleName_b, _cellInfo_b]) => {
        return roleName_a.localeCompare(roleName_b);
      });


      let allApps: Array<AppEntry>;
      try {
        allApps = await getAllApps((this.appWebsocket! as AppWebsocket), appStoreInfo);
      } catch (e) {
        console.error(`Error getting all apps: ${e}`);
        // Catch other errors than being offline
        allApps = [];
      }

      console.log("@fetchApps: allApps: ", allApps);

      this.installableApps = allApps;

      this.loading = false;

      // console.log("ALL APPS: ", allApps);
      // console.log("FILTERED APPS: ", this.installableApps);
      // console.log("hdk versions: ", hdk_versions);
    },
    async howToPublish() {
      await invoke("open_url_cmd", {
        url: this.howToPublishUrl,
      });
    },
    async peerToPeer() {
      await invoke("open_url_cmd", {
        url: "https://developer.holochain.org/glossary/#peer-to-peer",
      });
    },
    async saveApp(app: AppEntry) {
      // // if downloading, always take holochain version of DevHub
      this.holochainSelection = false;
      this.loadingText = "Fetching available releases";
      (this.$refs.downloading as typeof HCLoading).open();

      // 1. get happ releases for app from DevHub
      if (!this.appWebsocket) {
        await this.connectAppWebsocket();
      }

      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: `Appstore`,
      });

      let happReleases = undefined;

      try {
        happReleases = await getHappReleases(this.appWebsocket as AppWebsocket, appStoreInfo, app.devhub_address.happ)
      } catch (e) {
        this.errorText = `Error getting happ releases from a DevHub host. See console for details.`;
        (this.$refs as any).snackbar.show();
        (this.$refs.downloading as typeof HCLoading).close();
        throw new Error(`Error getting happ releases from a DevHub host: ${JSON.stringify(e)}`);
      }

      if (!happReleases) {
        this.errorText = "Undefined happ releases.";
        (this.$refs as any).snackbar.show();
        (this.$refs.downloading as typeof HCLoading).close();
        throw new Error("Undefined happ releases.");
      }

      // 1b. Filter out releases without GUIs for now. Installing headless apps should become possible as well of course
      happReleases = happReleases.filter((release) => !!release.content.official_gui);

      // 2. select latest happ release (later maybe option to select older ones)
      const latestHappRelease = happReleases.sort((a, b) => b.content.last_updated - a.content.last_updated)[0];

      // 3. fetchwebhapp

      const happReleaseHash = latestHappRelease.id;
      const guiReleaseHash = latestHappRelease.content.official_gui;

      let bytes = undefined;

      this.loadingText = "Searching available Host";

      const host: AgentPubKey = await getAvailableHostForZomeFunction(
        this.appWebsocket as AppWebsocket,
        appStoreInfo,
        "happ_library",
        "get_webhapp_package",
      );

      this.loadingText = "Requesting webhapp";

      try {
        this.selectedAppBundlePath = await invoke("fetch_and_save_app", {
          holochainId: this.holochainId,
          appstoreAppId: appStoreInfo.installed_app_id,
          appTitle: app.name,
          host: Array.from(host),
          devhubHappLibraryDnaHash: Array.from(DEVHUB_HAPP_LIBRARY_DNA_HASH), // DNA hash of the DevHub to which the remote call shall be made
          appstorePubKey: encodeHashToBase64(appStoreInfo.agent_pub_key),
          happReleaseHash: encodeHashToBase64(happReleaseHash),
          guiReleaseHash: guiReleaseHash ? encodeHashToBase64(guiReleaseHash) : undefined,
        });

        (this.$refs.downloading as typeof HCLoading).close();
        this.loadingText = "";

        this.$nextTick(() => {
          (this.$refs["install-app-dialog"] as typeof InstallAppDialog).open();
        });

        console.log("@saveApp: selectedAppBundlePath: ", this.selectedAppBundlePath);
      } catch (e) {
        console.error("Error fetching the webhapp from the DevHub host: ", e);
        this.errorText = "Failed to fetch webhapp from DevHub host.";
        (this.$refs as any).snackbar.show();
        (this.$refs.downloading as typeof HCLoading).close();
        return;
      }

      // try {
      //   bytes = await fetchWebHapp(
      //     this.appWebsocket! as AppWebsocket,
      //     appStoreInfo,
      //     app.name,
      //     happReleaseHash,
      //     guiReleaseHash!, // releases without official_gui have been filtered out earlier
      //   );
      // } catch (e) {
      //   console.error("Error fetching the webhapp: ", e);
      //   this.errorText = "Failed to fetch webhapp from DevHub host.";
      //   (this.$refs as any).snackbar.show();
      //   (this.$refs.downloading as typeof HCLoading).close();
      //   this.downloadFailed = true;
      //   setTimeout(() => (this.downloadFailed = false), 3000);
      //   return;
      // }

      // if (bytes) {
      //   try {
      //     this.selectedAppBundlePath = await invoke("save_app", {
      //       appBundleBytes: bytes,
      //     });
      //     // this.hdkVersionForApp = release.content.hdk_version;
      //     this.selectedHappReleaseHash = encodeHashToBase64(happReleaseHash);
      //     this.selectedGuiReleaseHash = encodeHashToBase64(guiReleaseHash!);
      //     (this.$refs.downloading as typeof HCLoading).close();
      //     this.loadingText = "";

      //     this.$nextTick(() => {
      //       (this.$refs["install-app-dialog"] as typeof InstallAppDialog).open();
      //     });
      //   } catch (e) {
      //     console.log("Error when decoding and saving webhapp to temp folder: ", e);
      //     console.log("Error Payload: ", (e as any).data);

      //     if ((e as any).data) {
      //       this.errorText = `Failed to decode and save webhapp: ${(e as any).data}`;
      //     } else {
      //       this.errorText = `Failed to decode and save webhapp: ${e}`;
      //     }

      //     (this.$refs as any).snackbar.show();
      //     (this.$refs.downloading as typeof HCLoading).close();
      //   }
      // } else {
      //   console.log("Error when decoding and saving webhapp to temp folder: Undefined bytes");
      //   this.errorText = `Failed to decode and save webhapp: Undefined bytes`;

      //   (this.$refs as any).snackbar.show();
      //   (this.$refs.downloading as typeof HCLoading).close();
      // }
    },
    async selectFromFileSystem() {
      this.selectedAppBundlePath = (await open({
        filters: [
          { name: "Holochain Application", extensions: ["webhapp", "happ"] },
        ],
      })) as string;

      this.$nextTick(() => {
        (this.$refs["install-app-dialog"] as typeof InstallAppDialog).open();
      });
    },
    installClosed() {
      this.selectedAppBundlePath = undefined;
      // this.hdkVersionForApp = undefined;
    },
    async getNetworkState() {
      if (!this.appWebsocket) {
        await this.connectAppWebsocket();
      }

      const networkInfo: NetworkInfo[] = await this.appWebsocket!.networkInfo({
        dnas: this.provisionedCells!.filter(([roleName, cellInfo]) => !!cellInfo)
          .map(([_roleName, cellInfo]) => getCellId(cellInfo!)![0] as Uint8Array),
      });

      networkInfo.forEach((info, idx) => {
        const expectedIncoming =
          info.fetch_pool_info.op_bytes_to_fetch;

        // In case expected incoming bytes are undefined, keep the chached values, otherwise update
        // expectedIncoming
      if (expectedIncoming || expectedIncoming === 0) {
          // if the expected incoming bytes are larger then the max cached value or there
          // is no cached max value, replace it
          const currentMax = this.cachedMaxExpected[idx];
          if ((!currentMax && currentMax !== 0) || expectedIncoming > currentMax) {
            this.cachedMaxExpected[idx] = expectedIncoming;
            this.maxExceeded[idx] = true;
            setTimeout(() => (this.maxExceeded[idx] = false), 500);
          }

          if (expectedIncoming != this.networkStates[idx]) {
            this.idleStates[idx] = false;
            this.latestNetworkUpdates[idx] = Date.now();
          }
          // make this call after setting max cached value to ensure it is always <= to it
          this.networkStates[idx] = expectedIncoming;
        }

        // if expected incoming remains the same for > 10 seconds, set to idle. Except expectedIncoming
        // is below 16MB, in this case transmission may already be finished.
        if (new Date().getTime() - this.latestNetworkUpdates[idx] > 10000) {
          if (this.networkStates[idx] || this.networkStates[idx] === 0) {
            if (this.networkStates[idx]! > 16000000) {
              this.idleStates[idx] = false
            }
          } else {
            this.idleStates[idx] = true;
          }
        }


        // if latest non-zero update to gossip progress is older than 80 seconds, set expected incoming
        // and max cached expected incoming to undefined again
        if (new Date().getTime() - this.latestNetworkUpdates[idx] > 80000) {
          this.networkStates[idx] = undefined;
          this.cachedMaxExpected[idx] = undefined;
        }
      });


    },
    showError(e: string) {
      this.errorText = e;
      (this.$refs as any).snackbar.show();
    },
    progressRatio(idx: number) {
      if ((this.networkStates[idx] || this.networkStates[idx] === 0) && this.cachedMaxExpected[idx]) {
        return (
          (1 - this.networkStates[idx]! / this.cachedMaxExpected[idx]!) * 100
        );
      } else {
        return undefined;
      }
    },
    prettyBytesLocal(input: number | undefined) {
      if (input || input === 0) {
        return prettyBytes(input);
      } else {
        return "-";
      }
    },
    byteDiff(idx: number) {
      const cachedMax = this.cachedMaxExpected[idx] ? this.cachedMaxExpected[idx] : 0;
      const currentExpected = this.networkStates[idx] ? this.networkStates[idx] : 0;
      const diff = cachedMax! - currentExpected!;

      if (diff < 0) {
        return 0;
      } else {
        return diff;
      }
    },
    closeNote() {
      (this.$refs.appLibraryFirstEnter as typeof HCDialog).close()
    }
  },
});
</script>

<style scoped>
.top-bar {
  align-items: center;
  height: 64px;
  background: white;
  box-shadow: 0 0px 5px #9b9b9b;
}

.progress-indicator {
  position: fixed;
  bottom: 0;
  right: 0;
  padding: 20px;
  background-color: white;
  box-shadow: 0 0px 5px #9b9b9b;
  border-radius: 20px 0 0 0;
  min-width: 540px;
}

.highlighted {
  border-top: 4px solid transparent;
  border-left: 4px solid transparent;
  animation: bordercolorchange 1s linear infinite;
}

.highglightedText {
  font-weight: bold;
  color: #482edf;
}

.refresh-button {
  height: 30px;
  border-radius: 8px;
  padding: 0 15px;
  --hc-primary-color: #2c3e50;
  opacity: 0.75;
  margin-top: 30px;
}

.refresh-button:hover {
  opacity: 1;
}

@keyframes bordercolorchange {
  0% {
    border-color: white;
  }
  50% {
    border-color: #482edf;
  }
  100% {
    border-color: white;
  }
}
</style>
