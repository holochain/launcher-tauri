<template>
  <span v-if="$store.state.localAppsPorts.loading">Loading...</span>
  <div v-else class="column">
    <InstallApp style="margin: 16px"></InstallApp>
    <ActiveApps :appUrls="getAppUrls()"></ActiveApps>
  </div>
</template>

<script lang="ts">
import InstallApp from "@/components/InstallApp.vue";
import { defineComponent } from "vue";

export default defineComponent({
  name: "Home",
  components: {
    InstallApp,
  },
  data(): { appUrls: { [key: string]: number } } {
    return {
      appUrls: {},
    };
  },
  mounted() {
    console.log("hi");

    this.$store.dispatch("fetchLocalAppsPorts");
  },
  methods: {
    getAppUrls() {
      const portMapping = this.$store.state.localAppsPorts.portMapping;

      const appUrls: { [key: string]: string } = {};

      for (const [appId, port] of Object.entries(portMapping)) {
        appUrls[appId] = `http://localhost:${port}`;
      }
      console.log(appUrls);
      return appUrls;
    },
  },
});
</script>
