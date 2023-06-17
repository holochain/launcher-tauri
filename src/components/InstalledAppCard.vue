<template>
  <div
    :class="getAppStatus(app) === ('Disabled' || 'Offline/Paused') ? 'disabled' : undefined"
    >
      <!-- App Logo -->
      <div
        class="icon-container"
        :class="getAppStatus(app) === ('Disabled' || 'Offline/Paused') ? 'container-disabled': undefined"
        style="position: relative"
        tabindex="0"
        @click="handleClick()"
        v-on:keyup.enter="handleClick()"
        :title="`${getAppStatus(app) === 'Disabled' ? 'This app is diabled - Go to Settings to enable this app' : ''}${getAppStatus(app) === 'Offline/Paused' ? ' (OFFLINE/PAUSED)' : ''}`"
      >
        <img
          v-if="app.webAppInfo.icon_src"
          class="appIcon"
          :class="getAppStatus(app) === 'Running' ? 'pointer': 'cursor-default'"
          :src="`${app.webAppInfo.icon_src}`"
        />
        <div
          v-else
          class="appIcon column center-content"
          :class="getAppStatus(app) === 'Running' ? 'pointer': 'cursor-default'"
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
      class="installed-app-name"
      :title="`${app.webAppInfo.installed_app_info.installed_app_id}${getAppStatus(app) === 'Disabled' ? ' (DISABLED)' : ''}${getAppStatus(app) === 'Offline/Paused' ? ' (OFFLINE/PAUSED)' : ''}`"
    >
      {{ app.webAppInfo.installed_app_info.installed_app_id.slice(0,20) }}{{ app.webAppInfo.installed_app_info.installed_app_id.length > 20 ? '...' : '' }}
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { HolochainAppInfo, HolochainAppInfoExtended } from "../types";
import { isAppRunning, isAppDisabled, isAppPaused, getReason } from "../utils";

import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/themes/light.css";
// import "@holochain-open-dev/utils/dist/holo-identicon";
import HoloIdenticon from "../components/subcomponents/HoloIdenticon.vue";

import HCGenericDialog from "./subcomponents/HCGenericDialog.vue";
import InstalledCellCard from "./subcomponents/InstalledCellCard.vue";
import DisabledCloneCard from "./subcomponents/DisabledCloneCard.vue";


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
    showPubKeyTooltip: boolean;
  } {
    return {
      showPubKeyTooltip: false,
    };
  },
  emits: ["openApp"],
  methods: {
    getReason,
    isAppRunning,
    isAppDisabled,
    isAppPaused,
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
    handleClick() {
      if (!isAppDisabled(this.app.webAppInfo.installed_app_info)) {
        this.$emit('openApp', this.app)
      }
    }
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
  margin: 8px;
  /* box-shadow: 0 0 2px rgb(131, 128, 176); */
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
}

.pointer {
  cursor: pointer;
}

.cursor-default {
  cursor: default;
}

.icon-container {
  width: 120px;
  box-shadow: 0 0px 5px #9b9b9b;
  border-radius: 22px;
}
.icon-container:not(.container-disabled):hover {
  box-shadow: 0 0px 12px #676767;
}

.icon-container:not(.container-disabled):focus {
  box-shadow: 0 0px 12px #676767;
}

.disabled {
  opacity: 0.3;
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

.installed-app-name {
  width: 120px;
  text-align: center;
  font-size: 18px;
  font-weight: 700;
  word-break: break-all;
}
</style>
