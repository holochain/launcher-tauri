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

  <div class="container">
    <draggable :list="draggableList" class="grid">
      <transition-group>
        <div
          v-for="item in draggableList"
          :key="item.webAppInfo.installed_app_info.installed_app_id"
          class="grid-item"
        >
          <InstalledAppCard
            @openApp="$emit('openApp', $event)"
            :app="item"
            class="list-group-item"
          />
        </div>
      </transition-group>
    </draggable>
  </div>
</template>

<script lang="ts">
import { computed, defineComponent, ref, watch } from "vue";

import "@material/mwc-button";
import "@material/mwc-icon-button";
import "@material/mwc-icon";

import { HolochainAppInfo, HolochainAppInfoExtended } from "../types";
import { VueDraggableNext } from "vue-draggable-next";
import { isAppRunning } from "../utils";
import InstalledAppCard from "./InstalledAppCard.vue";
import HCLoading from "./subcomponents/HCLoading.vue";
import HCButton from "./subcomponents/HCButton.vue";
import prettyBytes from "pretty-bytes";
import HCSnackbar from "./subcomponents/HCSnackbar.vue";
import { mapActions, useStore } from "vuex";
import { APPSTORE_APP_ID, DEVHUB_APP_ID } from "../constants";

export default defineComponent({
  name: "InstalledAppsList",
  components: {
    InstalledAppCard,
    HCLoading,
    HCSnackbar,
    HCButton,
    draggable: VueDraggableNext,
  },
  async mounted() {
    await this.connectToWebsocket();
  },
  setup() {
    const store = useStore();

    const installedApps = computed(() => store.getters.allApps);

    const createExtendedApp = (
      app: HolochainAppInfo
    ): HolochainAppInfoExtended => ({
      webAppInfo: app.webAppInfo,
      holochainId: app.holochainId,
      holochainVersion: app.holochainVersion,
      guiUpdateAvailable: undefined,
    });

    const isNotAppStoreOrDevHub = (app: HolochainAppInfoExtended) =>
      app.webAppInfo.installed_app_info.installed_app_id !== APPSTORE_APP_ID &&
      app.webAppInfo.installed_app_info.installed_app_id !== DEVHUB_APP_ID &&
      app.webAppInfo.web_uis.default.type !== "Headless";

    const compareAppIds = (
      appA: HolochainAppInfoExtended,
      appB: HolochainAppInfoExtended
    ) =>
      appA.webAppInfo.installed_app_info.installed_app_id.localeCompare(
        appB.webAppInfo.installed_app_info.installed_app_id
      );

    const compareRunningApps = (
      appA: HolochainAppInfoExtended,
      appB: HolochainAppInfoExtended
    ) =>
      isAppRunning(appA.webAppInfo.installed_app_info) ===
      isAppRunning(appB.webAppInfo.installed_app_info)
        ? 0
        : isAppRunning(appA.webAppInfo.installed_app_info)
        ? -1
        : 1;

    const sortedApps = computed(() => {
      const order = window.localStorage.getItem("installedAppsListOrder");
      const orderParsed = order ? JSON.parse(order) : [];

      const sortedAppList: Array<HolochainAppInfoExtended> = installedApps.value
        .map(createExtendedApp)
        .filter(isNotAppStoreOrDevHub)
        .sort(compareAppIds)
        .sort(compareRunningApps);

      if (orderParsed.length > 0) {
        sortedAppList.sort(
          (a, b) =>
            orderParsed.indexOf(
              a.webAppInfo.installed_app_info.installed_app_id
            ) -
            orderParsed.indexOf(
              b.webAppInfo.installed_app_info.installed_app_id
            )
        );
      }

      return sortedAppList;
    });

    const draggableList = ref([...sortedApps.value]);

    watch(
      draggableList,
      (newList) => {
        const order = newList.map(
          (app) => app.webAppInfo.installed_app_info.installed_app_id
        );
        localStorage.setItem("installedAppsListOrder", JSON.stringify(order));
      },
      {
        deep: true,
      }
    );

    return {
      draggableList,
    };
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
  computed: {
    noWebApps(): boolean {
      return this.draggableList.every(
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

.button-large {
  height: 65px;
  min-width: 200px;
  border-radius: 12px;
}

.container {
  padding: 2rem;
  width: calc(100% - 4rem);
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  /* This is better for small screens, once min() is better supported */
  grid-template-columns: repeat(auto-fill, minmax(min(140px, 100%), 1fr));
  gap: 1rem;
  user-select: none;
}

.grid-item {
  cursor: pointer;
}
</style>
