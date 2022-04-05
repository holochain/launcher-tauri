<template>
  <div class="column" style="flex: 1; margin: 24px">
    <div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <InstalledApps
            style="display: flex; margin-bottom: 50px"
          ></InstalledApps>
        </div>
      </div>
    </div>
    <InstallApp
      v-if="showInstallApp"
      @app-installed="showInstallApp = false"
    ></InstallApp>

    <mwc-fab
      extended
      icon="add"
      label="Install new app"
      @click="showInstallApp = true"
      style="
        margin: 16px;
        position: absolute;
        right: 0;
        bottom: 0;
        --mdc-theme-secondary: #4720e3;
      "
    ></mwc-fab>

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
import InstallApp from "@/components/install-app/InstallApp.vue";
import InstalledApps from "./InstalledApps.vue";
import { defineComponent } from "vue";
import "@material/mwc-fab";

export default defineComponent({
  name: "Home",
  components: {
    InstallApp,
    InstalledApps,
  },
  data(): {
    snackbarText: string | undefined;
    showInstallApp: boolean;
  } {
    return { snackbarText: undefined, showInstallApp: false };
  },
  methods: {},
});
</script>
