<template>
  <div v-if="loading" class="column center-content" style="flex: 1">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  <div v-else>
    <div v-for="(app, i) of installableApps" :key="i">
      <span>{{ app.app.content.title }}</span>
      <span>{{ app.app.content.description }}</span>
      <span>{{ getLatestRelease(app).content.name }}</span>

      <mwc-button label="Install App" @click="saveApp(app)"></mwc-button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import "@material/mwc-dialog";
import "@material/mwc-circular-progress";
import { AppWebsocket } from "@holochain/client";
import {
  AppWithReleases,
  getAllPublishedApps,
  filterByHdkVersion,
  getLatestRelease,
  fetchWebHapp,
} from "../../devhub/get-happs";
import { HdkVersion } from "@/hdk";
import "@material/mwc-button";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "InstallableApps",
  data(): { loading: boolean; installableApps: Array<AppWithReleases> } {
    return {
      loading: true,
      installableApps: [],
    };
  },

  async mounted() {
    const version = this.$store.getters["holochainVersionForDevhub"];

    const port = this.$store.getters["appInterfacePort"](version);

    const appWs = await AppWebsocket.connect(`ws://localhost:${port}`);

    const devhubInfo = await appWs.appInfo({ installed_app_id: "DevHub" });

    const allApps = await getAllPublishedApps(appWs, devhubInfo);

    const supportedHdks: HdkVersion[] = await invoke(
      "get_supported_hdk_versions",
      {}
    );

    this.installableApps = filterByHdkVersion(supportedHdks, allApps);

    this.loading = false;
  },
  methods: {
    getLatestRelease,
    async saveApp(app: AppWithReleases) {
      const release = getLatestRelease(app);

      const version = this.$store.getters["holochainVersionForDevhub"];

      const port = this.$store.getters["appInterfacePort"](version);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`);
      const devhubInfo = await appWs.appInfo({ installed_app_id: "DevHub" });

      const bytes = await fetchWebHapp(
        appWs,
        devhubInfo,
        app.app.content.title,
        release.address
      );

      const appBundlePath = await invoke("save_app", {
        appBundleBytes: bytes,
      });

      this.$emit("selected-app-bundle", {
        appBundlePath,
        hdkVersionForApp: release.content.hdk_version,
      });
    },
  },
});
</script>
