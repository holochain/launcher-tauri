<template>
  <div v-if="loading" class="column center-content" style="flex: 1">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  <div v-else class="column" style="flex: 1; margin: 8px">
    <div class="row center-content">
      <mwc-icon-button
        icon="arrow_back"
        @click="$emit('go-back')"
      ></mwc-icon-button>

      <span style="flex: 1; font-size: 1.5em">App Library</span>
      <mwc-button
        label="How to publish an app"
        @click="howToPublish()"
        style=""
      >
      </mwc-button>
      <mwc-button
        icon="folder"
        raised
        style="--mdc-theme-primary: #4720e3; margin-left: 8px"
        label="Select app from FileSystem"
        @click="selectFromFileSystem()"
      >
      </mwc-button>
    </div>

    <div
      v-if="installableApps.length === 0"
      class="column center-content"
      style="flex: 1"
    >
      <span>There are no apps available yet in the DevHub.</span>
      <span style="margin-top: 8px"
        ><span
          style="cursor: pointer; text-decoration: underline"
          @click="howToPublish()"
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
        style="width: 300px; margin-right: 16px; margin-bottom: 16px"
      >
        <ui5-card style="width: auto">
          <div class="column" style="margin: 8px">
            <span style="font-size: 18px">{{ app.app.content.title }}</span>
            <span style="margin-top: 8px; height: 80px; overflow: auto">{{
              app.app.content.description
            }}</span>

            <div class="row center-content" style="flex: 1; margin-top: 8px">
              <span>Version: {{ getLatestRelease(app).content.name }}</span>

              <span style="flex: 1"></span>

              <mwc-button
                icon="add"
                label="Install App"
                @click="saveApp(app)"
              ></mwc-button>
            </div>
          </div>
        </ui5-card>
      </div>
    </div>
  </div>

  <InstallApp
    v-if="selectedAppBundlePath"
    :appBundlePath="selectedAppBundlePath"
    :hdkVersionForApp="hdkVersionForApp"
    @app-installed="
      installClosed();
      $emit('go-back');
    "
    @closing-dialog="installClosed()"
  ></InstallApp>
  <mwc-snackbar
    leading
    labelText="App download failed. Please try again later."
    ref="snackbar"
  ></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "@material/mwc-dialog";
import "@material/mwc-circular-progress";
import "@material/mwc-button";
import "@material/mwc-snackbar";
import { AppWebsocket } from "@holochain/client";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

import {
  AppWithReleases,
  getAllPublishedApps,
  filterByHdkVersion,
  getLatestRelease,
  fetchWebHapp,
} from "../devhub/get-happs";
import { HdkVersion } from "@/hdk";
import InstallApp from "../components/InstallApp.vue";

export default defineComponent({
  name: "AppStore",
  components: {
    InstallApp,
  },
  data(): {
    loading: boolean;
    installableApps: Array<AppWithReleases>;
    selectedAppBundlePath: string | undefined;
    hdkVersionForApp: HdkVersion | undefined;
  } {
    return {
      loading: true,
      installableApps: [],
      selectedAppBundlePath: undefined,
      hdkVersionForApp: undefined,
    };
  },

  async mounted() {
    const holochainId = this.$store.getters["holochainIdForDevhub"];

    const port = this.$store.getters["appInterfacePort"](holochainId);

    const appWs = await AppWebsocket.connect(`ws://localhost:${port}`);

    const devhubInfo = await appWs.appInfo({ installed_app_id: "DevHub" });

    let allApps: Array<AppWithReleases>;
    try {
      allApps = await getAllPublishedApps(appWs, devhubInfo);
    } catch (e) {
      console.log(e);
      // Catch other errors than being offline
      allApps = [];
    }

    const { hdk_versions }: { hdk_versions: HdkVersion[] } = await invoke(
      "get_supported_versions",
      {}
    );
    this.installableApps = filterByHdkVersion(hdk_versions, allApps);

    this.loading = false;
  },
  methods: {
    async howToPublish() {
      await invoke("open_url", {
        url: "https://github.com/holochain/launcher#publishing-a-webhapp-to-the-devhub",
      });
    },
    getLatestRelease,
    async saveApp(app: AppWithReleases) {
      this.loading = true;
      const release = getLatestRelease(app);

      const holochainId = this.$store.getters["holochainIdForDevhub"];

      const port = this.$store.getters["appInterfacePort"](holochainId);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      const devhubInfo = await appWs.appInfo({ installed_app_id: "DevHub" });

      try {
        const bytes = await fetchWebHapp(
          appWs,
          devhubInfo,
          app.app.content.title,
          release.address
        );

        this.selectedAppBundlePath = await invoke("save_app", {
          appBundleBytes: bytes,
        });
        this.hdkVersionForApp = release.content.hdk_version;
      } catch (e) {
        console.log(e);
        (this.$refs as any).snackbar.show();
      }

      this.loading = false;
    },
    async selectFromFileSystem() {
      this.selectedAppBundlePath = (await open({
        filters: [
          { name: "Holochain Application", extensions: ["webhapp", "happ"] },
        ],
      })) as string;
    },
    installClosed() {
      this.selectedAppBundlePath = undefined;
      this.hdkVersionForApp = undefined;
    },
  },
});
</script>
