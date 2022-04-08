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

      <span style="flex: 1; font-size: 1.5em">App Store</span>
      <mwc-button
        icon="folder"
        raised
        style="--mdc-theme-primary: #4720e3"
        label="Select app from FileSystem"
        @click="selectFromFileSystem()"
      >
      </mwc-button>
    </div>
    <div class="row" style="flex-wrap: wrap; margin: 16px">
      <div
        v-for="(app, i) of installableApps"
        :key="i"
        class="column"
        style="width: 400px; margin-right: 16px; margin-bottom: 16px"
      >
        <ui5-card style="width: auto">
          <div class="column" style="margin: 8px">
            <span style="font-size: 18px">{{ app.app.content.title }}</span>
            <span style="margin-top: 8px">{{
              app.app.content.description
            }}</span>
            <span style="margin-top: 8px"
              >Release: {{ getLatestRelease(app).content.name }}</span
            >

            <div class="row center-content" style="flex: 1; margin-top: 8px">
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
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "@material/mwc-dialog";
import "@material/mwc-circular-progress";
import "@material/mwc-button";
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
    const version = this.$store.getters["holochainVersionForDevhub"];

    const port = this.$store.getters["appInterfacePort"](version);

    const appWs = await AppWebsocket.connect(`ws://localhost:${port}`);

    const devhubInfo = await appWs.appInfo({ installed_app_id: "DevHub" });

    const allApps = await getAllPublishedApps(appWs, devhubInfo);

    const { hdk_versions }: { hdk_versions: HdkVersion[] } = await invoke(
      "get_supported_versions",
      {}
    );

    this.installableApps = filterByHdkVersion(hdk_versions, allApps);

    this.loading = false;
  },
  methods: {
    getLatestRelease,
    async saveApp(app: AppWithReleases) {
      this.loading = true;
      const release = getLatestRelease(app);

      const version = this.$store.getters["holochainVersionForDevhub"];

      const port = this.$store.getters["appInterfacePort"](version);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      const devhubInfo = await appWs.appInfo({ installed_app_id: "DevHub" });

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
