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
      <HCSelect
        style="
          width: 200px;
          margin-right: 5px;
          box-shadow: 0 0px 3px -1px #9b9b9b;
          --hc-label-background: #e8e8eb;
        "
        placeholder="filter by"
        :items="holochainVersions"
        label="Holochain Version"
        @item-selected="selectedHolochainVersion = $event"
      ></HCSelect>
      <img
        src="/img/Square284x284Logo.png"
        style="
          height: 30px;
          filter: grayscale(50%);
          margin-right: 20px;
          margin-left: -2px;
        "
      />

      <HCSelect
        style="
          width: 200px;
          margin-right: 5px;
          box-shadow: 0 0px 3px -1px #9b9b9b;
          --hc-label-background: #e8e8eb;
        "
        placeholder="sort by"
        :items="sortOptions"
        label="sort by"
        @item-selected="sortOption = $event"
      ></HCSelect>
      <mwc-icon style="color: #482edf; text-shadow: 0 0px 5px #9b9b9b"
        >sort</mwc-icon
      >
    </div>
    <!-- <InstallAppDialog ref="test-dialog"/> -->

    <!-- <InstalledAppCard style="margin: 5px" />
    <InstalledAppCard appIcon="/img/dummy_app_icon.png" style="margin: 5px" /> -->

    <div
      style="
        border-bottom: 2px solid rgba(0, 0, 0, 0.2);
        width: 98%;
        margin: 10px;
        margin-top: -18px;
        max-width: 1080px;
        padding-bottom: 3px;
      "
    >
      <span
        style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.4)"
        title="Holochain Apps with Graphical User Interface"
        >Web Apps</span
      >
    </div>
    <div
      v-if="installedApps.length === 0"
      style="
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
      "
    >
      <span style="margin: 24px"
        >There are no apps installed yet in this Holochain version.</span
      >
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

    <div
      style="
        border-bottom: 2px solid rgba(0, 0, 0, 0.2);
        width: 98%;
        margin: 10px;
        margin-top: 50px;
        max-width: 1080px;
        padding-bottom: 3px;
      "
    >
      <span
        style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.4)"
        title="Holochain Apps without Graphical User Interface"
        >Headless Apps</span
      >
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
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { uniq } from "lodash-es";

import "@ui5/webcomponents/dist/Card.js";
import "@material/mwc-button";
import "@material/mwc-icon-button";
import "@material/mwc-icon";

import { HolochainAppInfo } from "../types";
import { isAppRunning } from "../utils";
import InstalledAppCard from "./InstalledAppCard.vue";
import HCSelect from "./subcomponents/HCSelect.vue";

export default defineComponent({
  name: "InstalledAppsList",
  components: {
    InstalledAppCard,
    HCSelect,
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
  } {
    return {
      sortOptions: [
        ["name", "name"],
        ["name descending", "name descending"],
        // ["Holochain Version", "Holochain Version"],
      ],
      sortOption: undefined,
      selectedHolochainVersion: "All Versions",
    };
  },
  emits: ["openApp"],
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
    holochainVersions(): [string, string][] {
      let allApps = this.installedApps;
      let hcVersions: [string, string][] = [["All Versions", "All Versions"]];
      uniq(allApps.map((app) => app.holochainVersion)).forEach((hcVer) => {
        hcVersions.push([hcVer, hcVer]);
      });
      return hcVersions;
    },
  },
  methods: {
    isAppRunning,
    isAppHeadless(app: HolochainAppInfo) {
      return app.webAppInfo.web_ui_info.type === "Headless";
    },
  },
});
</script>
