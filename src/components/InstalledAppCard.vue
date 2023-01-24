<template>
  <HCGenericDialog
    @confirm="uninstallApp(app)"
    closeOnSideClick
    ref="uninstall-app-dialog"
    primaryButtonLabel="Uninstall"
    ><div style="text-align: center">
      Are you sure you want to uninstall this App? This will irrevocably delete
      all data stored in it.
    </div>
  </HCGenericDialog>

  <div class="container">
    <div
      style="
        position: relative;
        display: flex;
        flex-direction: row;
        align-items: center;
        width: 100%;
        height: 120px;
      "
    >
    <!-- App Logo with Holo Identicon -->
      <div style="position: relative">
        <!-- assumes same agent pub key for all cells (just taking the first one) -->
        <div v-show="showPubKeyTooltip" class="tooltip">Copied!</div>
        <HoloIdenticon
          title="Your Public Key"
          :class="{ holoIdenticon: !showMore, holoIdenticonMore: showMore }"
          style="position: absolute; top: 78px; left: 78px; cursor: pointer"
          :hash="getPubKey()"
          :size="42"
          @click="copyPubKey()"
        ></HoloIdenticon>

        <img
          v-if="appIcon"
          :class="{ appIcon: !showMore, appIconMore: showMore }"
          :src="`${appIcon}`"
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



      <!-- Installed App Id -->
      <div
        style="
          display: flex;
          flex: 1;
          font-size: 23px;
          font-weight: 700;
          margin-left: 40px;
          margin-right: 30px;
          word-break: break-all;
        "
      >
        {{ app.webAppInfo.installed_app_info.installed_app_id }}
      </div>
      <!-- ---------------- -->

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
            running: isAppRunning(app.webAppInfo.installed_app_info),
            stopped: isAppDisabled(app.webAppInfo.installed_app_info),
            paused: isAppPaused(app.webAppInfo.installed_app_info),
          }"
          class="app-status"
          style="margin-right: 29px"
          tabindex="0"
        ></div>
      </sl-tooltip>
      <!-- ----------------- -->

      <!-- Open App Icon Button -->
      <div
        v-if="
          isAppRunning(app.webAppInfo.installed_app_info) && !isAppHeadless(app)
        "
        style="display: flex"
      >
        <sl-tooltip class="tooltip" hoist placement="top" content="Open App">
          <img
            tabindex="0"
            style="margin-right: 29px; width: 24px; cursor: pointer"
            src="/img/Open_App.svg"
            @click="$emit('openApp', app)"
            v-on:keyup.enter="$emit('openApp', app)"
          />
        </sl-tooltip>
      </div>
      <!-- ------------------- -->

      <!-- Disable/enable switch -->
      <sl-tooltip
        class="tooltip"
        hoist
        placement="top"
        :content="
          isAppRunning(app.webAppInfo.installed_app_info)
            ? 'Disable App'
            : 'Start App'
        "
      >
        <ToggleSwitch
          v-if="
            isAppUninstallable(
              app.webAppInfo.installed_app_info.installed_app_id
            )
          "
          style="margin-right: 29px"
          :sliderOn="isSliderOn"
          @click="handleSlider(app)"
          @keydown.enter="handleSlider(app)"
        />
      </sl-tooltip>
      <!-- ------------------- -->

      <!-- Triple dot icon to show app details -->
      <sl-tooltip class="tooltip" hoist placement="top" content="App Details">
        <HCMoreToggle
          @toggle="showMore = !showMore"
          style="margin-right: 33px"
          tabindex="0"
        />
      </sl-tooltip>
      <!-- ------------------- -->
    </div>



    <!-------------- App details --------------->
    <div
      v-if="showMore"
      class="column"
      style="align-items: left; width: 100%; margin-bottom: 20px"
    >
      <div class="row" style="margin-top: 45px; margin-left: 140px">
        <span style="margin-right: 10px; font-weight: bold; font-size: 1em"
          >Holochain Version:</span
        >
        <span style="opacity: 0.7; font-family: monospace: font-size: 1em;">{{
          app.holochainId.type === "CustomBinary"
            ? "Custom Binary"
            : app.holochainId.content
        }}</span>
        <!-- <span style="flex: 1;"></span>
        <img
          src="/img/refresh.png"
          title="Refresh"
          @click="refresh"
          style="width: 20px; height: 20px; margin-right: 30px; cursor: pointer;"
        > -->
      </div>

      <!-- provisioned cells -->
      <div
        class="row"
        style="margin-top: 20px; margin-left: 140px; margin-right: 30px"
      >
        <span style="margin-right: 10px; font-weight: bold; font-size: 1em"
          >Provisioned Cells:</span
        ><span style="display: flex; flex: 1"></span>
        <span
          style="opacity: 0.7; cursor: pointer; font-size: 0.8em"
          @click="showProvisionedCells = !showProvisionedCells"
          >{{ showProvisionedCells ? "[Hide]" : "[Show]" }}
        </span>
      </div>
      <div v-if="showProvisionedCells" style="margin-left: 140px; margin-right: 20px">
        <InstalledCellCard
          v-for="[roleName, cellInfo] in provisionedCells"
          :key="roleName"
          style="margin: 12px 0"
          :cellInfo="cellInfo"
          :roleName="roleName"
          :holochainId="app.holochainId"
        >
        </InstalledCellCard>
      </div>

      <!-- cloned cells -->
      <div
        class="row"
        style="margin-top: 20px; margin-left: 140px; margin-right: 30px"
      >
        <span style="margin-right: 10px; font-weight: bold; font-size: 1em"
          >Cloned Cells:</span
        ><span style="display: flex; flex: 1"></span>
        <span
          style="opacity: 0.7; cursor: pointer; font-size: 0.8em"
          @click="showClonedCells = !showClonedCells"
          >{{ showClonedCells ? "[Hide]" : "[Show]" }}
        </span>
      </div>
      <div
        v-if="showClonedCells"
        style="margin-left: 140px; margin-right: 20px"
      >
        <div v-if="clonedCells.length > 0">
          <InstalledCellCard
            v-for="[roleName, cellInfo] in clonedCells"
            :key="roleName"
            style="margin: 12px 0"
            :cellInfo="cellInfo"
            :roleName="roleName"
            :holochainId="app.holochainId"
          >
          </InstalledCellCard>
        </div>

        <div v-else style="text-align: center; opacity: 0.7">
          There are no cloned cells in this app.
        </div>
      </div>
      <span
        v-if="getReason(app.webAppInfo.installed_app_info)"
        style="margin-top: 16px; margin-left: 140px"
      >
        {{ getReason(app.webAppInfo.installed_app_info) }}
      </span>

      <div
        style="
          display: flex;
          flex-direction: row;
          justify-content: flex-end;
          margin-top: 40px;
          margin-right: 20px;
        "
      >
        <HCButton
          class="btn"
          style="--hc-primary-color: #d80d0d"
          @click="requestUninstall"
          v-if="
            isAppUninstallable(
              app.webAppInfo.installed_app_info.installed_app_id
            )
          "
          outlined
          >Uninstall
        </HCButton>

        <HCButton
          style="--hc-primary-color: #dd821a"
          v-if="
            !isAppDisabled(app.webAppInfo.installed_app_info) &&
            isAppUninstallable(
              app.webAppInfo.installed_app_info.installed_app_id
            )
          "
          outlined
          @click="disableApp(app)"
          >Disable
        </HCButton>
        <HCButton
          style="--hc-primary-color: #008704"
          v-if="isAppDisabled(app.webAppInfo.installed_app_info)"
          @click="enableApp(app)"
          outlined
          >Enable
        </HCButton>
        <HCButton
          style="--hc-primary-color: #008704"
          v-if="isAppPaused(app.webAppInfo.installed_app_info)"
          @click="startApp(app)"
          outlined
          >Start
        </HCButton>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { HolochainAppInfo, HolochainAppInfoExtended } from "../types";
import { isAppRunning, isAppDisabled, isAppPaused, getReason, flattenCells, getCellId } from "../utils";
import { writeText } from "@tauri-apps/api/clipboard";
import { CellInfo, encodeHashToBase64, NetworkInfo } from "@holochain/client";

import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/themes/light.css";
// import "@holochain-open-dev/utils/dist/holo-identicon";
import HoloIdenticon from "../components/subcomponents/HoloIdenticon.vue";

import ToggleSwitch from "./subcomponents/ToggleSwitch.vue";
import HCButton from "./subcomponents/HCButton.vue";
import HCMoreToggle from "./subcomponents/HCMoreToggle.vue";
import HCGenericDialog from "./subcomponents/HCGenericDialog.vue";
import InstalledCellCard from "./subcomponents/InstalledCellCard.vue";

export default defineComponent({
  name: "InstalledAppCard",
  components: {
    ToggleSwitch,
    HCButton,
    HCMoreToggle,
    HCGenericDialog,
    HoloIdenticon,
    InstalledCellCard,
  },
  props: {
    appIcon: {
      type: String,
    },
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
  } {
    return {
      showMore: false,
      showUninstallDialog: false,
      showPubKeyTooltip: false,
      gossipInfo: {},
      showProvisionedCells: true,
      showClonedCells: true,
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
    clonedCells(): [string, CellInfo][] {
      return flattenCells(this.app.webAppInfo.installed_app_info.cell_info)
        .filter(([_roleName, cellInfo]) => "cloned" in cellInfo)
        .sort(([roleName_a, _cellInfo_a], [roleName_b, _cellInfo_b]) => roleName_a.localeCompare(roleName_b));
    },
    isSliderOn() {
      return isAppRunning(this.app.webAppInfo.installed_app_info);
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
      if (isAppRunning(app.webAppInfo.installed_app_info)) {
        return "Running";
      }
      if (isAppDisabled(app.webAppInfo.installed_app_info)) {
        return "Disabled";
      }
      if (isAppPaused(app.webAppInfo.installed_app_info)) {
        return "Paused";
      }
      return "Unknown State";
    },
    isAppUninstallable(installedAppId: string) {
      const _hdiOfDevhub = this.$store.getters["hdiOfDevhub"];
      const holochainId = this.$store.getters["holochainIdForDevhub"];

      return installedAppId !== `DevHub-${holochainId.content}`;
    },
    async handleSlider(app: HolochainAppInfo) {
      if (isAppRunning(app.webAppInfo.installed_app_info)) {
        await this.disableApp(app);
      } else if (isAppDisabled(app.webAppInfo.installed_app_info)) {
        await this.enableApp(app);
      } else if (isAppPaused(app.webAppInfo.installed_app_info)) {
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
  width: 100%;
  max-width: 1100px;
  min-width: 900px;
  margin: 8px;
  /* box-shadow: 0 0 2px rgb(131, 128, 176); */
  box-shadow: 0 0px 5px #9b9b9b;
}

.btn {
  width: 80px;
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
  border-radius: 22px 0 0 22px;
  object-fit: cover;
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
  border-radius: 12px 0 0 0;
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

</style>
