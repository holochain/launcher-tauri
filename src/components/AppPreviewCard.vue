<template>
  <div class="column card">
    <div
      v-if="showDescription"
      class="column"
      style="flex: 1; overflow-y: auto; padding: 20px"
    >
      <div style="font-weight: bold">Compatibility:</div>
      <div v-if="holochainVersion" class="row">
        <div style="width: 160px">Holochain version:</div>
        <div>{{ holochainVersion }}</div>
      </div>
      <div class="row">
        <div style="width: 160px">HDK version:</div>
        <div>{{ getLatestRelease(app).content.hdk_version }}</div>
      </div>
      <div style="font-weight: bold; margin-top: 10px">Description:</div>
      {{ app.app.content.description }}
    </div>

    <div v-else class="column" style="flex: 1">
      <div class="row" style="align-items: center">
        <!-- if icon provided -->
        <img
          v-if="appIcon"
          :src="appIcon"
          alt="app icon"
          style="
            width: 80px;
            min-width: 80px;
            height: 80px;
            border-radius: 12px;
            object-fit: cover;
            margin: 15px;
          "
        />
        <!-- if no icon provided -->
        <div
          v-else
          class="column center-content"
          style="
            width: 80px;
            min-width: 80px;
            height: 80px;
            border-radius: 12px;
            background: #372ba5;
            margin: 15px;
          "
        >
          <div style="color: white; font-size: 40px; font-weight: 600">
            {{ app.app.content.title.slice(0, 2) }}
          </div>
        </div>

        <div class="column">
          <div
            style="
              font-size: 25px;
              font-weight: 600;
              margin-right: 15px;
              margin-bottom: 8px;
              line-height: 115%;
              word-break: break-all;
            "
          >
            {{ app.app.content.title }}
          </div>
          <div style="margin-top: -5px">
            {{ getLatestRelease(app).content.name }}
          </div>
        </div>
      </div>
      <div
        style="
          display: flex;
          flex: 1;
          margin: 0 20px 0 25px;
          color: rgba(0, 0, 0, 0.6);
          font-size: 17px;
          overflow-y: auto;
        "
      >
        {{ app.app.content.subtitle }}
      </div>
    </div>

    <div class="row" style="justify-content: flex-end; align-items: center">
      <HCMoreToggle
        style="margin-left: 22px; margin-bottom: 5px"
        title="Details"
        @click="showDescription = !showDescription"
      />
      <span style="display: flex; flex: 1"></span>
      <HCButton
        class="install-btn"
        style="border-radius: 12px; margin: 8px"
        @click="$emit('install-app')"
        >Install</HCButton
      >
    </div>
  </div>
</template>

<script lang="ts">
import { AppWithReleases, getLatestRelease } from "@/devhub/get-happs";
import { HolochainVersion } from "@/types";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent, PropType } from "vue";

import HCButton from "./subcomponents/HCButton.vue";
import HCMoreToggle from "./subcomponents/HCMoreToggle.vue";

export default defineComponent({
  name: "AppPreviewCard",
  components: { HCButton, HCMoreToggle },
  props: {
    appIcon: {
      type: String,
    },
    app: {
      type: Object as PropType<AppWithReleases>,
      required: true,
    },
  },
  data(): {
    showDescription: boolean;
    holochainVersion: HolochainVersion | undefined;
  } {
    return {
      showDescription: false,
      holochainVersion: undefined,
    };
  },
  emits: ["installApp"],
  async mounted() {
    this.holochainVersion = await invoke("choose_version_for_hdk", {
      hdkVersion: getLatestRelease(this.app).content.hdk_version,
    });
  },
  methods: {
    getLatestRelease,
  },
});
</script>

<style scoped>
.card {
  width: 340px;
  height: 220px;
  background: white;
  border-radius: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
}

.install-btn:hover {
  background-color: #674df9;
}
</style>
