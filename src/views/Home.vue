<template>
  <div class="column">
    <InstallApp style="margin: 16px"></InstallApp>
    <ActiveApps></ActiveApps>
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
});
</script>
