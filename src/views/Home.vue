<template>
  <div class="column" style="flex: 1">
    <InstallApp style="margin: 16px"></InstallApp>
    <InstalledApps
      @open-app="openApp($event)"
      @app-deactivated="deactiveAppUI($event)"
      @app-activated="activeAppUI($event)"
      style="flex: 1; padding: 24px"
    ></InstalledApps>
    <Logs style="height: 220px"></Logs>
  </div>
</template>

<script lang="ts">
import InstallApp from "@/components/InstallApp.vue";
import Logs from "@/components/Logs.vue";
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "Home",
  components: {
    InstallApp,
    Logs,
  },
  methods: {
    async openApp(appId: string) {
      try {
        this.$store.commit("log", { log: `Opening app ${appId}...` });

        await invoke("open_app_ui", { appId });
        this.$store.commit("log", { log: `App ${appId} opened` });
      } catch (e) {
        this.$store.commit("log", {
          log: `Error opening app ${appId}: ${JSON.stringify(e)}`,
        });
      }
    },
    async deactiveAppUI(appId: string) {
      try {
        await invoke("deactivate_app_ui", { appId });
        this.$store.commit("log", { log: `App ${appId} deactivated` });
      } catch (e) {
        this.$store.commit("log", {
          log: `Deactivated app ${appId} failed: ${JSON.stringify(e)}`,
        });
      }
    },
    async activeAppUI(appId: string) {
      try {
        await invoke("activate_app_ui", { appId });
        this.$store.commit("log", { log: `App ${appId} activated` });
      } catch (e) {
        this.$store.commit("log", {
          log: `Activated app ${appId} failed: ${JSON.stringify(e)}`,
        });
      }
    },
  },
});
</script>
