<template>
  <div class="column" style="flex: 1">
    <InstallApp style="margin: 16px"></InstallApp>
    <ActiveApps
      @open-app="openApp($event)"
      style="flex: 1; padding: 24px"
    ></ActiveApps>
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
        this.$store.commit("log", `Opening app ${appId}...`);

        const result = await invoke("open_app_ui", { appId });
        this.$store.commit("log", `App ${appId} opened`);
      } catch (e) {
        this.$store.commit(
          "log",
          `Error opening app ${appId}: ${JSON.stringify(e)}`
        );
      }
    },
  },
});
</script>
