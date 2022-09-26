<template>
  <div
    style="
      position: relative;
      display: flex;
      flex-direction: row;
      align-items: center;
      background: #ffffff;
      border: 1px solid #e1e1e1;
      border-radius: 25px;
      height: 120px;
      max-width: 900px;
      min-width: 900px;
      margin: 10px;
    "
  >
    <img v-if="appIcon" class="app-icon" :src="`${this.appIcon}`" />
    <div v-else class="app-icon" style="background-color: #49209e"></div>

    <div
      style="
        display: flex;
        font-size: 23px;
        font-weight: 700;
        margin-left: 40px;
      "
    >
      {{ app.installed_app_info.installed_app_id }}
    </div>
    <span style="flex: 1"></span>

    <div
      :class="{
        running: isAppRunning(app.installed_app_info),
        stopped: isAppDisabled(app.installed_app_info),
        paused: isAppPaused(app.installed_app_info),
      }"
      class="app-status"
      style="margin-right: 18px"
    ></div>

    <ToggleSwitch
      style="margin-right: 29px"
      :sliderOn="isAppRunning(app.installed_app_info)"
    />

    <img
      v-if="isAppRunning(app.installed_app_info) && !isAppHeadless(app)"
      style="margin-right: 29px; width: 24px; cursor: pointer"
      src="/img/Open_App.svg"
      @click="$emit('openApp', this.app.installed_app_info.installed_app_id)"
    />
    <div
      style="margin-right: 33px; width: 28px; height: 28px; cursor: pointer"
      @click="this.showMore = !this.showMore"
    >
      <img
        style="width: 28px"
        :class="{ rotated: this.showMore }"
        src="/img/More.svg"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { InstalledWebAppInfo } from "../types";
import { isAppRunning, isAppDisabled, isAppPaused } from "../utils";

import ToggleSwitch from "./subcomponents/ToggleSwitch.vue";

export default defineComponent({
  name: "InstalledAppCard",
  components: { ToggleSwitch },
  props: {
    appIcon: {
      type: String,
    },
    app: {
      type: Object as PropType<InstalledWebAppInfo>,
      required: true,
    },
  },
  data(): {
    showMore: boolean;
  } {
    return {
      showMore: false,
    };
  },
  emits: ["openApp"],
  methods: {
    isAppRunning,
    isAppDisabled,
    isAppPaused,
    isAppHeadless(app: InstalledWebAppInfo) {
      return app.web_ui_info.type === "Headless";
    },
  },
});
</script>

<style scoped>
.app-icon {
  display: flex;
  width: 120px;
  height: 120px;
  padding: 0;
  border-radius: 25px 0 0 25px;
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

.rotated {
  transform: rotate(90deg);
}
</style>
