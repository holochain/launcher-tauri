<template>
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
    <div
      class="row"
      style="
        width: 100%;
        justify-content: flex-end;
        align-items: center;
        max-width: 1100px;
        margin-top: 20px;
        margin-bottom: -5px;
      "
    >
      <HCSelectCard
        style="
          width: 200px;
          margin-right: 5px;
          box-shadow: 0 0px 3px -1px #9b9b9b;
          --hc-label-background: #e8e8eb;
        "
        placeholder="Holochain Versions"
        :items="holochainVersionOptions"
        @item-selected="selectedHolochainVersion = $event"
      ></HCSelectCard>
      <img
        src="/img/Square284x284Logo.png"
        style="
          height: 30px;
          filter: grayscale(50%);
          margin-right: 20px;
          margin-left: -2px;
        "
      />

      <HCSelectCard
        style="
          width: 200px;
          margin-right: 5px;
          box-shadow: 0 0px 3px -1px #9b9b9b;
          --hc-label-background: #e8e8eb;
        "
        placeholder="sort by"
        :items="sortOptions"
        @item-selected="sortOption = $event"
      ></HCSelectCard>
      <mwc-icon style="color: #482edf; text-shadow: 0 0px 5px #9b9b9b"
        >sort</mwc-icon
      >
    </div>

    <!-- Web Apps -->
    <div
      class="row section-title"
      :class="{ borderBottomed: showWebApps }"
      style="margin-top: -18px"
    >
      <span
        style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.6)"
        title="Holochain Apps with Graphical User Interface"
        >Web Apps</span
      >
      <span
        @click="showWebApps = !showWebApps"
        class="show-hide"
        style="opacity: 0.7; cursor: pointer; margin-left: 10px"
      >
        {{ showWebApps ? "[-]" : "[show]" }}
      </span>
    </div>
    <div v-if="showWebApps" style="margin-bottom: 50px; width: 100%">
      <div
        v-if="noWebApps"
        style="margin-top: 30px; color: rgba(0, 0, 0, 0.6); text-align: center"
      >
        There are no Web Apps installed{{
          selectedHolochainVersion === "All Versions"
            ? "."
            : " in this Holochain Version."
        }}
      </div>

      <div
        v-else
        v-for="app in sortedApps"
        :key="app.webAppInfo.installed_app_info.installed_app_id"
        style="
          display: flex;
          flex-direction: column;
          width: 100%;
          align-items: center;
        "
      >
        <InstalledAppCard
          v-if="app.webAppInfo.web_ui_info.type !== 'Headless'"
          style="margin: 5px; display: flex; flex: 1"
          :app="app"
          @openApp="$emit('openApp', $event)"
          @uninstallApp="$emit('uninstall-app', $event)"
          @disableApp="$emit('disable-app', $event)"
          @enableApp="$emit('enable-app', $event)"
        />
      </div>
    </div>

    <!-- Headless Apps -->
    <div
      class="row section-title"
      :class="{ borderBottomed: showHeadlessApps }"
    >
      <span
        style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.6)"
        title="Holochain Apps without Graphical User Interface"
        >Headless Apps</span
      >
      <span
        @click="showHeadlessApps = !showHeadlessApps"
        class="show-hide"
        style="opacity: 0.7; cursor: pointer; margin-left: 10px"
      >
        {{ showHeadlessApps ? "[-]" : "[show]" }}
      </span>
    </div>
    <div v-if="showHeadlessApps" style="margin-bottom: 50px; width: 100%">
      <div
        v-if="noHeadlessApps"
        style="margin-top: 30px; color: rgba(0, 0, 0, 0.6); text-align: center"
      >
        There are no headless apps installed{{
          selectedHolochainVersion === "All Versions"
            ? "."
            : " in this Holochain Version."
        }}
      </div>
      <div
        v-for="app in sortedApps"
        :key="app.webAppInfo.installed_app_info.installed_app_id"
        style="
          display: flex;
          flex-direction: column;
          width: 100%;
          align-items: center;
        "
      >
        <InstalledAppCard
          v-if="app.webAppInfo.web_ui_info.type === 'Headless'"
          style="margin: 5px; display: flex; flex: 1"
          :app="app"
          @openApp="$emit('openApp', $event)"
          @uninstallApp="$emit('uninstall-app', $event)"
          @disableApp="$emit('disable-app', $event)"
          @enableApp="$emit('enable-app', $event)"
        />
      </div>
    </div>

    <!-- Holochain verison info -->
    <div
      class="row section-title"
      :class="{ borderBottomed: showHolochainVersions }"
    >
      <span
        style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.6)"
        title="Installed Holochain Versions"
        >Holochain Versions</span
      >
      <span
        @click="showHolochainVersions = !showHolochainVersions"
        class="show-hide"
        style="opacity: 0.7; cursor: pointer; margin-left: 10px"
      >
        {{ showHolochainVersions ? "[-]" : "[show]" }}
      </span>
      <span style="flex: 1"></span>
      <span
        @click="refreshStorageInfo"
        style="margin-right: 5px; margin-bottom: -8px; cursor: pointer"
      >
        <img
          src="/img/refresh.png"
          style="height: 12px; margin-right: 3px; opacity: 0.7"
        />
        Refresh
      </span>
    </div>
    <div
      v-if="showHolochainVersions"
      class="column"
      style="margin-bottom: 50px; width: 100%; align-items: center"
    >
      <div
        v-if="noHolochainVersions"
        style="margin-top: 30px; color: rgba(0, 0, 0, 0.6); text-align: center"
      >
        There are no Holochain Versions installed.
      </div>
      <div v-else style="max-width: 1090px; width: 99%">
        <div
          v-for="hcVersion in holochainVersions"
          :key="hcVersion"
          style="
            display: flex;
            flex: 1;
            flex-direction: column;
            width: 100%;
            align-items: center;
          "
        >
          <div class="row hc-version" style="margin: 5px 0">
            <img
              src="/img/Square284x284Logo.png"
              style="height: 42px; margin-left: 11px; margin-right: 11px"
            />
            <div style="font-weight: 600; font-size: 1.1em">
              {{ hcVersion }}
            </div>
            <span style="display: flex; flex: 1"></span>
            <span
              v-if="storageInfos && !refreshing"
              style="font-weight: 600; margin-right: 15px"
              >{{
                totalStorage(hcVersion)
                  ? prettyBytes(totalStorage(hcVersion))
                  : "?"
              }}</span
            >
            <StackedChart
              v-if="storageInfos && !refreshing"
              :fractions="storageFractions(hcVersion)"
              :labels="storageLabels(hcVersion)"
              style="width: 200px; height: 34px; margin-right: 12px"
            ></StackedChart>
            <!-- <span style="width: 120px; text-align: center">{{
              storageInfos[hcVersion]
                ? prettyBytes(storageInfos[hcVersion].conductor)
                : "?"
            }}</span>
            <span style="width: 120px; text-align: center">{{
              storageInfos[hcVersion]
                ? prettyBytes(storageInfos[hcVersion].uis)
                : "?"
            }}</span> -->
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { uniq } from "lodash-es";

import "@ui5/webcomponents/dist/Card.js";
import "@material/mwc-button";
import "@material/mwc-icon-button";
import "@material/mwc-icon";

import { HolochainAppInfo, HolochainId, StorageInfo } from "../types";
import { isAppRunning } from "../utils";
import InstalledAppCard from "./InstalledAppCard.vue";
import HCSelectCard from "./subcomponents/HCSelectCard.vue";
import StackedChart from "./subcomponents/StackedChart.vue";
import { invoke } from "@tauri-apps/api/tauri";
import prettyBytes from "pretty-bytes";

export default defineComponent({
  name: "InstalledAppsList",
  components: {
    InstalledAppCard,
    HCSelectCard,
    StackedChart,
  },
  props: {
    installedApps: {
      type: Object as PropType<Array<HolochainAppInfo>>,
      required: true,
    },
  },
  data(): {
    sortOptions: [string, string][];
    sortOption: string | undefined;
    selectedHolochainVersion: string;
    showHeadlessApps: boolean;
    showWebApps: boolean;
    showHolochainVersions: boolean;
    storageInfos: Record<string, StorageInfo>;
    refreshing: boolean;
    refreshTimeout: number | null;
  } {
    return {
      sortOptions: [
        ["name", "name"],
        ["name descending", "name descending"],
        // ["Holochain Version", "Holochain Version"],
      ],
      sortOption: undefined,
      selectedHolochainVersion: "All Versions",
      showHeadlessApps: true,
      showWebApps: true,
      showHolochainVersions: true,
      storageInfos: {},
      refreshing: false,
      refreshTimeout: null,
    };
  },
  emits: ["openApp", "uninstall-app", "enable-app", "disable-app"],
  async mounted() {
    await Promise.all(
      this.installedApps.map(async (app) => {
        this.storageInfos[app.holochainVersion] = await invoke(
          "get_storage_info",
          { holochainId: app.holochainId }
        );
      })
    );
  },
  computed: {
    sortedApps() {
      let sortedAppList = this.installedApps;

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
    noHeadlessApps(): boolean {
      return !this.sortedApps.some(
        (app) => app.webAppInfo.web_ui_info.type === "Headless"
      );
    },
    noWebApps(): boolean {
      return this.sortedApps.every(
        (app) => app.webAppInfo.web_ui_info.type === "Headless"
      );
    },
    noHolochainVersions(): boolean {
      return this.noWebApps && this.noHeadlessApps;
    },
    holochainVersions(): string[] {
      const allApps = this.installedApps;
      return uniq(allApps.map((app) => app.holochainVersion));
    },
    holochainVersionOptions(): [string, string][] {
      let allApps = this.installedApps;
      let hcVersions: [string, string][] = [["All Versions", "All Versions"]];
      uniq(allApps.map((app) => app.holochainVersion)).forEach((hcVer) => {
        hcVersions.push([hcVer, hcVer]);
      });
      return hcVersions;
    },
  },
  methods: {
    prettyBytes,
    isAppRunning,
    isAppHeadless(app: HolochainAppInfo) {
      return app.webAppInfo.web_ui_info.type === "Headless";
    },
    storageFractions(holochainVersion: string) {
      const storageInfo: StorageInfo = this.storageInfos[holochainVersion];
      if (storageInfo) {
        const totalStorage = this.totalStorage(holochainVersion);
        const fractions = Object.values(storageInfo).map(
          (value: number) => (value / totalStorage!) * 100
        );
        return fractions;
      } else {
        return undefined;
      }
    },
    totalStorage(holochainVersion: string): number | undefined {
      const storageInfo = this.storageInfos[holochainVersion];
      if (storageInfo) {
        return Object.values(storageInfo).reduce(
          (acc, currValue) => acc + currValue
        );
      } else {
        return undefined;
      }
    },
    storageLabels(holochainVersion: string) {
      const storageInfo = this.storageInfos[holochainVersion];
      if (storageInfo) {
        return Object.entries(storageInfo).map(
          ([key, value]) => `${key} (${prettyBytes(value)})`
        );
      } else {
        return undefined;
      }
    },
    async refreshStorageInfo() {
      this.refreshing = true;
      this.refreshTimeout = window.setTimeout(
        () => (this.refreshing = false),
        200
      );
      await Promise.all(
        this.installedApps.map(async (app) => {
          this.storageInfos[app.holochainVersion] = await invoke(
            "get_storage_info",
            { holochainId: app.holochainId }
          );
        })
      );
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

.hc-version {
  align-items: center;
  flex: 1;
  width: 100%;
  max-width: 1090px;
  margin-top: 8px;
  border-radius: 15px;
  padding: 8px 0;
  background: white;
  box-shadow: 0 0px 5px #9b9b9b;
}
</style>
