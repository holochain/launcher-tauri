<template>
  <div class="column" style="flex: 1">
    <InstallApp style="margin: 16px"></InstallApp>
    <ActiveApps
      @launch-app="launchApp($event)"
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
    async launchApp(appId: string) {
      try {
        this.$store.commit("log", `Opening app ${appId}...`);

        const result = await invoke("launch_app_ui", { appId });
        this.$store.commit("log", `App ${appId} opened, ${result}`);
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
