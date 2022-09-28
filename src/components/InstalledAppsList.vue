<template>
  <div style="display: flex; flex: 1; flex-direction: column">
    <HCButton @click="handleKlick" style="width: 128px">Continue</HCButton>
    <HCDialog ref="test-dialog">
      <div style="font-weight: 700">Title</div>
      <HCTextField placeholder="Field 1" />
      <HCTextField placeholder="Field 1" />
    </HCDialog>
    <!-- <InstalledAppCard style="margin: 5px" />
    <InstalledAppCard appIcon="/img/dummy_app_icon.png" style="margin: 5px" /> -->
    <div
      v-if="installedWebApps.length === 0"
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
      v-for="app in installedWebApps"
      :key="app.installed_app_info.installed_app_id"
      style="display: flex; flex-direction: column; margin-bottom: 16px"
    >
      <InstalledAppCard
        style="margin: 5px"
        :app="app"
        @openApp="$emit('openApp', $event)"
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

import { InstalledWebAppInfo } from "../types";
import { isAppRunning } from "../utils";
import InstalledAppCard from "./InstalledAppCard.vue";
import HCButton from "./subcomponents/HCButton.vue";
import HCDialog from "./subcomponents/HCDialog.vue";
import HCTextField from "./subcomponents/HCTextField.vue";

export default defineComponent({
  name: "InstalledAppsList",
  components: { InstalledAppCard, HCButton, HCDialog, HCTextField },
  props: {
    installedWebApps: {
      type: Object as PropType<Array<InstalledWebAppInfo>>,
      required: true,
    },
  },
  emits: ["openApp"],
  methods: {
    isAppRunning,
    isAppHeadless(app: InstalledWebAppInfo) {
      return app.web_ui_info.type === "Headless";
    },
    // to be removed:
    handleKlick() {
      (this.$refs["test-dialog"] as typeof HCDialog).open();
    },
  },
});
</script>
