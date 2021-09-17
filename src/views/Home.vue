<template>
  <div class="column" style="flex: 1">
    <InstalledApps
      @open-app="openApp($event)"
      @disable-app="disableApp($event)"
      @enable-app="enableApp($event)"
      @starte-app="startApp($event)"
      @uninstall-app="uninstallApp($event)"
      style="flex: 1; padding: 24px"
    ></InstalledApps>
    <InstallApp
      style="margin: 16px; position: absolute; right: 0; bottom: 0"
    ></InstallApp>
    <mwc-snackbar :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
  </div>
</template>

<script lang="ts">
import InstallApp from "@/components/InstallApp.vue";
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import AdminUI from "@holochain/admin-ui";
import "@material/mwc-snackbar";
import "@material/mwc-button";

export default defineComponent({
  name: "Home",
  components: {
    InstallApp,
  },
  data(): {
    snackbarText: string | undefined;
  } {
    return { snackbarText: undefined };
  },

  methods: {
    async openApp(appId: string) {
      try {
        await invoke("log", { log: "Installed app" });

        await invoke("open_app_ui", { appId });
        this.showMessage(`App ${appId} opened`);
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async disableApp(appId: string) {
      try {
        await invoke("deactivate_app", { appId });
        this.showMessage(`App ${appId} disabled`);
      } catch (e) {
        const error = `Disable app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async enableApp(appId: string) {
      try {
        await invoke("enable_app", { appId });
        this.showMessage(`App ${appId} enabled`);
      } catch (e) {
        const error = `Enable app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async startApp(appId: string) {
      try {
        await invoke("start_app", { appId });
        this.showMessage(`App ${appId} started`);
      } catch (e) {
        const error = `Start app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async uninstallApp(appId: string) {
      try {
        await invoke("uninstall_app", { appId });

        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.fetchInstalledApps}`
        );

        this.showMessage(`App ${appId} uninstalled`);
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
