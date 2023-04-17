<template>
  <div class="column card">
    <div
      v-if="showDescription"
      class="column"
      style="flex: 1; overflow-y: auto; padding: 20px"
    >
      <div style="font-weight: bold">Version details:</div>
      <!-- <div v-if="holochainVersion" class="row">
        <div style="width: 160px">Holochain version:</div>
        <div>{{ holochainVersion }}</div>
      </div>
      <div class="row">
        <div style="width: 160px">HDK version:</div>
        <div>{{ getLatestRelease(app).content.hdk_version }}</div>
      </div> -->
      <div class="row">
        <div style="width: 160px">hApp version:</div>
        <div>[NOT IMPLEMENTED]</div>
      </div>
      <div class="row">
        <div style="width: 160px">UI version:</div>
        <div v-if="guiVersion">{{ guiVersion }}</div>
        <div v-else style="font-size: 0.9em; opacity: 0.7;">loading...</div>
      </div>
      <div style="font-weight: bold; margin-top: 10px">Description:</div>
      {{ app.description }}
    </div>

    <div v-else class="column" style="flex: 1">
      <div class="row" style="align-items: center">
        <!-- if icon provided -->
        <img
          v-if="appIcon"
          :src="appIconSrc()"
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
            {{ app.title.slice(0, 2) }}
          </div>
        </div>

        <div class="column" style="overflow: hidden;">
          <div
            style="
              font-size: 25px;
              font-weight: 600;
              margin-right: 15px;
              margin-bottom: 8px;
              line-height: 115%;
              word-break: normal;
            "
            :title="app.title"
          >
            {{ app.title }}
          </div>
          <div style="margin-top: -5px">
            [VERSION NOT IMPLEMENTED]
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
        {{ app.subtitle }}
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
        style="border-radius: 12px; margin: 12px;"
        @click="$emit('installApp')"
        >Install</HCButton
      >
    </div>
  </div>
</template>

<script lang="ts">
import { HolochainVersion } from "../types";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent, PropType } from "vue";

import HCButton from "./subcomponents/HCButton.vue";
import HCMoreToggle from "./subcomponents/HCMoreToggle.vue";
import { AppEntry } from "../appstore/types";

export default defineComponent({
  name: "AppPreviewCard",
  components: { HCButton, HCMoreToggle },
  props: {
    appIcon: {
      type: Uint8Array,
    },
    app: {
      type: Object as PropType<AppEntry>,
      required: true,
    },
  },
  data(): {
    showDescription: boolean;
    holochainVersion: HolochainVersion | undefined;
    guiVersion: string | undefined;
  } {
    return {
      showDescription: false,
      holochainVersion: undefined,
      guiVersion: undefined,
    };
  },
  emits: ["installApp"],
  methods: {
    appIconSrc(): string | undefined {
      return this.appIcon ? URL.createObjectURL(new Blob([this.appIcon], { type: 'image/png' })) : undefined
    }
  }
});
</script>

<style scoped>
.card {
  width: 370px;
  height: 240px;
  background: white;
  border-radius: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
}

/* .install-btn {

} */
.install-btn:hover {
  background-color: #674df9;
}
</style>
