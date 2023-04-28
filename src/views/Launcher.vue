<template>
  <HCDialog ref="devHubDevsOnlyWarning">
    <div
      class="column"
      style="padding: 30px; align-items: center; max-width: 500px"
    >
      <div style="font-weight: 600; font-size: 27px; margin-bottom: 25px">
        DevHub
      </div>
      <div>
        DevHub is the place where <span style="font-weight: bold; white-space: nowrap;">app developers</span> can upload their apps such that they appear in the App Library.<br><br>
        If you instead want to install other apps, click on the <span style="font-weight: bold; white-space: nowrap;">"Install New App"</span> button in the bottom right corner of the
        main window. It will lead you to the <span style="font-weight: bold; white-space: nowrap;">App Library</span>.
      </div>

      <div class="row" style="margin-top: 30px; margin-bottom: 10px; margin-left: 50px; width: 100%;">
        <ToggleSwitch
          :sliderOn="ignoreDevHubWaring"
          @click="() => ignoreDevHubWaring = !ignoreDevHubWaring"
          @keydown.enter="() => ignoreDevHubWaring = !ignoreDevHubWaring"
        />
        <span style="margin-left: 10px;">Don't show this message again.</span>
      </div>

      <div class="row" style="margin-top: 20px;">
        <HCButton style="height: 30px; margin: 4px 6px;" outlined @click="closeDevHubNote">Cancel</HCButton>
        <HCButton style="margin: 4px 6px;" @click="handleOpenDevHub">Open DevHub</HCButton>
      </div>
    </div>
  </HCDialog>

  <div
    v-if="isLoading()"
    class="column center-content" style="flex: 1; height: calc(100vh - 64px);"
  >
    <LoadingDots style="--radius: 15px; --dim-color: #e8e8eb; --fill-color: #b5b5b5"></LoadingDots>
  </div>

  <div
    v-else
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

  <!-- <HCButton
    tabindex="0"
    @click="$emit('open-app-store')"
    class="btn-install"
    style="
      font-family: Poppins;
      margin: 16px;
      height: 54px;
      position: absolute;
      right: 0;
      bottom: 0;
    "
    ><div class="row center-content" style="font-size: 18px">
      <mwc-icon style="margin-right: 10px; font-size: 26px">add</mwc-icon
      >{{ $t("main.installNewApp") }}
    </div>
  </HCButton> -->

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
  name: "InstalledApps",
  components: { InstalledAppsList, HCButton, HCSnackbar, HCDialog, ToggleSwitch, LoadingDots },
  data(): {
    snackbarText: string | undefined;
    showDevHubDevsOnlyWarning: boolean;
    devHubAppInfo: HolochainAppInfo | undefined;
    ignoreDevHubWaring: boolean;
    installingDevHub: boolean;
  } {
    return {
      snackbarText: undefined,
      showDevHubDevsOnlyWarning: false,
      devHubAppInfo: undefined,
      ignoreDevHubWaring: false,
      installingDevHub: false,
    };
  },
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);
  },
  methods: {
    isLoading() {
      return this.$store.state.launcherStateInfo === "loading";
    },
    closeDevHubNote() {
      if (this.ignoreDevHubWaring) {
        window.localStorage.setItem("ignoreDevHubDevsOnlyWarning", "true");
      }
      (this.$refs["devHubDevsOnlyWarning"] as typeof HCDialog).close();
    },
    async handleOpenDevHub() {
      if (this.ignoreDevHubWaring) {
        window.localStorage.setItem("ignoreDevHubDevsOnlyWarning", "true");
      }
      const appId = this.devHubAppInfo!.webAppInfo.installed_app_info.installed_app_id;
      (this.$refs["devHubDevsOnlyWarning"] as typeof HCDialog).close();
      try {
        await invoke("open_app_ui", { appId, holochainId: this.devHubAppInfo!.holochainId });
        this.showMessage(`App ${appId} opened`);
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async openApp(app: HolochainAppInfo) {
      // if the DevHub is requested to be opened, show a warning dialog that
      // this is intended for developers

      if ((app.webAppInfo.installed_app_info.installed_app_id == `DevHub-${app.holochainId.content}`)
       && (!window.localStorage.ignoreDevHubDevsOnlyWarning)) {
        this.devHubAppInfo = app;
        (this.$refs["devHubDevsOnlyWarning"] as typeof HCDialog).open();
        return;
      }

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
    async installDevHub() {
      this.installingDevHub = true;
      try {
        await invoke("install_devhub", {});
        this.installingDevHub = false;
        window.location.reload();
      } catch (e) {
        alert(`Failed to install DevHub: ${JSON.stringify(e)}`);
        console.error(`Failed to install DevHub: ${JSON.stringify(e)}`);
        this.installingDevHub = false;
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
