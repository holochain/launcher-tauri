<template>
  <div
    v-if="isLoading()"
    class="column center-content"
    style="flex: 1; height: calc(100vh - 64px)"
  >
    <LoadingDots
      style="--radius: 15px; --dim-color: #e8e8eb; --fill-color: #b5b5b5"
    ></LoadingDots>
  </div>
  <div
    v-else
    class="column"
    style="flex: 1; min-height: calc(100vh - 64px); align-items: center"
  >
    <InstalledAppsList
      @openApp="openApp($event)"
      @select-view="$emit('select-view', $event)"
    />
  </div>

  <HCSnackbar leading :labelText="snackbarText" ref="snackbar"></HCSnackbar>
</template>

<script lang="ts">
import "@material/mwc-icon";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";

import { HolochainAppInfo } from "../types";
import InstalledAppsList from "../components/InstalledAppsList.vue";
import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import LoadingDots from "../components/subcomponents/LoadingDots.vue";

export default defineComponent({
  name: "Launcher",
  components: {
    InstalledAppsList,
    HCSnackbar,
    LoadingDots,
  },
  emits: ["show-message", "select-view"],
  data(): {
    snackbarText: string | undefined;
  } {
    return {
      snackbarText: undefined,
    };
  },
  methods: {
    isLoading() {
      return this.$store.state.launcherStateInfo === "loading";
    },
    async openApp(app: HolochainAppInfo) {
      const appId = app.webAppInfo.installed_app_info.installed_app_id;
      try {
        await invoke("open_app_ui", { appId, holochainId: app.holochainId });
        this.showMessage(`App ${appId} opened`);
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    showMessage(message: string) {
      this.$emit("show-message", message);
    },
  },
});
</script>
<!-- We don't have scoped styles with classes because it becomes harder to export a reusable library -->

<style scoped>
.btn-install:hover {
  cursor: pointer;
  --hc-primary-color: #5537fc;
}
.btn-install:focus-visible {
  --hc-primary-color: #5537fc;
}
</style>
