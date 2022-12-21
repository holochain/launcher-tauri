<template>
  <HCLoading ref="downloading" :text="loadingText" />

  <HCDialog ref="appLibraryFirstEnter" closeOnSideClick>
    <div
      class="column"
      style="padding: 30px; align-items: center; max-width: 600px"
    >
      <div style="font-weight: 600; font-size: 27px; margin-bottom: 25px">
        Note
      </div>
      <div>
        Holochain is <a href="https://developer.holochain.org/glossary/#peer-to-peer" target="_blank" title="https://developer.holochain.org/glossary/#peer-to-peer">peer-to-peer</a>
         and the <b>App Library is not optimized for download speed yet.</b><br><br>
        Before you can download your first app, <b>the App Library needs to get synchronized with other peers in the background.</b>
        This can take up to 10-20 Minutes depending on the number and size of apps available and the bandwidth of your internet connection.
        <br><br>
        You can see the progress of ongoing App Library Synchronizations in the bottom right corner.
      </div>

      <HCButton style="margin-top: 20px; width: 100px;" @click="$refs.appLibraryFirstEnter.close()">Ok</HCButton>
    </div>

  </HCDialog>

  <div class="column" style="flex: 1">
    <div class="row center-content top-bar">
      <mwc-icon-button
        icon="arrow_back"
        @click="$emit('go-back')"
        :title="$t('buttons.back')"
      ></mwc-icon-button>

      <span
        style="
          flex: 1;
          font-size: 1.5em;
          margin-left: 4px;
          position: sticky;
          top: 0;
          z-index: 1;
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
        icon="folder"
        style="
          margin-left: 8px;
          margin-right: 1px;
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

    <div v-if="loading" class="column center-content" style="flex: 1">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>

    <div
      v-else-if="installableApps.length === 0"
      class="column center-content"
      style="flex: 1"
    >
      <span>{{ $t("appStore.noAppsInStore") }}</span>
      <span style="margin-top: 8px"
        ><span
          style="cursor: pointer; text-decoration: underline"
          title="https://github.com/holochain/launcher#publishing-a-webhapp-to-the-devhub"
          @click="howToPublish()"
          @keydown.enter="howToPublish()"
          tabindex="0"
          >{{ $t("appStore.readThis") }}</span
        >
        {{ $t("appStore.readThisToPublish") }}</span
      >
    </div>

    <div v-else class="row" style="flex-wrap: wrap; margin: 16px">
      <div
        v-for="(app, i) of installableApps"
        :key="i"
        class="column"
        style="margin-right: 16px; margin-bottom: 16px"
      >
        <AppPreviewCard :app="app" @installApp="saveApp(app)" />
      </div>
    </div>
  </div>

  <div class="progress-indicator" :class="{ highlighted: downloadFailed }">
    <div
      style="margin-bottom: 10px; font-weight: 600; margin-left: 10px"
      title="Full Synchronization with Peers Required to Reliably Download all Apps."
    >
      App Library Synchronization:
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
              v-if="networkStates[idx]"
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
              no ongoing peer synchronization</span
            >
          </div>
          <div
            :style="`width: 30%; text-align: center; ${
              idleStates[idx] ? 'opacity: 0.7;' : ''
            }`"
            title="max expected bytes | remaining expected bytes"
          >
            <span :class="{ highglightedText: maxExceeded[idx] }">{{
              prettyBytesLocal(cachedMaxExpected[idx])
            }}</span>
            | {{ prettyBytesLocal(networkStates[idx]) }}
          </div>
        </div>
      </div>
    </div>
  </div>

  <InstallAppDialog
    v-if="selectedAppBundlePath"
    :appBundlePath="selectedAppBundlePath"
    :hdkVersionForApp="hdkVersionForApp"
    @app-installed="
      installClosed();
      $emit('go-back');
    "
    @closing-dialog="installClosed()"
    ref="install-app-dialog"
  ></InstallAppDialog>
  <HCSnackbar
    labelText="App Library Synchronization not Complete. Please try again later."
    ref="snackbar"
  ></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "@material/mwc-circular-progress";
import "@material/mwc-icon";
import "@material/mwc-icon-button";
import { AppWebsocket, NetworkInfo, InstalledCell, CellInfo, CellId } from "@holochain/client";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import HCProgressBar from "../components/subcomponents/HCProgressBar.vue";

import {
  AppWithReleases,
  getAllPublishedApps,
  filterByHdkVersion,
  getLatestRelease,
  fetchWebHapp,
} from "../devhub/get-happs";
import { HdkVersion } from "../hdk";
import InstallAppDialog from "../components/InstallAppDialog.vue";
import HCButton from "../components/subcomponents/HCButton.vue";
import AppPreviewCard from "../components/AppPreviewCard.vue";
import HCLoading from "../components/subcomponents/HCLoading.vue";
import HCDialog from "../components/subcomponents/HCDialog.vue";

import { HolochainId } from "../types";
import prettyBytes from "pretty-bytes";
import { getCellId } from "../utils";

export default defineComponent({
  name: "AppStore",
  components: {
    InstallAppDialog,
    HCButton,
    AppPreviewCard,
    HCLoading,
    HCSnackbar,
    HCProgressBar,
    HCDialog
  },
  data(): {
    loadingText: string;
    loading: boolean;
    installableApps: Array<AppWithReleases>;
    selectedAppBundlePath: string | undefined;
    hdkVersionForApp: HdkVersion | undefined;
    howToPublishUrl: string;
    holochainId: HolochainId | undefined;
    pollInterval: number | null;
    provisionedCells: [string, CellInfo | undefined][] | undefined;
    networkStates: (number | undefined)[];
    cachedMaxExpected: (number | undefined)[];
    latestGossipUpdates: number[]; // timestamps of the latest non-zero gossipInfo update
    idleStates: boolean[];
    maxExceeded: boolean[];
    showProgressIndicator: boolean;
    downloadFailed: boolean;
  } {
    return {
      loadingText: "",
      loading: true,
      installableApps: [],
      selectedAppBundlePath: undefined,
      hdkVersionForApp: undefined,
      howToPublishUrl:
        "https://github.com/holochain/launcher#publishing-a-webhapp-to-the-devhub",
      holochainId: undefined,
      pollInterval: null,
      provisionedCells: undefined,
      networkStates: [undefined, undefined, undefined],
      cachedMaxExpected: [undefined, undefined, undefined],
      latestGossipUpdates: [0, 0, 0],
      idleStates: [true, true, true],
      maxExceeded: [false, false, false],
      showProgressIndicator: false,
      downloadFailed: false,
    };
  },
  // created() {
  //   // check localStorage to know whether this is the first time entering the App Library
  //   if (window.localStorage.getItem("appLibraryWarningShown") || true) {
  //     (this.$refs.appLibraryFirstEnter as typeof HCDialog).open();
  //   }
  //   //window.localStorage.setItem("appLibraryWarningShown", "true");

  // },
  beforeUnmount() {
    window.clearInterval(this.pollInterval!);
  },
  async mounted() {

    if (!window.localStorage.getItem("appLibraryWarningShown")) {
      (this.$refs.appLibraryFirstEnter as typeof HCDialog).open();
      window.localStorage.setItem("appLibraryWarningShown", "true");
    }


    const holochainId = this.$store.getters["holochainIdForDevhub"];
    this.holochainId = holochainId;

    const _hdiOfDevhub = this.$store.getters["hdiOfDevhub"]; // currently not used

    const port = this.$store.getters["appInterfacePort"](holochainId);

    const appWs = await AppWebsocket.connect(`ws://localhost:${port}`);

    const devhubInfo = await appWs.appInfo({
      installed_app_id: `DevHub-${holochainId.content}`,
    });


    const allCells = devhubInfo.cell_info;
    const provisionedCells: [string, CellInfo | undefined][] = Object.entries(allCells).map(([roleName, cellInfos]) => {
      return [roleName, cellInfos.find((cellInfo) => "Provisioned" in cellInfo)]
    });

    this.provisionedCells = provisionedCells.sort(([roleName_a, _cellInfo_a], [roleName_b, _cellInfo_b]) => {
      return roleName_a.localeCompare(roleName_b);
    });


    let allApps: Array<AppWithReleases>;
    try {
      allApps = await getAllPublishedApps(appWs, devhubInfo);
    } catch (e) {
      console.error(e);
      // Catch other errors than being offline
      allApps = [];
    }

    const { hdk_versions }: { hdk_versions: HdkVersion[] } = await invoke(
      "get_supported_versions",
      {}
    );
    this.installableApps = filterByHdkVersion(hdk_versions, allApps);

    // set up polling loop to periodically get gossip progress, global scope (window) seems to
    // be required to clear it again on beforeUnmount()
    await this.getNetworkState();
    this.pollInterval = window.setInterval(
      async () => await this.getNetworkState(),
      2000
    );

    this.loading = false;
  },
  methods: {
    async howToPublish() {
      await invoke("open_url_cmd", {
        url: this.howToPublishUrl,
      });
    },
    getLatestRelease,
    async saveApp(app: AppWithReleases) {
      this.loadingText = "Connecting with DevHub";
      (this.$refs.downloading as typeof HCLoading).open();
      const release = getLatestRelease(app);

      const holochainId = this.$store.getters["holochainIdForDevhub"];

      const port = this.$store.getters["appInterfacePort"](holochainId);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      const devhubInfo = await appWs.appInfo({
        installed_app_id: `DevHub-${holochainId.content}`,
      });

      this.loadingText = "Downloading...";

      try {
        console.log("Release: ", release);
        const bytes = await fetchWebHapp(
          appWs,
          devhubInfo,
          app.app.content.title,
          release.id
        );

        this.selectedAppBundlePath = await invoke("save_app", {
          appBundleBytes: bytes,
        });
        this.hdkVersionForApp = release.content.hdk_version;
        (this.$refs.downloading as typeof HCLoading).close();
        this.loadingText = "";

        this.$nextTick(() => {
          (this.$refs["install-app-dialog"] as typeof InstallAppDialog).open();
        });
      } catch (e) {
        console.log(e);
        (this.$refs as any).snackbar.show();
        (this.$refs.downloading as typeof HCLoading).close();
        this.downloadFailed = true;
        setTimeout(() => (this.downloadFailed = false), 3000);
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
      this.hdkVersionForApp = undefined;
    },
    async getNetworkState() {
      const port = this.$store.getters["appInterfacePort"](this.holochainId);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      const networkInfo: NetworkInfo[] = await appWs.networkInfo({
        dnas: this.provisionedCells!.filter(([roleName, cellInfo]) => !!cellInfo)
          .map(([_roleName, cellInfo]) => getCellId(cellInfo!)![0] as Uint8Array),
      });

      networkInfo.forEach((info, idx) => {
        const expectedIncoming =
          info.fetch_queue_info.op_bytes_to_fetch;

        // In case expected incoming bytes are 0, keep the chached values, otherwise update
        // expectedIncoming
        if (expectedIncoming != 0) {
          this.idleStates[idx] = false;
          this.latestGossipUpdates[idx] = Date.now();
          // if the expected incoming bytes are larger then the max cached value or there
          // is no cached max value, replace it
          const currentMax = this.cachedMaxExpected[idx];
          if (!currentMax || expectedIncoming > currentMax) {
            this.cachedMaxExpected[idx] = expectedIncoming;
            this.maxExceeded[idx] = true;
            setTimeout(() => (this.maxExceeded[idx] = false), 500);
          }
          // make this call after setting max cached value to ensure it is always <= to it
          this.networkStates[idx] = expectedIncoming;
        }

        // If expected incoming is zero, set the progress bar to idle state
        if (expectedIncoming == 0) {
          this.idleStates[idx] = true;
        }

        // if latest non-zero update to gossip progress is older than 30 seconds, set expected incoming
        // and max cached expected incoming to undefined again
        if (new Date().getTime() - this.latestGossipUpdates[idx] > 30000) {
          this.networkStates[idx] = undefined;
          this.cachedMaxExpected[idx] = undefined;
        }
      });

      // if latest updates to gossip progress are older than 30 seconds, set them to undefined again
      this.latestGossipUpdates.forEach((latest, idx) => {
        if (Date.now() - latest > 30000) {
          this.networkStates[idx] = undefined;
          this.cachedMaxExpected[idx] = undefined;
        }
      });
    },
    progressRatio(idx: number) {
      if (this.networkStates[idx] && this.cachedMaxExpected[idx]) {
        return (
          (1 - this.networkStates[idx]! / this.cachedMaxExpected[idx]!) * 100
        );
      } else {
        return undefined;
      }
    },
    prettyBytesLocal(input: number | undefined) {
      if (input) {
        return prettyBytes(input);
      } else {
        return "-";
      }
    },
  },
});
</script>

<style scoped>
.top-bar {
  /* background-color: rgb(225, 226, 255); */
  padding: 8px 8px 8px 6px;
  /* border-bottom: 1px solid black; */
  background: white;
  box-shadow: 0 0px 5px #9b9b9b;
  position: sticky;
  top: 0;
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
