<template>

  <HCLoading ref="downloading" :text="loadingText" />

  <HCSnackbar
    :labelText="errorText"
    ref="snackbar"
  ></HCSnackbar>

  <div
    style="
      display: flex;
      flex: 1;
      flex-direction: column;
      margin-bottom: 80px;
      padding: 0 30px;
      width: 70%;
      align-items: center;
      min-width: 900px;
    "
  >

    <!-- Web Apps -->
    <div
      class="row section-title"
      style="margin-top: -18px"
    >
      <span
        @click="showWebApps = !showWebApps"
        class="show-hide"
        style="opacity: 0.7; cursor: pointer; margin-left: 10px"
      >
        &nbsp;<!-- {{ showWebApps ? "[-]" : "[show]" }} -->
      </span>
    </div>
    <div v-if="showWebApps" style="margin-bottom: 50px; width: 100%">
      <div
        v-if="noWebApps"
        style="margin-top: 30px; color: rgba(0, 0, 0, 0.6); text-align: center"
      >
        {{ $t("main.noApps") }}
        {{
          selectedHolochainVersion === "All Versions"
            ? "."
            : " in this Holochain Version."
        }}
      </div>

      <div
        v-else
        class="app-grid-container"
      >
        <div
          v-for="app in sortedApps"
          :key="app.webAppInfo.installed_app_info.installed_app_id"
        >
          <InstalledAppCard
            v-if="app.webAppInfo.web_uis.default.type !== 'Headless'"
            :app="app"
            @openApp="$emit('openApp', $event)"
            @uninstallApp="$emit('uninstall-app', $event)"
            @disableApp="$emit('disable-app', $event)"
            @enableApp="$emit('enable-app', $event)"
            @startApp="$emit('startApp', $event)"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import "@material/mwc-button";
import "@material/mwc-icon-button";
import "@material/mwc-icon";

import { HolochainAppInfo, HolochainAppInfoExtended } from "../types";
import { isAppRunning } from "../utils";
import InstalledAppCard from "./InstalledAppCard.vue";
import HCSelectCard from "./subcomponents/HCSelectCard.vue";
import StackedChart from "./subcomponents/StackedChart.vue";
import HCGenericDialog from "./subcomponents/HCGenericDialog.vue";
import HCLoading from "./subcomponents/HCLoading.vue";
import prettyBytes from "pretty-bytes";
import HCSnackbar from "./subcomponents/HCSnackbar.vue";
import { AppInfo, AppWebsocket } from "@holochain/client";
import { i18n } from "../locale";
import { APPSTORE_APP_ID, DEVHUB_APP_ID } from "../constants";


export default defineComponent({
  name: "InstalledAppsList",
  components: {
    InstalledAppCard,
    HCSelectCard,
    StackedChart,
    HCGenericDialog,
    HCLoading,
    HCSnackbar,
  },
  props: {
    installedApps: {
      type: Object as PropType<Array<HolochainAppInfo>>,
      required: true,
    },
  },
  data(): {
    appWebsocket: AppWebsocket | undefined;
    appstoreAppInfo: AppInfo | undefined;
    sortOptions: [string, string][];
    sortOption: string | undefined;
    selectedHolochainVersion: string;
    showHeadlessApps: boolean;
    showWebApps: boolean;
    selectedApp: HolochainAppInfoExtended | undefined;
    loadingText: string;
    errorText: string;
  } {
    return {
      appWebsocket: undefined,
      appstoreAppInfo: undefined,
      sortOptions: [
        [i18n.global.t('main.name'), "name"],
        [i18n.global.t('main.nameDescending'), "name descending"],
        // ["Holochain Version", "Holochain Version"],
      ],
      sortOption: undefined,
      selectedHolochainVersion: "All Versions",
      showHeadlessApps: true,
      showWebApps: true,
      selectedApp: undefined,
      loadingText: "",
      errorText: "Unknown error occured",
    };
  },
  emits: ["openApp", "uninstall-app", "enable-app", "disable-app", "startApp"],
  async mounted() {
    const holochainId = this.$store.getters["holochainIdForDevhub"];
    // connect to AppWebsocket
    const port = this.$store.getters["appInterfacePort"](holochainId);
    const appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
    this.appWebsocket = appWebsocket;
    // TODO add correct installed app id here.
    const appstoreAppInfo = await appWebsocket.appInfo({
        installed_app_id: APPSTORE_APP_ID,
    });
    this.appstoreAppInfo = appstoreAppInfo;
  },
  computed: {
    sortedApps() {
      // if extended happ releases are not yet fetched from the DevHub to include potential
      // GUI updates, just return installedApps with guiUpdateAvailable undefined
      let sortedAppList: Array<HolochainAppInfoExtended> = this.installedApps.map((app) => {
        return {
          webAppInfo: app.webAppInfo,
          holochainId: app.holochainId,
          holochainVersion: app.holochainVersion,
          guiUpdateAvailable: undefined,
        }
      });

      // Filter out App Store and DevHub
      sortedAppList = sortedAppList.filter(
        (app) => app.webAppInfo.installed_app_info.installed_app_id !== APPSTORE_APP_ID && app.webAppInfo.installed_app_info.installed_app_id !== DEVHUB_APP_ID
      );

      if (this.selectedHolochainVersion !== "All Versions") {
        sortedAppList = sortedAppList.filter(
          (app) => app.holochainVersion === this.selectedHolochainVersion
        );
      }

      if (this.sortOption === "name") {
        sortedAppList = sortedAppList.sort((appA, appB) =>
          appA.webAppInfo.installed_app_info.installed_app_id.localeCompare(
            appB.webAppInfo.installed_app_info.installed_app_id
          )
        );
      } else if (this.sortOption === "name descending") {
        sortedAppList = sortedAppList.sort((appA, appB) =>
          appB.webAppInfo.installed_app_info.installed_app_id.localeCompare(
            appA.webAppInfo.installed_app_info.installed_app_id
          )
        );
      } else {
        // default is alphabetical by app id
        sortedAppList = sortedAppList.sort((appA, appB) =>
          appA.webAppInfo.installed_app_info.installed_app_id.localeCompare(
            appB.webAppInfo.installed_app_info.installed_app_id
          )
        );
      }

      return sortedAppList;
    },
    noWebApps(): boolean {
      return this.sortedApps.every(
        (app) => app.webAppInfo.web_uis.default.type === "Headless"
      );
    },
  },
  methods: {
    prettyBytes,
    isAppRunning,
    isAppHeadless(app: HolochainAppInfo) {
      return app.webAppInfo.web_uis.default.type === "Headless";
    },
  },
});
</script>

<style scoped>
.show-hide:hover {
  color: black;
}
.section-title {
  width: 98%;
  margin: 10px;
  max-width: 1080px;
  padding-bottom: 3px;
  align-items: center;
}

.borderBottomed {
  border-bottom: 2px solid rgba(0, 0, 0, 0.4);
}

.app-grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  /* This is better for small screens, once min() is better supported */
  /* grid-template-columns: repeat(auto-fill, minmax(min(200px, 100%), 1fr)); */
  gap: 1rem;
}
</style>
