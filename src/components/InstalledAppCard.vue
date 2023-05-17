<template>
  <HCGenericDialog
    @confirm="uninstallApp(app)"
    closeOnSideClick
    ref="uninstall-app-dialog"
    :primaryButtonLabel="$t('buttons.uninstall')"
    ><div style="text-align: center">
      {{ $t('dialogs.confirmUninstallApp') }}
    </div>
  </HCGenericDialog>

  <div class="container">
    <div
      style="
        position: relative;
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        height: 120px;
      "

      @click="$emit('openApp', app)"
      v-on:keyup.enter="$emit('openApp', app)"
    >
    <!-- App Logo with Holo Identicon -->
      <div style="position: relative">
        <!-- assumes same agent pub key for all cells (just taking the first one) -->
        <!-- <div v-show="showPubKeyTooltip" class="tooltip">Copied!</div> -->
        <sl-tooltip class="tooltip" hoist placement="top" :content="showPubKeyTooltip ? $t('main.copied') : $t('main.yourPublicKey')">
          <HoloIdenticon
            :class="{ holoIdenticon: !showMore, holoIdenticonMore: showMore }"
            style="position: absolute; top: 78px; left: 78px; cursor: pointer"
            :hash="getPubKey()"
            :size="42"
            tabindex="0"
            @click="copyPubKey()"
            @keypress.enter="copyPubKey()"
          ></HoloIdenticon>
        </sl-tooltip>

        <img
          v-if="app.webAppInfo.icon_src"
          :class="{ appIcon: !showMore, appIconMore: showMore }"
          :src="`${app.webAppInfo.icon_src}`"
        />
        <div
          v-else
          :class="{ appIcon: !showMore, appIconMore: showMore }"
          class="column center-content"
          style="background-color: #372ba5"
        >
          <div style="color: white; font-size: 45px; font-weight: 600">
            {{ app.webAppInfo.installed_app_info.installed_app_id.slice(0, 2) }}
          </div>
        </div>
      </div>
      <!-- ------------- -->

      <!-- GUI update available Icon -->
      <div
        v-if="
          app.guiUpdateAvailable
        "
        style="display: flex"
      >
        <sl-tooltip class="tooltip" hoist placement="top" content="New UI available">
          <!-- <img
            tabindex="0"
            style="margin-right: 29px; width: 24px; cursor: pointer"
            src="/img/Open_App.svg"
            @click="$emit('openApp', app)"
            v-on:keyup.enter="$emit('openApp', app)"
          /> -->
          <div
            @click="$emit('updateGui', app)"
            @keypress.enter="$emit('updateGui', app)"
            tabindex="0"
            class="update-button"
          >
            Update
          </div>
        </sl-tooltip>
      </div>
      <!-- -------------------- -->

      <!-- App status indicator -->
      <sl-tooltip
        style="--show-delay: 500"
        hoist
        placement="top"
        :content="getAppStatus(app)"
      >
        <div
          :class="{
            running: isAppRunning(app.webAppInfo.installed_app_info) || isAppPaused(app.webAppInfo.installed_app_info),
            stopped: isAppDisabled(app.webAppInfo.installed_app_info),
            paused: false,
          }"
          class="app-status"
          style="margin-right: 29px"
          tabindex="0"
        ></div>
      </sl-tooltip>
      <!-- ----------------- -->

      <!-- Installed App Id -->
      <div class="installed-app-name">
        {{ app.webAppInfo.installed_app_info.installed_app_id }}
      </div>
      <!-- EO Installed App Id ---------------- -->
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { HolochainAppInfo, HolochainAppInfoExtended } from "../types";
import { isAppRunning, isAppDisabled, isAppPaused, getReason, flattenCells, getCellId } from "../utils";
import { writeText } from "@tauri-apps/api/clipboard";
import { CellInfo, CellType, ClonedCell, encodeHashToBase64, NetworkInfo } from "@holochain/client";

import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/themes/light.css";
// import "@holochain-open-dev/utils/dist/holo-identicon";
import HoloIdenticon from "../components/subcomponents/HoloIdenticon.vue";

import HCGenericDialog from "./subcomponents/HCGenericDialog.vue";
import InstalledCellCard from "./subcomponents/InstalledCellCard.vue";
import DisabledCloneCard from "./subcomponents/DisabledCloneCard.vue";
import { APPSTORE_APP_ID } from "../constants";


export default defineComponent({
  name: "InstalledAppCard",
  components: {
    HCGenericDialog,
    HoloIdenticon,
    InstalledCellCard,
    DisabledCloneCard,
  },
  props: {
    app: {
      type: Object as PropType<HolochainAppInfoExtended>,
      required: true,
    },
  },
  data(): {
    showMore: boolean;
    showUninstallDialog: boolean;
    showPubKeyTooltip: boolean;
    gossipInfo: Record<string, NetworkInfo>;
    showProvisionedCells: boolean;
    showClonedCells: boolean;
    showDisabledClonedCells: boolean;
  } {
    return {
      showMore: false,
      showUninstallDialog: false,
      showPubKeyTooltip: false,
      gossipInfo: {},
      showProvisionedCells: true,
      showClonedCells: false,
      showDisabledClonedCells: false,
    };
  },
  emits: ["openApp", "enableApp", "disableApp", "startApp", "uninstallApp", "updateGui"],
  computed: {
    provisionedCells(): [string, CellInfo][] {
      const provisionedCells = flattenCells(this.app.webAppInfo.installed_app_info.cell_info)
        .filter(([_roleName, cellInfo]) => "provisioned" in cellInfo)
        .sort(([roleName_a, _cellInfo_a], [roleName_b, _cellInfo_b]) => roleName_a.localeCompare(roleName_b));
      return provisionedCells
    },
    enabledClonedCells(): [string, CellInfo][] {
      return flattenCells(this.app.webAppInfo.installed_app_info.cell_info)
        .filter(([_roleName, cellInfo]) => "cloned" in cellInfo)
        .filter(([_roleName, cellInfo]) => (cellInfo as { [CellType.Cloned]: ClonedCell }).cloned.enabled)
        .sort(([roleName_a, _cellInfo_a], [roleName_b, _cellInfo_b]) => roleName_a.localeCompare(roleName_b));
    },
    disabledClonedCells(): [string, CellInfo][] {
      return flattenCells(this.app.webAppInfo.installed_app_info.cell_info)
        .filter(([_roleName, cellInfo]) => "cloned" in cellInfo)
        .filter(([_roleName, cellInfo]) => !(cellInfo as { [CellType.Cloned]: ClonedCell }).cloned.enabled)
        .sort(([roleName_a, _cellInfo_a], [roleName_b, _cellInfo_b]) => roleName_a.localeCompare(roleName_b));
    },
    isSliderOn() {
      return (isAppRunning(this.app.webAppInfo.installed_app_info) || isAppPaused(this.app.webAppInfo.installed_app_info));
    },
  },
  methods: {
    encodeHashToBase64,
    getReason,
    isAppRunning,
    isAppDisabled,
    isAppPaused,
    writeText,
    getCellId,
    isAppHeadless(app: HolochainAppInfo) {
      return app.webAppInfo.web_uis.default.type === "Headless";
    },
    requestUninstall() {
      (this.$refs["uninstall-app-dialog"] as typeof HCGenericDialog).open();
      this.showUninstallDialog = true;
    },
    async enableApp(app: HolochainAppInfo) {
      this.$emit("enableApp", app);
    },
    async disableApp(app: HolochainAppInfo) {
      this.$emit("disableApp", app);
    },
    async startApp(app: HolochainAppInfo) {
      this.$emit("startApp", app);
    },
    async uninstallApp(app: HolochainAppInfo) {
      this.showUninstallDialog = false;
      this.$emit("uninstallApp", app);
    },
    getAppStatus(app: HolochainAppInfo) {
      if (isAppRunning(app.webAppInfo.installed_app_info) || isAppPaused(app.webAppInfo.installed_app_info)) {
        return "Running";
      }
      if (isAppDisabled(app.webAppInfo.installed_app_info)) {
        return "Disabled";
      }
      // Currently this won't be called as paused and running are conflated both into running
      // because app status is not getting updated: https://github.com/holochain/holochain/issues/1580#issuecomment-1377471698
      if (isAppPaused(app.webAppInfo.installed_app_info)) {
        return "Offline/Paused";
      }
      return "Unknown State";
    },
    isAppUninstallable(installedAppId: string) {
      return installedAppId !== APPSTORE_APP_ID;
    },
    async handleSlider(app: HolochainAppInfo) {
      if (isAppRunning(app.webAppInfo.installed_app_info) || isAppPaused(app.webAppInfo.installed_app_info)) {
        await this.disableApp(app);
      } else if (isAppDisabled(app.webAppInfo.installed_app_info)) {
        await this.enableApp(app);
      } else if (isAppPaused(app.webAppInfo.installed_app_info)) {
        // Currently this won't be called as paused and running are conflated both into running
        // because app status is not getting updated: https://github.com/holochain/holochain/issues/1580#issuecomment-1377471698
        await this.startApp(app);
      } else {
        throw new Error("Unknown App state.");
      }
    },
    copyPubKey() {
      const pubKey =
        this.getPubKey();
      this.writeText(encodeHashToBase64(new Uint8Array(pubKey)));
      this.showPubKeyTooltip = true;
      setTimeout(() => {
        this.showPubKeyTooltip = false;
      }, 1200);
    },
    getPubKey() {
      const cell = Object.values(this.app.webAppInfo.installed_app_info.cell_info)[0]
        .find((c) => "provisioned" in c);

      if (!cell || !("provisioned" in cell)) {
        throw new Error("no provisioned cell found");
      }

      return cell.provisioned.cell_id[1];
    },
  },
});
</script>

<style scoped>
.container {
  position: relative;
  display: flex;
  flex: 1;
  flex-direction: column;
  align-items: center;
  background: #ffffff;
  border-radius: 22px;
  width: 120px;
  margin: 8px;
  /* box-shadow: 0 0 2px rgb(131, 128, 176); */
  box-shadow: 0 0px 5px #9b9b9b;
}

.btn {
  margin: 5px;
}

.tooltip {
  --show-delay: 1000;
}

.tooltip::part(base) {
  font-family: "Poppins";
}

.appIcon {
  display: flex;
  width: 120px;
  height: 120px;
  padding: 0;
  border-radius: 22px;
  object-fit: cover;
  cursor: pointer;
}

.appIconMore {
  display: flex;
  width: 120px;
  height: 120px;
  padding: 0;
  border-radius: 22px 0 22px 0;
  object-fit: cover;
}

.holoIdenticon {
  border-radius: 12px;
}

.holoIdenticonMore {
  border-radius: 12px 0 22px 0;
}

.app-status {
  height: 10px;
  width: 10px;
  border-radius: 50%;
}

.running {
  background-color: rgb(0, 185, 0);
}

.stopped {
  background-color: rgb(220, 0, 0);
}

.paused {
  background-color: rgb(175, 175, 175);
}

.tooltip {
  position: absolute;
  /* color: #482edf; */
  color: white;
  bottom: 56px;
  left: 62px;
  background: #5537fc;
  border-radius: 5px;
  /* border: 2px solid #482edf; */
  padding: 1px 7px;
}

.update-button {
  font-weight: bold;
  color: black;
  cursor: pointer;
  border: 2px solid black;
  border-radius: 4px;
  padding: 0 5px;
  margin-right: 29px;
  opacity: 0.85;
}

.update-button:hover {
  opacity: 0.6;
}

.installed-app-name {
  width: 120px;
  text-align: center;
  font-size: 18px;
  font-weight: 700;
  word-break: break-all;
}
</style>
