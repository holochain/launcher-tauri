<template>
  <div class="column" style="flex: 1">
    <div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <span style="font-size: 20px">Installed Apps</span>
          <InstalledApps
            style="padding: 24px; display: flex; margin-bottom: 50px"
          ></InstalledApps>
        </div>
      </div>
    </div>
    <InstallApp></InstallApp>
    <mwc-dialog
      heading="Uninstall App"
      :open="appToBeUninstalled"
      @closing="appToBeUninstalled = undefined"
    >
      <div>Are you sure you want to uninstall {{ appToBeUninstalled }}?</div>

      <mwc-button
        label="Cancel"
        slot="secondaryAction"
        dialogAction="close"
      ></mwc-button>
      <mwc-button
        label="Uninstall"
        slot="primaryAction"
        @click="uninstallApp(appToBeUninstalled)"
      ></mwc-button>
    </mwc-dialog>
  </div>
</template>

<script lang="ts">
import InstallApp from "@/components/InstallApp.vue";
import InstalledApps from "./InstalledApps.vue";
import { defineComponent } from "vue";

export default defineComponent({
  name: "Home",
  components: {
    InstallApp,
    InstalledApps,
  },
  data(): {
    snackbarText: string | undefined;
    appToBeUninstalled: string | undefined;
  } {
    return { snackbarText: undefined, appToBeUninstalled: undefined };
  },
  methods: {},
});
</script>
