<template>
  <HCLoading ref="downloading" :text="loadingText" />

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
      <AppPreviewCard :app="app" :appWebsocket="appWebsocket" @installApp="requestInstall(app)" />
    </div>
  </div>

  <!-- Indicator of online peer hosts -->
  <div
    class="peer-host-indicator column"
  >
    <div class="row" style="align-items: center;" title="number of peers that are part of the app distribution peer network and currently responsive">
      <span style="background-color: #17d310; border-radius: 50%; width: 10px; height: 10px; margin-right: 10px;"></span>
      <span v-if="peerHostStatus"><span style="font-weight: 600;">{{ peerHostStatus.responded }} online</span> peer host{{ peerHostStatus.responded === 1 ? "" : "s"}}</span>
      <span v-else>pinging peer hosts...</span>
    </div>
    <div class="row" style="align-items: center;" title="number of peers that registered themselves in the app distribution peer network but are currently unresponsive">
      <span style="background-color: #bfbfbf; border-radius: 50%; width: 10px; height: 10px; margin-right: 10px;"></span>
      <span v-if="peerHostStatus"><span style="font-weight: 600;">{{ peerHostStatus.totalHosts - peerHostStatus.responded }} unresponsive</span> peer host{{ (peerHostStatus.totalHosts - peerHostStatus.responded) === 1 ? "" : "s"}}</span>
      <span v-else>pinging peer hosts...</span>
    </div>
  </div>

  <!-- Dialog to select releases -->
  <SelectReleaseDialog
    v-if="selectedReleaseInfos && selectedAppName"
    :release-infos="selectedReleaseInfos"
    :app-name="selectedAppName"
    ref="selectAppReleasesDialog"
    @cancel="() => {
      selectedReleaseInfos = undefined;
      selectedAppName = undefined;
      selectedApp = undefined;
    }"
    @release-selected="saveApp($event)"
  >
  </SelectReleaseDialog>

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

import { getHappReleases, getAvailableHostForZomeFunction, fetchGuiReleaseEntry, getVisibleHostsForZomeFunction } from "../appstore/appstore-interface";
import InstallAppDialog from "../components/InstallAppDialog.vue";
import HCButton from "../components/subcomponents/HCButton.vue";
import AppPreviewCard from "../components/AppPreviewCard.vue";
import HCLoading from "../components/subcomponents/HCLoading.vue";
import HCDialog from "../components/subcomponents/HCDialog.vue";
import SelectReleaseDialog from "../components/SelectReleaseDialog.vue";

import { HolochainId, ReleaseInfo } from "../types";
import prettyBytes from "pretty-bytes";
import { getCellId } from "../utils";
import { i18n } from "../locale";
import { AppEntry, Entity, HappReleaseEntry, HostAvailability } from "../appstore/types";
import { getAllApps } from "../appstore/appstore-interface";
import { APP_STORE_ID, DEVHUB_HAPP_LIBRARY_DNA_HASH } from "../constants";



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
    SelectReleaseDialog,
  },
  data(): {
    loadingText: string;
    loading: boolean;
    installableApps: Array<AppEntry>;
    selectedAppBundlePath: string | undefined;
    howToPublishUrl: string;
    holochainId: HolochainId | undefined;
    holochainSelection: boolean;
    peerHostStatus: HostAvailability | undefined;
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
    selectedReleaseInfos: Array<ReleaseInfo> | undefined;
    selectedAppName: string | undefined;
    selectedApp: AppEntry | undefined;
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
      peerHostStatus: undefined,
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
      selectedReleaseInfos: undefined,
      selectedAppName: undefined,
      selectedApp: undefined,
    };
  },
  beforeUnmount() {
    window.clearInterval(this.pollInterval!);
  },
  async mounted() {


    try {
      await this.fetchApps();
    } catch (e) {
      console.error(`Failed to fetch apps in mounted() hook: ${e}`);
    }

    await this.connectAppWebsocket();

    // set up polling loop to periodically get gossip progress, global scope (window) seems to
    // be required to clear it again on beforeUnmount()
    const appStoreInfo = await this.appWebsocket!.appInfo({
      installed_app_id: APP_STORE_ID,
    });

    try {
      const result = await getVisibleHostsForZomeFunction(this.appWebsocket as AppWebsocket, appStoreInfo, 'happ_library', 'get_webhapp_package');
      this.peerHostStatus = result;
    } catch (e) {
      console.error(`Failed to get peer host statuses: ${JSON.stringify(e)}`);
    }


    this.pollInterval = window.setInterval(
      async () => {
        const result = await getVisibleHostsForZomeFunction(
          this.appWebsocket as AppWebsocket,
          appStoreInfo,
          "happ_library",
          "get_webhapp_package"
        );

        this.peerHostStatus = result;
      },
      60000
    );
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

      this.loading = true;

      if (!this.appWebsocket) {
        await this.connectAppWebsocket();
      }

      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: APP_STORE_ID,
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

      // filter by apps of the relevant DevHub dna hash
      this.installableApps = allApps.filter((appEntry) => JSON.stringify(appEntry.devhub_address.dna) === JSON.stringify(DEVHUB_HAPP_LIBRARY_DNA_HASH));

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
    /**
     *
     */
    async requestInstall(app: AppEntry) {
      // fetch releases and open a dialog offering to install different releases
      this.holochainSelection = false;
      this.loadingText = "fetching available releases from peer host";
      (this.$refs.downloading as typeof HCLoading).open();

      // 1. get happ releases for app from DevHub
      if (!this.appWebsocket) {
        await this.connectAppWebsocket();
      }

      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: APP_STORE_ID,
      });

      let happReleases: Array<Entity<HappReleaseEntry>> | undefined = undefined;

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

      // this.selectedHappReleases = happReleases.map((entity) => entity.content).sort((a, b) => b.last_updated - a.last_updated);
      let selectedReleaseInfos: Array<ReleaseInfo> = [];

      console.log("@requestInstall: fetching gui release entries...");

      try {
        await Promise.all(happReleases.map(
          async (happReleaseEntity) => {
            let releaseInfo: ReleaseInfo = {
              happRelease: happReleaseEntity,
              guiRelease: undefined,
            };

            const guiReleaseHash = happReleaseEntity.content.official_gui;
            if (guiReleaseHash) {
              const guiReleaseEntry = await fetchGuiReleaseEntry(this.appWebsocket as AppWebsocket, appStoreInfo, guiReleaseHash);
              releaseInfo.guiRelease = guiReleaseEntry;
            }

            console.log("@requestInstall: fetched GUIReleaseEntry: ReleaseInfo: ", releaseInfo);

            selectedReleaseInfos.push(releaseInfo);
          })
        );
      } catch (e) {
        this.errorText = "Failed to fetch UI release infos.";
        (this.$refs as any).snackbar.show();
        (this.$refs.downloading as typeof HCLoading).close();
        throw new Error("Failed to fetch UI release infos.");
      }

      this.selectedAppName = app.title;
      this.selectedApp = app;
      this.selectedReleaseInfos = selectedReleaseInfos.sort((a, b) => b.happRelease.content.published_at - a.happRelease.content.published_at);

      console.log("@requestInstall: successfully fetched GUIReleaseEntries: ", this.selectedReleaseInfos);

      this.$nextTick(() => {
        (this.$refs.selectAppReleasesDialog as typeof SelectReleaseDialog).open();
        this.loadingText = "Loading";
        (this.$refs.downloading as typeof HCLoading).close();
        console.log("@requestInstall: closed Dialog.")
      });
    },
    async saveApp(releaseInfo: ReleaseInfo) {
      // // if downloading, always take holochain version of DevHub
      this.holochainSelection = false;
      this.loadingText = "searching available peer host";
      (this.$refs.downloading as typeof HCLoading).open();


      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: APP_STORE_ID,
      });

      const happReleaseHash = releaseInfo.happRelease.id;
      const guiReleaseHash = releaseInfo.happRelease.content.official_gui;

      this.selectedHappReleaseHash = encodeHashToBase64(happReleaseHash);
      this.selectedGuiReleaseHash = guiReleaseHash ? encodeHashToBase64(guiReleaseHash) : undefined;

      const host: AgentPubKey = await getAvailableHostForZomeFunction(
        this.appWebsocket as AppWebsocket,
        appStoreInfo,
        "happ_library",
        "get_webhapp_package",
      );

      this.loadingText = "requesting app from peer host";

      try {
        this.selectedAppBundlePath = await invoke("fetch_and_save_app", {
          holochainId: this.holochainId,
          appstoreAppId: appStoreInfo.installed_app_id,
          appTitle: this.selectedApp!.title,
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
        this.selectedHappReleaseHash = undefined;
        this.selectedGuiReleaseHash = undefined;
        this.selectedApp = undefined;
        this.selectedAppName = undefined;
        this.selectedReleaseInfos = undefined;
        (this.$refs as any).snackbar.show();
        (this.$refs.downloading as typeof HCLoading).close();
        return;
      }
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
      this.selectedApp = undefined;
      this.selectedReleaseInfos = undefined;
      this.selectedAppName = undefined;
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

.peer-host-indicator {
  position: fixed;
  bottom: 20px;
  right: 20px;
  padding: 10px 15px;
  background-color: white;
  box-shadow: 0 0px 5px #9b9b9b;
  border-radius: 10px 10px 6px 6px;
  min-width: 220px;
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

</style>
