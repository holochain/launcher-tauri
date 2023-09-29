<template>
  <div
    :class="
      getAppStatus(app) === ('Disabled' || 'Offline/Paused')
        ? 'disabled'
        : undefined
    "
    style="position: relative"
  >
    <!-- App Logo -->
    <div
      class="icon-container"
      :class="
        getAppStatus(app) === ('Disabled' || 'Offline/Paused')
          ? 'container-disabled'
          : undefined
      "
      style="position: relative"
      tabindex="0"
      @click="handleClick()"
      @keyup.enter="handleClick()"
      :title="`${
        getAppStatus(app) === 'Disabled'
          ? 'This app is disabled - Go to Settings to enable this app'
          : app.webAppInfo.installed_app_info.installed_app_id
      }${getAppStatus(app) === 'Offline/Paused' ? ' (OFFLINE/PAUSED)' : ''}`"
    >
      <img
        v-if="app.webAppInfo.icon_src"
        class="appIcon"
        :class="getAppStatus(app) === 'Running' ? 'pointer' : 'cursor-default'"
        :src="`${app.webAppInfo.icon_src}`"
      />
      <div
        v-else
        class="appIcon column center-content"
        style="background-color: #372ba5"
        :class="getAppStatus(app) === 'Running' ? 'pointer' : 'cursor-default'"
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
      :title="`${app.webAppInfo.installed_app_info.installed_app_id}${
        getAppStatus(app) === 'Disabled' ? ' (DISABLED)' : ''
      }${getAppStatus(app) === 'Offline/Paused' ? ' (OFFLINE/PAUSED)' : ''}`"
    >
      <!-- {{ app.webAppInfo.installed_app_info.installed_app_id.slice(0,20) }}{{ app.webAppInfo.installed_app_info.installed_app_id.length > 20 ? '...' : '' }} -->
      ({{ unreadNotifications.length }}){{
        app.webAppInfo.installed_app_info.installed_app_id
      }}
    </div>

    <!-- notification dot -->
    <div
      class="notification-dot"
      v-if="
        $store.state.notificationState[
          app.webAppInfo.installed_app_info.installed_app_id
        ] &&
        $store.state.notificationState[
          app.webAppInfo.installed_app_info.installed_app_id
        ].length > 0
      "
    >
      <span style="padding: 0 5px">{{
        $store.state.notificationState[
          app.webAppInfo.installed_app_info.installed_app_id
        ].length
      }}</span>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { HolochainAppInfo, HolochainAppInfoExtended } from "../types";
import {
  isAppRunning,
  isAppDisabled,
  isAppPaused,
  getReason,
  readUnreadHappNotifications,
} from "../utils";

import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/themes/light.css";

import { HappNotification } from "@holochain/launcher-api";

export default defineComponent({
  name: "InstalledAppCard",
  props: {
    app: {
      type: Object as PropType<HolochainAppInfoExtended>,
      required: true,
    },
  },
  data(): {
    showPubKeyTooltip: boolean;
    unreadNotifications: Array<HappNotification>;
  } {
    return {
      showPubKeyTooltip: false,
      unreadNotifications: [],
    };
  },
  emits: ["openApp"],
  mounted() {
    this.unreadNotifications = readUnreadHappNotifications(
      this.app.webAppInfo.installed_app_info.installed_app_id
    );
  },
  methods: {
    getReason,
    isAppRunning,
    isAppDisabled,
    isAppPaused,
    getAppStatus(app: HolochainAppInfo) {
      if (
        isAppRunning(app.webAppInfo.installed_app_info) ||
        isAppPaused(app.webAppInfo.installed_app_info)
      ) {
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
        this.$emit("openApp", this.app);
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
  margin: 8px;
  /* box-shadow: 0 0 2px rgb(131, 128, 176); */
}

.pointer {
  cursor: pointer;
}
.cursor-default {
  cursor: default;
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

.notification-dot {
  display: flex;
  align-items: center;
  justify-content: center;
  position: absolute;
  top: -8px;
  right: -11px;
  font-weight: bold;
  background: #faf035;
  border-radius: 16px;
  height: 32px;
  min-width: 32px;
  box-shadow: 0 0 4px rgb(9, 9, 95);
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
  word-break: normal;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
