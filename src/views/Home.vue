<template>
  <div class="column" style="flex: 1">
    <div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <InstalledApps
            @open-app="openApp($event)"
            @disable-app="disableApp($event)"
            @enable-app="enableApp($event)"
            @starte-app="startApp($event)"
            @uninstall-app="uninstallApp($event)"
            style="padding: 24px; display: flex; margin-bottom: 50px"
          ></InstalledApps>
        </div>
      </div>
    </div>
    <InstallApp></InstallApp>
    <mwc-snackbar
      leading
      :labelText="snackbarText"
      ref="snackbar"
    ></mwc-snackbar>
  </div>
</template>

<script lang="ts">
import InstallApp from "@/components/InstallApp.vue";
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import AdminUI from "@holochain/admin-ui";

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
        await invoke("disable_app", { appId });

        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.fetchInstalledApps}`
        );
        this.showMessage(`Disabled ${appId}`);
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

        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.fetchInstalledApps}`
        );
        this.showMessage(`Enabled ${appId}`);
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

        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.fetchInstalledApps}`
        );

        this.showMessage(`Started ${appId}`);
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
