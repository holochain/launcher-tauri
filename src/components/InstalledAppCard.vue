<template>
  <HCGenericDialog
    @confirm="uninstallApp(app)"
    closeOnSideClick
    ref="uninstall-app-dialog"
    text="Are you sure you want to uninstall this App? This will irrevocably delete all data stored in it."
    primaryButtonLabel="Uninstall"
  />
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
      <img
        v-if="appIcon"
        :class="{ appIcon: !showMore, appIconMore: showMore }"
        :src="`${appIcon}`"
      />
      <div
        v-else
        :class="{ appIcon: !showMore, appIconMore: showMore }"
        style="background-color: #49209e"
      ></div>

      <div
        style="
          display: flex;
          font-size: 23px;
          font-weight: 700;
          margin-left: 40px;
        "
      >
        {{ app.webAppInfo.installed_app_info.installed_app_id }}
      </div>
      <span style="flex: 1"></span>

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
          style="margin-right: 18px"
          tabindex="0"
        ></div>
      </sl-tooltip>

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
          style="margin-right: 29px"
          :sliderOn="isAppRunning(app.webAppInfo.installed_app_info)"
          @click="handleSlider(app)"
        />
      </sl-tooltip>

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

      <sl-tooltip class="tooltip" hoist placement="top" content="App Details">
        <HCMoreToggle
          @toggle="showMore = !showMore"
          style="margin-right: 33px"
          tabindex="0"
        />
      </sl-tooltip>
    </div>

    <div
      v-if="showMore"
      class="column"
      style="align-items: left; width: 100%; margin-bottom: 20px"
    >
      <div class="row" style="margin-top: 35px; margin-left: 25px">
        <span style="margin-right: 10px; font-weight: bold; font-size: 1em"
          >Your Public Key:</span
        >
        <span style="opacity: 0.7; font-family: monospace: font-size: 1em;">{{
          serializeHash(
            app.webAppInfo.installed_app_info.cell_data[0].cell_id[1]
          )
        }}</span>
      </div>

      <div class="row" style="margin-top: 20px; margin-left: 25px">
        <span style="margin-right: 10px; font-weight: bold; font-size: 1em"
          >Holochain Version:</span
        >
        <span style="opacity: 0.7; font-family: monospace: font-size: 1em;">{{
          app.holochainId.type === "CustomBinary"
            ? "Custom Binary"
            : app.holochainId.content
        }}</span>
      </div>

      <table style="text-align: left; margin-top: 20px; margin-left: 25px">
        <tr>
          <th>Cell Role</th>
          <th>Dna Hash</th>
        </tr>

        <tr
          style=""
          v-for="cellData in app.webAppInfo.installed_app_info.cell_data"
          :key="[...cellData.cell_id[0], ...cellData.cell_id[1]]"
        >
          <td>
            <span
              >{{ cellData.role_id.slice(0, 20)
              }}{{ cellData.role_id.length > 20 ? "..." : "" }}</span
            >
          </td>
          <td>
            <span
              style="opacity: 0.7; font-family: monospace; font-size: 14px"
              >{{ serializeHash(cellData.cell_id[0]) }}</span
            >
          </td>
        </tr>
      </table>

      <span
        v-if="getReason(app.webAppInfo.installed_app_info)"
        style="margin-top: 16px; margin-left: 25px"
      >
        {{ getReason(app.webAppInfo.installed_app_info) }}
      </span>

      <div
        style="
          display: flex;
          flex-direction: row;
          justify-content: end;
          margin-top: 16px;
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
import { HolochainAppInfo } from "../types";
import { serializeHash } from "@holochain-open-dev/utils";
import { isAppRunning, isAppDisabled, isAppPaused, getReason } from "../utils";
import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";

import ToggleSwitch from "./subcomponents/ToggleSwitch.vue";
import HCButton from "./subcomponents/HCButton.vue";
import HCMoreToggle from "./subcomponents/HCMoreToggle.vue";
import HCGenericDialog from "./subcomponents/HCGenericDialog.vue";

export default defineComponent({
  name: "InstalledAppCard",
  components: { ToggleSwitch, HCButton, HCMoreToggle, HCGenericDialog },
  props: {
    appIcon: {
      type: String,
    },
    app: {
      type: Object as PropType<HolochainAppInfo>,
      required: true,
    },
  },
  data(): {
    showMore: boolean;
    showUninstallDialog: boolean;
  } {
    return {
      showMore: false,
      showUninstallDialog: false,
    };
  },
  emits: ["openApp", "enableApp", "disableApp", "startApp", "uninstallApp"],
  methods: {
    serializeHash,
    getReason,
    isAppRunning,
    isAppDisabled,
    isAppPaused,
    isAppHeadless(app: HolochainAppInfo) {
      return app.webAppInfo.web_ui_info.type === "Headless";
    },
    requestUninstall() {
      (this.$refs["uninstall-app-dialog"] as typeof HCGenericDialog).open();
      console.log("requesting uninstall.");
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
      const holochainId = this.$store.getters["holochainIdForDevhub"];

      return installedAppId !== `DevHub-${holochainId.content}`;
    },
    async handleSlider(app: HolochainAppInfo) {
      if (isAppRunning(app.webAppInfo.installed_app_info)) {
        console.log("trying to disable app");
        await this.disableApp(app);
      } else if (isAppDisabled(app.webAppInfo.installed_app_info)) {
        await this.enableApp(app);
      } else if (isAppPaused(app.webAppInfo.installed_app_info)) {
        await this.startApp(app);
      } else {
        throw new Error("Unknown App state.");
      }
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
  border-radius: 25px;
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
  border-radius: 25px 0 0 25px;
  object-fit: cover;
}

.appIconMore {
  display: flex;
  width: 120px;
  height: 120px;
  padding: 0;
  border-radius: 25px 0 25px 0;
  object-fit: cover;
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
</style>
