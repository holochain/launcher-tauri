<template>
  <div style="display: flex; flex: 1; flex-direction: column">
    <span style="margin-bottom: 16px; font-size: 1.5em">Installed Apps</span>
    <div
      v-for="version in $store.getters[`holochainVersions`]"
      :key="version"
      style="
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
      "
    >
      <InstalledAppsList
        :installedWebApps="$store.getters[`appsForVersion`](version)"
        @open-app="openApp(version, $event)"
        @disable-app="disableApp(version, $event)"
        @enable-app="enableApp(version, $event)"
        @starte-app="startApp(version, $event)"
        @uninstall-app="uninstallApp(version, $event)"
      />
    </div>
  </div>
  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ActionTypes } from "../store/actions";
import { HolochainVersion } from "../types";
import "@material/mwc-snackbar";
import { invoke } from "@tauri-apps/api/tauri";
import InstalledAppsList from "../components/InstalledAppsList.vue";

export default defineComponent({
  name: "InstalledApps",
  components: { InstalledAppsList },
  data(): {
    snackbarText: string | undefined;
  } {
    return { snackbarText: undefined };
  },

  methods: {
    async openApp(holochainVersion: HolochainVersion, appId: string) {
      try {
        await invoke("open_app_ui", { appId, holochainVersion });
        this.showMessage(`App ${appId} opened`);
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async disableApp(holochainVersion: HolochainVersion, appId: string) {
      try {
        await invoke("disable_app", { appId, holochainVersion });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.showMessage(`Disabled ${appId}`);
      } catch (e) {
        const error = `Disable app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async enableApp(holochainVersion: HolochainVersion, appId: string) {
      try {
        await invoke("enable_app", { appId, holochainVersion });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.showMessage(`Enabled ${appId}`);
      } catch (e) {
        const error = `Enable app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async startApp(holochainVersion: HolochainVersion, appId: string) {
      try {
        await invoke("start_app", { appId, holochainVersion });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Started ${appId}`);
      } catch (e) {
        const error = `Start app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async uninstallApp(holochainVersion: HolochainVersion, appId: string) {
      try {
        await invoke("uninstall_app", { appId, holochainVersion });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Uninstalled ${appId}`);
      } catch (e) {
        const error = `Uninstall app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    showMessage(message: string) {
      this.snackbarText = message;
      (this.$refs as any).snackbar.show();
    },
  },
});
</script>
<!-- We don't have scoped styles with classes because it becomes harder to export a reusable library -->
