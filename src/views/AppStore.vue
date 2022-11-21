<template>
  <HCLoading ref="downloading" :text="loadingText" />

  <div class="column" style="flex: 1">
    <div class="row center-content top-bar">
      <mwc-icon-button
        icon="arrow_back"
        @click="$emit('go-back')"
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
        >App Library</span
      >
      <HCButton
        outlined
        @click="howToPublish()"
        style="height: 36px; border-radius: 8px; padding: 0 20px"
        :title="howToPublishUrl"
        >How to publish an app
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
          <span style="margin-left: 5px">Select app from Filesystem</span>
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
      <span>There are no apps available yet in the DevHub.</span>
      <span style="margin-top: 8px"
        ><span
          style="cursor: pointer; text-decoration: underline"
          title="https://github.com/holochain/launcher#publishing-a-webhapp-to-the-devhub"
          @click="howToPublish()"
          @keydown.enter="howToPublish()"
          tabindex="0"
          >Read this</span
        >
        to learn how to publish a Holochain application to the DevHub.</span
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

  <div class="progress-indicator">
    <div
      style="margin-bottom: 10px; font-weight: 600; margin-left: 10px"
      title="Full Synchronization with Peers Required to Reliably Download all Apps."
    >
      App Library Synchronization Progress:
    </div>
    <div v-if="gossipInfo">
      <div v-for="(cell, idx) in cells" :key="cell.role_id" class="column">
        <div class="row" style="align-items: center">
          <div
            style="
              width: 25%;
              margin-left: 20px;
              font-size: 0.95em;
              text-align: right;
            "
          >
            {{ cell.role_id }}
          </div>
          <div style="width: 50%; margin: 0 30px">
            <HCProgressBar
              v-if="
                gossipInfo[idx].total_historical_gossip_throughput
                  .expected_op_bytes.incoming > 0
              "
              title="Full Synchronization Required to Download All Apps."
              :progress="gossipProgressIncoming(gossipInfo[idx])"
              style="--height: 10px"
            />
            <span
              v-else
              style="
                opacity: 0.7;
                font-size: 0.8em;
                display: flex;
                justify-content: center;
              "
            >
              no other peers discovered yet</span
            >
          </div>
          <div
            style="width: 25%; text-align: left"
            title="actual bytes / expected bytes"
          >
            {{ gossipProgressIncomingString(gossipInfo[idx]) }}
          </div>
        </div>
      </div>
    </div>

    <div v-else style="text-align: center; opacity: 0.7">
      No Information available.
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
    labelText="Peer Synchronization not Ready. Please try again later."
    ref="snackbar"
  ></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "@material/mwc-circular-progress";
import "@material/mwc-icon";
import "@material/mwc-icon-button";
import { AppWebsocket, DnaGossipInfo, InstalledCell } from "@holochain/client";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import HCProgressBar from "@/components/subcomponents/HCProgressBar.vue";

import {
  AppWithReleases,
  getAllPublishedApps,
  filterByHdkVersion,
  getLatestRelease,
  fetchWebHapp,
} from "../devhub/get-happs";
import { HdkVersion } from "@/hdk";
import InstallAppDialog from "../components/InstallAppDialog.vue";
import HCButton from "../components/subcomponents/HCButton.vue";
import AppPreviewCard from "../components/AppPreviewCard.vue";
import HCLoading from "../components/subcomponents/HCLoading.vue";
import { HolochainId } from "@/types";
import {
  gossipProgressIncoming,
  gossipProgressIncomingString,
  gossipProgressOutgoing,
  gossipProgressOutgoingString,
} from "@/utils";

export default defineComponent({
  name: "AppStore",
  components: {
    InstallAppDialog,
    HCButton,
    AppPreviewCard,
    HCLoading,
    HCSnackbar,
    HCProgressBar,
  },
  data(): {
    loadingText: string;
    loading: boolean;
    installableApps: Array<AppWithReleases>;
    selectedAppBundlePath: string | undefined;
    hdkVersionForApp: HdkVersion | undefined;
    howToPublishUrl: string;
    gossipInfo: DnaGossipInfo[] | undefined;
    holochainId: HolochainId | undefined;
    pollInterval: number | null;
    cells: InstalledCell[] | undefined;
  } {
    return {
      loadingText: "",
      loading: true,
      installableApps: [],
      selectedAppBundlePath: undefined,
      hdkVersionForApp: undefined,
      howToPublishUrl:
        "https://github.com/holochain/launcher#publishing-a-webhapp-to-the-devhub",
      gossipInfo: undefined,
      holochainId: undefined,
      pollInterval: null,
      cells: undefined,
    };
  },
  async created() {
    console.log("Hello I'm created.");
  },
  beforeUnmount() {
    clearInterval(this.pollInterval!);
  },
  async mounted() {
    const holochainId = this.$store.getters["holochainIdForDevhub"];
    this.holochainId = holochainId;

    const _hdiOfDevhub = this.$store.getters["hdiOfDevhub"]; // currently not used

    const port = this.$store.getters["appInterfacePort"](holochainId);

    const appWs = await AppWebsocket.connect(`ws://localhost:${port}`);

    const devhubInfo = await appWs.appInfo({
      installed_app_id: `DevHub-${holochainId.content}`,
    });

    this.cells = devhubInfo.cell_data;

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

    await this.getGossipInfo();
    // set up polling loop to periodically get gossip progress, global scope (window) seems to
    // be required to clear it again on beforeUnmount()
    await this.getGossipInfo();
    this.pollInterval = window.setInterval(
      async () => await this.getGossipInfo(),
      2000
    );

    this.loading = false;
  },
  methods: {
    gossipProgressIncoming,
    gossipProgressOutgoing,
    gossipProgressIncomingString,
    gossipProgressOutgoingString,
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
    async getGossipInfo() {
      console.log("fetching gossip info...");
      const port = this.$store.getters["appInterfacePort"](this.holochainId);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      const gossipInfo: DnaGossipInfo[] = await appWs.gossipInfo({
        dnas: this.cells!.map((cell) => cell.cell_id[0] as Uint8Array),
      });
      console.log("Received gossip info: ", gossipInfo);
      this.gossipInfo = gossipInfo;
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
  border-radius: 20px 0 0 0;
  min-width: 520px;
}
</style>
