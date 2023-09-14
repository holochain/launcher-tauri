<template>
  <HCLoading ref="downloading" :text="loadingText" />

  <div class="row search-bar">
    <mwc-icon style="font-size: 38px; margin-left: 16px">search</mwc-icon>
    <HCTextField
      ref="search-field"
      style="height: 45px; margin-left: 5px"
      placeholder="Search..."
      @input="highlightSearchString"
    ></HCTextField>
    <span style="display: flex; flex: 1"></span>
    <HCButton
      style="border-radius: 10px; height: 40px; margin-right: 7px"
      @click="selectFromFileSystem()"
      @keypress.enter="selectFromFileSystem()"
    >
      <div class="row" style="margin: -12px">
        <mwc-icon>folder</mwc-icon>
        <span style="margin-left: 10px">{{
          $t("appStore.selectAppFromFileSystem")
        }}</span>
      </div>
    </HCButton>
  </div>

  <div class="row" style="justify-content: flex-end; margin-top: 70px">
    <div
      class="row"
      style="align-items: center; margin-right: 10px"
      :title="$t('appStore.showFilteredAppsTitle')"
    >
      <span style="margin-right: 5px">{{
        $t("appStore.showFilteredApps")
      }}</span>
      <ToggleSwitch
        :sliderOn="showFilteredApps"
        @click="showFilteredApps = !showFilteredApps"
        @keydown.enter="showFilteredApps = !showFilteredApps"
      ></ToggleSwitch>
    </div>
  </div>

  <div
    v-if="loading"
    class="column"
    style="flex: 1; min-height: calc(100vh - 240px)"
  >
    <div class="column center-content" style="display: flex; flex: 1">
      <LoadingDots
        style="--radius: 15px; --dim-color: #e8e8eb; --fill-color: #b5b5b5"
      ></LoadingDots>
    </div>
  </div>

  <div
    v-else-if="installableApps.length === 0"
    class="column center-content"
    style="flex: 1; min-height: calc(100vh - 124px); margin: 16px"
  >
    <div class="column center-content">
      <div class="radar halo" style="width: 200px; height: 200px">
        <div class="__dot"></div>
        <div class="__dot"></div>
        <div class="__dot"></div>
      </div>
      <span
        style="
          max-width: 600px;
          text-align: center;
          margin-top: 25px;
          font-size: 20px;
        "
        >{{ $t("appStore.searchingForPeers") }}...</span
      >
      <span
        style="
          max-width: 600px;
          text-align: center;
          margin-top: 15px;
          opacity: 0.8;
        "
        >{{ $t("appStore.searchingForPeersDetail") }}</span
      >
    </div>
  </div>

  <div v-else-if="filteredApps.length === 0">
    <div
      class="column center-content"
      style="margin-top: 300px; font-size: 20px"
    >
      <span style="max-width: 600px; text-align: center">{{
        $t("appStore.noAppsForSearch")
      }}</span>
    </div>
  </div>

  <div
    ref="apps-list"
    v-else
    class="row"
    style="
      flex-wrap: wrap;
      align-content: flex-start;
      margin: 16px;
      min-height: calc(100vh - 124px);
      margin-bottom: 80px;
      margin-top: 20px;
    "
  >
    <div
      v-for="(appEntity, i) of installableApps"
      :key="i"
      class="column"
      style="margin-right: 16px; margin-bottom: 16px"
    >
      <AppPreviewCard
        v-show="filteredApps.includes(appEntity.content)"
        :app="appEntity.content"
        :appWebsocket="appWebsocket"
        @installApp="requestInstall(appEntity.content, $event.imgSrc)"
      />
    </div>
  </div>

  <!-- AppStore synchronization spinner -->
  <!-- <div v-show="showLoadingSpinner" class="progress-indicator">
    <div style="padding: 0 15px;">
      <div
        style="margin-bottom: 5px; font-weight: 600; font-size: 18px;"
        :title="$t('appStore.fullSynchronizationRequired')"
      >
        {{ $t('appStore.receivingData') }}...
      </div>
      <div style="text-align: right; margin-bottom: 10px;" :title="$t('appStore.amountOfData')">
        <b>{{ prettyBytesLocal(queuedBytes) }}</b> {{ $t('appStore.inQueue') }}
      </div>
    </div>
    <span :class="queuedBytes ? 'loader' : 'inactive-loader'" style="position: absolute; bottom: 0;"></span>
  </div> -->

  <!-- refresh button -->
  <HCButton
    style="
      height: 50px;
      border-radius: 18px;
      padding: 0 20px;
      position: fixed;
      bottom: 20px;
      right: 20px;
      font-size: 18px;
      margin-left: -140px;
    "
    @click="fetchApps(false)"
    @keypress.enter="fetchApps(false)"
  >
    <div class="row center-content">
      <mwc-icon>refresh</mwc-icon>
      <span style="margin-left: 8px">{{ $t("main.refresh") }}</span>
    </div>
  </HCButton>

  <!-- Dialog to select releases -->
  <SelectReleaseDialog
    v-if="selectedApp"
    :app="selectedApp"
    :appWebsocket="appWebsocket"
    :imgSrc="selectedIconSrc"
    ref="selectAppReleasesDialog"
    @cancel="
      () => {
        selectedApp = undefined;
      }
    "
    @release-selected="saveApp($event.releaseData, $event.appEntry)"
  >
  </SelectReleaseDialog>

  <InstallAppDialog
    v-if="selectedAppBundlePath"
    :appBundlePath="selectedAppBundlePath"
    :holochainSelection="holochainSelection"
    :happReleaseInfo="selectedHappReleaseInfo"
    :guiReleaseInfo="selectedGuiReleaseInfo"
    :iconSrc="selectedIconSrc"
    @app-installed="
      holochainSelection = true;
      installClosed();
      showMessage(`Installed App ${$event}`);
      $emit('select-view', { type: 'launcher' });
    "
    @closing-dialog="installClosed()"
    @error="(e) => showMessage(e)"
    ref="install-app-dialog"
  ></InstallAppDialog>
  <HCSnackbar :labelText="errorText" ref="snackbar"></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "@material/mwc-circular-progress";
import "@material/mwc-icon";
import "@material/mwc-icon-button";
import {
  AppWebsocket,
  NetworkInfo,
  CellInfo,
  encodeHashToBase64,
} from "@holochain/client";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import Mark from "mark.js";
import { toSrc, getCellId } from "../utils";
import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import LoadingDots from "../components/subcomponents/LoadingDots.vue";
import {
  tryWithHosts,
  collectBytes,
  getAllApps,
} from "../appstore/appstore-interface";
import InstallAppDialog from "../components/InstallAppDialog.vue";
import HCButton from "../components/subcomponents/HCButton.vue";
import HCTextField from "../components/subcomponents/HCTextField.vue";
import AppPreviewCard from "../components/AppPreviewCard.vue";
import HCLoading from "../components/subcomponents/HCLoading.vue";
import SelectReleaseDialog from "../components/SelectReleaseDialog.vue";
import ToggleSwitch from "../components/subcomponents/ToggleSwitch.vue";
import {
  FilterListEntry,
  HolochainId,
  ReleaseData,
  ReleaseInfo,
} from "../types";
import prettyBytes from "pretty-bytes";
import { AppEntry, Entity } from "../appstore/types";
import { APPSTORE_APP_ID } from "../constants";
import { mapActions, mapGetters } from "vuex";

export default defineComponent({
  name: "AppStore",
  components: {
    InstallAppDialog,
    HCButton,
    HCTextField,
    AppPreviewCard,
    HCLoading,
    HCSnackbar,
    LoadingDots,
    SelectReleaseDialog,
    ToggleSwitch,
  },
  emits: ["show-message", "select-view"],
  data(): {
    filterList: Array<FilterListEntry> | undefined;
    loadingText: string;
    loading: boolean;
    installableApps: Array<Entity<AppEntry>>;
    selectedAppBundlePath: string | undefined;
    holochainId: HolochainId | undefined;
    holochainSelection: boolean;
    provisionedCells: [string, CellInfo | undefined][] | undefined;
    networkStates: (number | undefined)[];
    cachedMaxExpected: (number | undefined)[];
    latestQueuedBytesUpdate: number;
    showProgressIndicator: boolean;
    errorText: string;
    pollInterval: number | null;
    queuedBytes: number | undefined;
    selectedHappReleaseInfo: ReleaseInfo | undefined;
    selectedGuiReleaseInfo: ReleaseInfo | undefined;
    selectedApp: AppEntry | undefined;
    selectedIconSrc: string | undefined;
    showLoadingSpinner: boolean;
    showFilteredApps: boolean;
  } {
    return {
      filterList: undefined,
      loadingText: "",
      loading: true,
      installableApps: [],
      selectedAppBundlePath: undefined,
      holochainId: undefined,
      holochainSelection: true,
      provisionedCells: undefined,
      networkStates: [undefined, undefined, undefined],
      cachedMaxExpected: [undefined, undefined, undefined],
      latestQueuedBytesUpdate: 0,
      showProgressIndicator: false,
      errorText: "Unknown error occured.",
      pollInterval: null,
      queuedBytes: undefined,
      selectedHappReleaseInfo: undefined,
      selectedGuiReleaseInfo: undefined,
      selectedApp: undefined,
      selectedIconSrc: undefined,
      showLoadingSpinner: false,
      showFilteredApps: false,
    };
  },
  beforeUnmount() {
    window.clearInterval(this.pollInterval!);
  },
  async mounted() {
    // If the "Filesystem" button is pressed in the "launcher" view with no apps installed, the
    // "installFromFs" item is set to "true" in localStorage and then the view is switched to
    // "appStore" view (i.e. to this component here).
    // In that case, the select from filesystem logic shall immediately be called after mounting of the component
    // and the localStorage item be removed again.
    if (window.localStorage.getItem("installFromFs")) {
      window.localStorage.removeItem("installFromFs");
      this.selectFromFileSystem();
    }

    try {
      // Temporary solution: Fetch centrally stored filter list to filter out orphaned apps in the appstore.
      const response = await fetch(
        "https://s3.eu-central-2.wasabisys.com/holochain-launcher/filter-lists/appstore_filter_list.json"
      );
      const filterList: Array<FilterListEntry> = await response.json();
      this.filterList = filterList;
    } catch (e) {
      console.warn("Failed to get filter list: ", e);
    }

    try {
      await this.fetchApps(false);
    } catch (e) {
      console.error(`Failed to fetch apps in mounted() hook: ${e}`);
    }

    // await this.getQueuedBytes();
    this.pollInterval = window.setInterval(async () => {
      // await this.getQueuedBytes();
      await this.fetchApps(true);
    }, 3000);
  },
  computed: {
    ...mapGetters(["appWebsocket"]),
    filteredApps(): Array<AppEntry> {
      if (this.installableApps.length === 0) return [];

      const listFilter =
        this.filterList && !this.showFilteredApps
          ? (appEntity: Entity<AppEntry>) => {
              const filterActions = this.filterList!.map(
                (listEntry) => listEntry.actionHash
              );
              return !filterActions.includes(
                encodeHashToBase64(appEntity.action)
              );
            }
          : (_appEntity: Entity<AppEntry>) => true;

      const searchString = (this.$refs["search-field"] as typeof HCTextField)
        .value;

      const searchFilter = (app: AppEntry) => {
        const lowerCaseSearchString = searchString.toLowerCase();
        return (
          app.title.toLowerCase().includes(lowerCaseSearchString) ||
          app.subtitle.toLowerCase().includes(lowerCaseSearchString)
        );
      };

      return this.installableApps
        .filter((appEntity: Entity<AppEntry>) => listFilter(appEntity))
        .map((appEntity: Entity<AppEntry>) => appEntity.content)
        .filter((app: AppEntry) => searchFilter(app));
    },
  },
  methods: {
    toSrc,
    ...mapActions(["connectToWebsocket"]),
    async fetchApps(silent: boolean) {
      this.loading = silent ? false : true;

      const holochainId = this.$store.getters["holochainIdForDevhub"];
      this.holochainId = holochainId;

      const appWebsocket = this.appWebsocket as AppWebsocket | undefined;

      if (!appWebsocket) {
        return;
      }

      const appStoreInfo = await appWebsocket!.appInfo({
        installed_app_id: APPSTORE_APP_ID,
      });

      // console.log("@fetchApps: appStoreInfo: ", appStoreInfo);
      const allCells = appStoreInfo.cell_info;
      // console.log("@fetchApps: allCells: ", allCells);

      const provisionedCells: [string, CellInfo | undefined][] = Object.entries(
        allCells
      ).map(([roleName, cellInfos]) => {
        return [
          roleName,
          cellInfos.find((cellInfo) => "provisioned" in cellInfo),
        ];
      });

      // console.log("@fetchApps: provisionedCells: ", provisionedCells);

      this.provisionedCells = provisionedCells.sort(
        ([roleName_a, _cellInfo_a], [roleName_b, _cellInfo_b]) => {
          return roleName_a.localeCompare(roleName_b);
        }
      );

      let allApps: Array<Entity<AppEntry>>;
      try {
        allApps = await getAllApps(
          this.appWebsocket! as AppWebsocket,
          appStoreInfo
        );
      } catch (e) {
        console.error(`Error getting all apps: ${e}`);
        // Catch other errors than being offline
        allApps = [];
      }

      // console.log("@fetchApps: allApps: ", allApps);

      // filter by apps of the relevant DevHub dna hash
      // this.installableApps = allApps.filter((appEntry) => JSON.stringify(appEntry.devhub_address.dna) === JSON.stringify(DEVHUB_HAPP_LIBRARY_DNA_HASH));

      this.installableApps = allApps;

      this.loading = false;

      // console.log("ALL APPS: ", allApps);
      // console.log("FILTERED APPS: ", this.installableApps);
      // console.log("hdk versions: ", hdk_versions);
    },
    async peerToPeer() {
      await invoke("open_url_cmd", {
        url: "https://developer.holochain.org/glossary/#peer-to-peer",
      });
    },
    /**
     *
     */
    async requestInstall(app: AppEntry, imgSrc: string | undefined) {
      this.selectedIconSrc = imgSrc ? imgSrc : undefined;
      this.selectedApp = app;

      // 1. get happ releases for app from DevHub
      if (!this.appWebsocket) {
        await this.connectToWebsocket();
      }

      this.$nextTick(() => {
        (
          this.$refs.selectAppReleasesDialog as typeof SelectReleaseDialog
        ).open();
      });
    },
    async saveApp(releaseInfo: ReleaseData, appEntry: AppEntry) {
      // if downloading, always take holochain version of DevHub
      this.holochainSelection = false;
      this.loadingText = "searching available peer host";
      (this.$refs.downloading as typeof HCLoading).open();

      const appStoreInfo = await this.appWebsocket!.appInfo({
        installed_app_id: APPSTORE_APP_ID,
      });

      // fetching icon from appstore if not already fetched earlier
      let errorFetchingIcon = false;
      if (!this.selectedIconSrc) {
        try {
          if (!this.appWebsocket) {
            await this.connectToWebsocket();
          }
          this.loadingText = `Loading app icon from App Store...`;
          const collectedBytes = await collectBytes(
            this.appWebsocket,
            appStoreInfo,
            appEntry.icon
          );
          this.selectedIconSrc = toSrc(
            collectedBytes,
            appEntry.metadata.icon_mime_type
          );
        } catch (e) {
          console.error("Error fetching app icon from App Store: ", e);
          errorFetchingIcon = true;
        }
      }

      const happReleaseHash = releaseInfo.happRelease.id;
      const guiReleaseHash = releaseInfo.happRelease.content.official_gui;

      this.selectedHappReleaseInfo = {
        resource_locator: {
          dna_hash: encodeHashToBase64(releaseInfo.devhubDnaHash),
          resource_hash: encodeHashToBase64(happReleaseHash),
        },
        version: releaseInfo.happRelease.content.version,
      };
      this.selectedGuiReleaseInfo = guiReleaseHash
        ? {
            resource_locator: {
              dna_hash: encodeHashToBase64(releaseInfo.devhubDnaHash),
              resource_hash: encodeHashToBase64(guiReleaseHash),
            },
            version: releaseInfo.guiRelease?.content.version,
          }
        : undefined;

      this.loadingText = `${
        errorFetchingIcon ? "Failed to fetch icon from App Store. F" : "f"
      }etching app from peer host${
        errorFetchingIcon ? " without icon " : ""
      }...`;

      try {
        await tryWithHosts<void>(
          async (host) => {
            this.selectedAppBundlePath = await invoke("fetch_and_save_app", {
              holochainId: this.holochainId,
              appstoreAppId: appStoreInfo.installed_app_id,
              appTitle: this.selectedApp!.title,
              host: Array.from(host),
              devhubHappLibraryDnaHash: Array.from(releaseInfo.devhubDnaHash), // DNA hash of the DevHub to which the remote call shall be made
              appstorePubKey: encodeHashToBase64(appStoreInfo.agent_pub_key),
              happReleaseHash: encodeHashToBase64(happReleaseHash),
            });

            (this.$refs.downloading as typeof HCLoading).close();
            this.loadingText = "";

            this.$nextTick(() => {
              (
                this.$refs["install-app-dialog"] as typeof InstallAppDialog
              ).open();
            });

            // console.log(
            //   "@saveApp: selectedAppBundlePath: ",
            //   this.selectedAppBundlePath
            // );
          },
          this.appWebsocket as AppWebsocket,
          appStoreInfo,
          releaseInfo.devhubDnaHash,
          "happ_library",
          "get_webhapp_package"
        );
      } catch (e) {
        console.error("Error fetching webhapp from DevHub host(s): ", e);
        this.selectedHappReleaseInfo = undefined;
        this.selectedGuiReleaseInfo = undefined;
        this.selectedApp = undefined;
        this.selectedIconSrc = undefined;
        this.showMessage("Failed to fetch webhapp from DevHub host(s).");
        (this.$refs.downloading as typeof HCLoading).close();
        return;
      }
    },
    async selectFromFileSystem() {
      this.selectedIconSrc = undefined;
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
      // this.hdkVersionForApp = undefined;
    },
    /**
     * Gets aggregated bytes that are in queue for the DevHub cells
     */
    async getQueuedBytes() {
      if (!this.appWebsocket) {
        await this.connectToWebsocket();
      }
      const networkInfo: NetworkInfo[] = await this.appWebsocket!.networkInfo({
        agent_pub_key: getCellId(this.provisionedCells![0][1]!)![1],
        dnas: this.provisionedCells!.filter(
          ([_roleName, cellInfo]) => !!cellInfo
        ).map(
          ([_roleName, cellInfo]) => getCellId(cellInfo!)![0] as Uint8Array
        ),
      });
      let queuedBytes = 0;
      networkInfo.forEach((info, _idx) => {
        queuedBytes += info.fetch_pool_info.op_bytes_to_fetch;
      });
      this.queuedBytes = queuedBytes;
      const now = Date.now();
      if (!!queuedBytes && queuedBytes > 0) {
        this.latestQueuedBytesUpdate = now;
        // console.log("updated timestamp: ", this.latestQueuedBytesUpdate);
      }
      if (now - this.latestQueuedBytesUpdate < 5000) {
        this.showLoadingSpinner = true;
      } else {
        this.showLoadingSpinner = false;
      }

      return queuedBytes;
    },
    showMessage(message: string) {
      this.$emit("show-message", message);
    },
    progressRatio(idx: number) {
      if (
        (this.networkStates[idx] || this.networkStates[idx] === 0) &&
        this.cachedMaxExpected[idx]
      ) {
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
      const cachedMax = this.cachedMaxExpected[idx]
        ? this.cachedMaxExpected[idx]
        : 0;
      const currentExpected = this.networkStates[idx]
        ? this.networkStates[idx]
        : 0;
      const diff = cachedMax! - currentExpected!;

      if (diff < 0) {
        return 0;
      } else {
        return diff;
      }
    },
    highlightSearchString() {
      const searchString = (this.$refs["search-field"] as typeof HCTextField)
        .value;
      console.log("searchstring: ", searchString);
      const appsListElement = this.$refs["apps-list"] as
        | HTMLElement
        | undefined;
      if (appsListElement) {
        const instance = new Mark(appsListElement);
        instance.unmark();
        instance.mark(searchString, {
          className: "mark",
          caseSensitive: false,
        });
      }
    },
  },
});
</script>

<style scoped>
.search-bar {
  position: fixed;
  width: 100%;
  height: 60px;
  align-items: center;
  background-color: #e8e8eb;
  box-shadow: 0 0px 5px #9b9b9b;
  z-index: 1;
}

.progress-indicator {
  position: fixed;
  bottom: 20px;
  left: 20px;
  padding: 10px 0 0 0;
  background-color: white;
  box-shadow: 0 0px 5px #9b9b9b;
  border-radius: 10px 10px 6px 6px;
  min-width: 350px;
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

.halo {
  background-image: url(/img/Square310x310Logo.png);
  background-size: 105%;
  background-position: center center;
  /* filter: grayscale(1); */
}

.radar {
  position: relative;
  width: 60vmin;
  height: 60vmin;
  border-radius: 50%;
  box-sizing: border-box;
  overflow: hidden;
}

.radar::after {
  content: "";
  position: absolute;
  inset: 0;
  background-image: conic-gradient(
    transparent 94%,
    #05edc600 94%,
    #05edc6
  ); /* scanner  color*/
  border-radius: 50%;
  animation: spin 2.5s linear infinite;
}

.radar .__dot {
  position: absolute;
  width: 8%;
  height: 8%;
  border-radius: 50%;
  transform: translate(-50%, -50%);
  animation: blink 2.5s ease-out infinite;
}

.radar .__dot:first-of-type {
  top: 24%;
  left: 76%;
  animation-delay: 0.3s;
}

.radar .__dot:nth-of-type(2) {
  top: 83%;
  left: 55%;
  animation-delay: 1.15s;
}

.radar .__dot:last-of-type {
  top: 36%;
  left: 36%;
  animation-delay: 2.2s;
}

@keyframes spin {
  to {
    transform: rotate(1turn);
  }
}

@keyframes blink {
  2%,
  20% {
    /* background-color: #2096c9; */
    background-color: #303dab;
    box-shadow: 0 0 0.3vmin #151e68;
  }

  90% {
    background-color: transparent;
  }
}
</style>

<style>
/* non-scoped styles */
mark {
  background: linear-gradient(228.21deg, #bc2fd870 0%, #2f86d872 94.99%);
  color: black;
}
</style>
