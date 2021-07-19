<template>
  <div class="column" style="flex: 1">
    <InstallApp style="margin: 16px"></InstallApp>
    <InstalledApps
      @open-app="openApp($event)"
      @app-disabled="disableAppUI($event)"
      @app-enabled="enableAppUI($event)"
      @app-started="startAppUI($event)"
      style="flex: 1; padding: 24px"
    ></InstalledApps>
  </div>
</template>

<script lang="ts">
import InstallApp from "@/components/InstallApp.vue";
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "Home",
  components: {
    InstallApp,
  },
  methods: {
    async openApp(appId: string) {
      try {
        await invoke("log", { log: "Installed app" });

        await invoke("open_app_ui", { appId });
        this.$snackbar.add({
          type: "success",
          text: `App ${appId} opened`,
        });
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.$snackbar.add({
          type: "error",
          text: error,
        });
        await invoke("log", {
          log: error,
        });
      }
    },
    async disableAppUI(appId: string) {
      try {
        await invoke("deactivate_app_ui", { appId });
        this.$snackbar.add({
          type: "success",
          text: `App ${appId} disabled`,
        });
      } catch (e) {
        const error = `Disable app ${appId} failed: ${JSON.stringify(e)}`;
        this.$snackbar.add({
          type: "error",
          text: error,
        });
        await invoke("log", {
          log: error,
        });
      }
    },
    async enableAppUI(appId: string) {
      try {
        await invoke("activate_app_ui", { appId });
        this.$snackbar.add({
          type: "success",
          text: `App ${appId} enabled`,
        });
      } catch (e) {
        const error = `Enable app ${appId} failed: ${JSON.stringify(e)}`;
        this.$snackbar.add({
          type: "error",
          text: error,
        });
        await invoke("log", {
          log: error,
        });
      }
    },
    async startAppUI(appId: string) {
      try {
        await invoke("activate_app_ui", { appId });
        this.$snackbar.add({
          type: "success",
          text: `App ${appId} started`,
        });
      } catch (e) {
        const error = `Start app ${appId} failed: ${JSON.stringify(e)}`;
        this.$snackbar.add({
          type: "error",
          text: error,
        });
        await invoke("log", {
          log: error,
        });
      }
    },
  },
});
</script>
