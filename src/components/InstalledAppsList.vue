<template>
  <HCLoading ref="downloading" :text="loadingText" />

  <HCSnackbar :labelText="errorText" ref="snackbar"></HCSnackbar>

  <!-- Web Apps -->
  <div v-if="noWebApps" class="column" style="margin-top: 14%">
    <div style="font-size: 30px; margin-bottom: 70px">
      {{ $t("launcher.getStarted") }}
    </div>

    <div class="row">
      <HCButton
        class="button-large"
        @click="$emit('select-view', { type: 'appStore' })"
        @keypress.enter="$emit('select-view', { type: 'appStore' })"
      >
        <div
          class="row"
          style="
            align-items: center;
            justify-content: center;
            font-size: 25px;
            font-weight: normal;
          "
        >
          <img
            src="/img/home_icon.svg"
            alt="App Store"
            style="
              filter: invert(100%) sepia(0%) saturate(1%) hue-rotate(73deg)
                brightness(104%) contrast(101%);
            "
          />
          <span style="margin-left: 10px">{{ $t("appStore.appStore") }}</span>
        </div>
      </HCButton>

      <HCButton
        class="button-large"
        style="margin-left: 20px"
        @click="installFromFs()"
        @keypress.enter="installFromFs()"
      >
        <div class="row center-content">
          <mwc-icon style="font-size: 33px">folder</mwc-icon>
          <span style="margin-left: 10px; font-size: 25px; font-weight: normal">
            {{ $t("launcher.filesystem") }}
          </span>
        </div>
      </HCButton>
    </div>
  </div>

  <div v-else class="app-grid-container" style="margin-top: 60px">
    <div
      v-for="app in sortedApps"
      :key="app.webAppInfo.installed_app_info.installed_app_id"
      style="margin: 5px 12px"
    >
      <InstalledAppCard :app="app" @openApp="$emit('openApp', $event)" />
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
import HCLoading from "./subcomponents/HCLoading.vue";
import HCButton from "./subcomponents/HCButton.vue";
import prettyBytes from "pretty-bytes";
import HCSnackbar from "./subcomponents/HCSnackbar.vue";
import { mapActions } from "vuex";
import { APPSTORE_APP_ID, DEVHUB_APP_ID } from "../constants";

export default defineComponent({
  name: "InstalledAppsList",
  components: {
    InstalledAppCard,
    HCLoading,
    HCSnackbar,
    HCButton,
  },
  props: {
    installedApps: {
      type: Object as PropType<Array<HolochainAppInfo>>,
      required: true,
    },
  },
  data(): {
    loadingText: string;
    errorText: string;
  } {
    return {
      loadingText: "",
      errorText: "Unknown error occured",
    };
  },
  emits: ["openApp", "select-view"],
  async mounted() {
    await this.connectToWebsocket();
  },
  computed: {
    sortedApps() {
      // if extended happ releases are not yet fetched from the DevHub to include potential
      // GUI updates, just return installedApps with guiUpdateAvailable undefined
      let sortedAppList: Array<HolochainAppInfoExtended> =
        this.installedApps.map((app) => {
          return {
            webAppInfo: app.webAppInfo,
            holochainId: app.holochainId,
            holochainVersion: app.holochainVersion,
            guiUpdateAvailable: undefined,
          };
        });

      // Filter out App Store and DevHub
      sortedAppList = sortedAppList.filter(
        (app) =>
          app.webAppInfo.installed_app_info.installed_app_id !==
            APPSTORE_APP_ID &&
          app.webAppInfo.installed_app_info.installed_app_id !==
            DEVHUB_APP_ID &&
          app.webAppInfo.web_uis.default.type !== "Headless"
      );

      // sort alphabetically, then disabled last
      sortedAppList = sortedAppList
        .sort((appA, appB) =>
          appA.webAppInfo.installed_app_info.installed_app_id.localeCompare(
            appB.webAppInfo.installed_app_info.installed_app_id
          )
        )
        .sort((appA, appB) => {
          return isAppRunning(appA.webAppInfo.installed_app_info) ===
            isAppRunning(appB.webAppInfo.installed_app_info)
            ? 0
            : isAppRunning(appA.webAppInfo.installed_app_info)
            ? -1
            : 1;
        });

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
    installFromFs() {
      window.localStorage.setItem("installFromFs", "true");
      this.$emit("select-view", { type: "appStore" });
    },
    ...mapActions(["connectToWebsocket"]),
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
  min-width: 70vw;
  max-width: 80%;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  /* This is better for small screens, once min() is better supported */
  grid-template-columns: repeat(auto-fill, minmax(min(140px, 100%), 1fr));
  gap: 1rem;
}

.button-large {
  height: 65px;
  min-width: 200px;
  border-radius: 12px;
}
</style>
