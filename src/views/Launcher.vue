<template>

  <!-- <div
    v-if="isLoading()"
    class="column center-content" style="flex: 1; height: calc(100vh - 64px);"
  >
    <LoadingDots style="--radius: 15px; --dim-color: #e8e8eb; --fill-color: #b5b5b5"></LoadingDots>
  </div> -->

  <div
    class="column"
    style="flex: 1; align-items: center; padding: 0 50px; margin-top: 20px"
  >
    <InstalledAppsList
      :installedApps="$store.getters[`allApps`]"
      @openApp="openApp($event)"
      @disableApp="disableApp($event)"
      @enableApp="enableApp($event)"
      @startApp="startApp($event)"
      @uninstallApp="uninstallApp($event)"
    />
  </div>

  <HCSnackbar leading :labelText="snackbarText" ref="snackbar"></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ActionTypes } from "../store/actions";
import { HolochainAppInfo, HolochainId, InstalledWebAppInfo } from "../types";
import "@material/mwc-icon";
import { invoke } from "@tauri-apps/api/tauri";
import InstalledAppsList from "../components/InstalledAppsList.vue";
import HCButton from "../components/subcomponents/HCButton.vue";
import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import HCDialog from "../components/subcomponents/HCDialog.vue";
import ToggleSwitch from "../components/subcomponents/ToggleSwitch.vue";
import LoadingDots from "../components/subcomponents/LoadingDots.vue";

export default defineComponent({
  name: "Launcher",
  components: { InstalledAppsList, HCButton, HCSnackbar, HCDialog, ToggleSwitch, LoadingDots },
  data(): {
    snackbarText: string | undefined;
  } {
    return {
      snackbarText: undefined

    };
  },
  methods: {
    isLoading() {
      return this.$store.state.launcherStateInfo === "loading";
    },
    async openApp(app: HolochainAppInfo) {

      const appId = app.webAppInfo.installed_app_info.installed_app_id;
      try {
        await invoke("open_app_ui", { appId, holochainId: app.holochainId });
        this.showMessage(`App ${appId} opened`);
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async disableApp(app: HolochainAppInfo) {
      const appId = app.webAppInfo.installed_app_info.installed_app_id;
      try {
        await invoke("disable_app", { appId, holochainId: app.holochainId });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.showMessage(`Disabled ${appId}`);
      } catch (e) {
        const error = `Disable app ${appId} failed: ${JSON.stringify(e)}`;

        // if disabling "purportedly" fails due to being offline, ignore the error.
        if (error.includes("failed to lookup address information: Temporary failure in name resolution")) {
          this.showMessage(`Disabled ${appId}`);
        } else {
          this.showMessage(error);
        }
        await invoke("log", {
          log: error,
        });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      }
    },
    async enableApp(app: HolochainAppInfo) {
      const appId = app.webAppInfo.installed_app_info.installed_app_id;

      try {
        await invoke("enable_app", { appId, holochainId: app.holochainId });

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
    async startApp(app: HolochainAppInfo) {
      // console.log("@InstalledApps: RECEIVED REQUEST TO START APP.");
      const appId = app.webAppInfo.installed_app_info.installed_app_id;
      // console.log("@InstalledApps: @startApp: appId: ", appId);

      // StartApp is not available anymore in conductor API since 0.1.0-beta-rc.4: https://github.com/holochain/holochain/blob/develop/crates/holochain_conductor_api/CHANGELOG.md#010-beta-rc4
      // instead disable app followed by enable app:
      try {
        // console.log("@InstalledApps: @startApp: disabling app.");

        await invoke("disable_app", { appId, holochainId: app.holochainId });
        // console.log("@InstalledApps: @startApp: app disabled, enabling app.");

        await invoke("enable_app", { appId, holochainId: app.holochainId });
        // console.log("@InstalledApps: @startApp: app enabled.");

        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Started ${appId}`);
      } catch (e) {
        const error = `Start app ${appId} failed: ${JSON.stringify(e)}`;
        console.error(error);
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      }
    },
    // TODO: remove
    async uninstallApp(app: HolochainAppInfo) {
      const appId = app.webAppInfo.installed_app_info.installed_app_id;

      try {
        await invoke("uninstall_app", { appId, holochainId: app.holochainId });

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

<style scoped>
.btn-install:hover {
  cursor: pointer;
  --hc-primary-color: #5537fc;
}
.btn-install:focus-visible {
  --hc-primary-color: #5537fc;
}
</style>
