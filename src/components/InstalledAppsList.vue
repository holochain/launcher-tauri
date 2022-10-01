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
        margin-top: 60px;
        margin-bottom: 5px;
      "
    >
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
      ></HCSelect>
      <mwc-icon style="text-shadow: 0 0px 5px #9b9b9b">sort</mwc-icon>
    </div>
    <!-- <InstallAppDialog ref="test-dialog"/> -->

    <!-- <InstalledAppCard style="margin: 5px" />
    <InstalledAppCard appIcon="/img/dummy_app_icon.png" style="margin: 5px" /> -->
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
        style="margin: 5px; display: flex; flex: 1"
        :app="app"
        @openApp="$emit('openApp', $event)"
        @uninstallApp="$emit('uninstall-app', $event)"
        @disableApp="$emit('disable-app', $event)"
        @enableApp="$emit('enable-app', $event)"
      />
      <!--
      <ui5-card style="width: auto">
        <div style="display: flex; flex-direction: column; flex: 1">
          <div style="display: flex; flex-direction: row">
            <span
              style="
                font-size: 1.6em;
                width: 300px;
                text-overflow: ellipsis;
                overflow: hidden;
                margin-top: 8px;
                margin-left: 8px;
              "
              >{{ app.installed_app_info.installed_app_id }}</span
            >
            <span style="flex: 1"></span>

            <div
              style="
                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;
              "
            >
              <mwc-button
                v-if="
                  isAppRunning(app.installed_app_info) && !isAppHeadless(app)
                "
                @click="
                  $emit('openApp', app.installed_app_info.installed_app_id)
                "
                style="margin-right: 8px"
                label="Open"
                icon="launch"
              >
              </mwc-button>

              <InstalledAppStatus :installedAppInfo="app.installed_app_info" />

              <mwc-icon-button
                @click="
                  $emit('app-selected', app.installed_app_info.installed_app_id)
                "
                style="margin-left: 8px"
                icon="settings"
              >
              </mwc-icon-button>
            </div>
          </div>
        </div>
      </ui5-card> -->
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
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
  } {
    return {
      sortOptions: [
        ["name", "name"],
        ["Holochain Version", "Holochain Version"],
      ],
    };
  },
  emits: ["openApp"],
  computed: {
    sortedApps() {
      let sortedAppList = this.installedApps;
      sortedAppList.sort((appA, appB) =>
        appA.webAppInfo.installed_app_info.installed_app_id.localeCompare(
          appB.webAppInfo.installed_app_info.installed_app_id
        )
      );
      return sortedAppList;
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
