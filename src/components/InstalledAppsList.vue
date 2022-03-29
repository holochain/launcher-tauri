<template>
  <div style="display: flex; flex: 1; flex-direction: column">
    <div
      v-if="installedWebApps.length === 0"
      style="
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
      "
    >
      <span style="margin-top: 160px"
        >You don't have any apps installed yet</span
      >
    </div>
    <div
      v-else
      v-for="app in installedWebApps"
      :key="app.installed_app_info.installed_app_id"
      style="display: flex; flex-direction: column; margin-bottom: 16px"
    >
      <ui5-card style="width: auto">
        <div
          style="display: flex; flex-direction: column; flex: 1; padding: 8px"
        >
          <div style="display: flex; flex-direction: row">
            <span
              style="
                font-size: 1.6em;
                width: 300px;
                text-overflow: ellipsis;
                overflow: hidden;
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
              <InstalledAppStatus :installedAppInfo="app.installed_app_info" />

              <mwc-icon-button
                @click="$refs[app.installed_app_info.installed_app_id].open()"
                style="margin-left: 8px"
                icon="settings"
              >
              </mwc-icon-button>
              <mwc-dialog
                :ref="app.installed_app_info.installed_app_id"
                :heading="app.installed_app_info.installed_app_id"
              >
                <InstalledAppDetail :installedWebAppInfo="app" />

                <mwc-button
                  label="Ok"
                  slot="primaryAction"
                  dialogAction="close"
                ></mwc-button>
              </mwc-dialog>
            </div>
          </div>

          <div
            style="display: flex; flex-direction: row; align-items: flex-end"
          >
            <mwc-button
              v-if="isAppRunning(app, installed_app_info)"
              @click="$emit('openApp', app.installed_app_info.installed_app_id)"
              style="margin-left: 8px"
              label="Open"
              icon="launch"
            >
            </mwc-button>
          </div>
        </div>
      </ui5-card>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { InstalledWebAppInfo } from "../types";
import { isAppRunning } from "../utils";
import InstalledAppStatus from "./InstalledAppStatus.vue";
import InstalledAppDetail from "./InstalledAppDetail.vue";

export default defineComponent({
  name: "InstalledAppsList",
  components: { InstalledAppStatus, InstalledAppDetail },
  props: {
    installedWebApps: {
      type: Object as PropType<Array<InstalledWebAppInfo>>,
      required: true,
    },
  },
  emits: ["openApp"],
  methods: {
    isAppRunning,
  },
});
</script>
