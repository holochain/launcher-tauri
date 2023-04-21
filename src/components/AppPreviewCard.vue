<template>
  <div class="column card">
    <div
      v-if="showDescription"
      class="column"
      style="flex: 1; overflow-y: auto; padding: 20px"
    >
      <div style="font-weight: bold; margin-top: 10px">Description:</div>
      {{ app.description }}
    </div>

    <div v-else class="column" style="flex: 1">
      <div class="row" style="align-items: center">
        <!-- if icon provided -->
        <img
          v-if="imgSrc"
          :src="imgSrc"
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
import { collectBytes } from "../appstore/appstore-interface";
import { AppWebsocket } from "@holochain/client";
import { APP_STORE_ID } from "../constants";

export default defineComponent({
  name: "AppPreviewCard",
  components: { HCButton, HCMoreToggle },
  props: {
    app: {
      type: Object as PropType<AppEntry>,
      required: true,
    },
    appWebsocket: {
      type: Object as PropType<any>,
      required: true,
    },
  },
  data(): {
    showDescription: boolean;
    holochainVersion: HolochainVersion | undefined;
    guiVersion: string | undefined;
    imgSrc: string | undefined;
  } {
    return {
      showDescription: false,
      holochainVersion: undefined,
      guiVersion: undefined,
      imgSrc: undefined,
    };
  },
  emits: ["installApp"],
  async mounted () {
    console.log("Preview card is mounted...");
    const iconHash = this.app.icon;
    console.log("@mounted: Getting mere_memory data for hash: ", iconHash);
    const appStoreInfo = await this.appWebsocket!.appInfo({
      installed_app_id: APP_STORE_ID,
    });

    const collectedBytes = await collectBytes(this.appWebsocket, appStoreInfo, iconHash);
    this.imgSrc = URL.createObjectURL(new Blob([collectedBytes],  { type: "image/png" }));
  },
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
