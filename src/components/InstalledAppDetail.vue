<template>
  <div style="display: flex; flex-direction: column">
    <span style="margin-right: 8px; opacity: 0.9">Your Public Key:</span>
    <span style="margin-right: 16px; opacity: 0.7; font-family: monospace">{{
      serializeHash(installedAppInfo.cell_data[0].cell_id[1])
    }}</span>

    <table style="text-align: left; margin-top: 8px">
      <tr>
        <th>Cell Role</th>
        <th>Dna Hash</th>
      </tr>

      <tr
        style=""
        v-for="cellData in installedAppInfo.cell_data"
        :key="[...cellData.cell_id[0], ...cellData.cell_id[1]]"
      >
        <td>
          <span
            >{{ cellData.role_id.slice(0, 20)
            }}{{ cellData.role_id.length > 20 ? "..." : "" }}</span
          >
        </td>
        <td>
          <span style="opacity: 0.7; font-family: monospace">{{
            serializeHash(cellData.cell_id[0])
          }}</span>
        </td>
      </tr>
    </table>

    <span
      >Status:

      <InstalledAppStatus :installedAppInfo="installedAppInfo" />
    </span>
    <span v-if="getReason(installedAppInfo)" style="margin-top: 8px">
      {{ getReason(installedAppInfo) }}
    </span>
    <div style="display: flex; flex-direction: row">
      <mwc-button
        @click="uninstallApp()"
        style="margin-left: 8px"
        label="Uninstall"
        icon="delete"
      >
      </mwc-button>

      <mwc-button
        v-if="!isAppDisabled(installedAppInfo)"
        @click="disableApp()"
        style="margin-left: 8px"
        label="Disable"
        icon="archive"
      >
      </mwc-button>
      <mwc-button
        v-if="isAppDisabled(installedAppInfo)"
        @click="enableApp()"
        style="margin-left: 8px"
        label="Enable"
        icon="unarchive"
      >
      </mwc-button>
      <mwc-button
        v-if="isAppPaused(installedAppInfo)"
        @click="startApp()"
        style="margin-left: 8px"
        label="Start"
        icon="play_arrow"
      >
      </mwc-button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { deserializeHash, serializeHash } from "@holochain-open-dev/utils";
import "@material/mwc-button";

import InstalledAppStatus from "./InstalledAppStatus.vue";
import { InstalledWebAppInfo } from "../types";
import { getReason, isAppDisabled, isAppPaused } from "../utils";

export default defineComponent({
  name: "InstalledAppDetail",
  components: { InstalledAppStatus },
  props: {
    installedWebAppInfo: {
      type: Object as PropType<InstalledWebAppInfo>,
      required: true,
    },
  },
  emits: ["disableApp", "enableApp", "startApp", "uninstallApp"],
  computed: {
    installedAppInfo() {
      return this.installedWebAppInfo.installed_app_info;
    },
    appId() {
      return this.installedAppInfo.installed_app_id;
    },
  },
  methods: {
    deserializeHash,
    serializeHash,
    getReason,
    isAppDisabled,
    isAppPaused,
    async enableApp() {
      this.$emit("enableApp", this.appId);
    },
    async disableApp() {
      this.$emit("disableApp", this.appId);
    },
    async startApp() {
      this.$emit("startApp", this.appId);
    },
    async uninstallApp() {
      this.$emit("uninstallApp", this.appId);
    },
  },
});
</script>
<!-- We don't have scoped styles with classes because it becomes harder to export a reusable library -->
