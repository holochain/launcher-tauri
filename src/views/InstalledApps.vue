<template>
  <div
    v-if="isLoading()"
    style="flex: 1; display: flex; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else>
    <div style="display: flex; flex: 1; flex-direction: column">
      <div class="row top-bar" style="position: sticky; top: 0; z-index: 1">
        <img
          src="/img/Square284x284Logo.png"
          style="height: 42px; margin-left: 11px"
        />
        <span style="font-size: 1.5em; margin-left: 13px">{{
          $t("main.installedApps")
        }}</span>
        <span style="display: flex; flex: 1"></span>
        <HCButton
          style="
            margin-left: 8px;
            margin-right: 12px;
            height: 40px;
            border-radius: 8px;
            padding: 0 20px;
          "
          :title="reportIssueUrl"
          @click="reportIssue()"
        >
          <div class="row center-content">
            <span style="margin-left: 5px">{{ $t("main.reportIssue") }}</span>
          </div>
        </HCButton>
      </div>

      <div
        class="column"
        style="flex: 1; align-items: center; padding: 0 50px; margin-top: 20px"
      >
        <InstalledAppsList
          :installedApps="$store.getters[`allApps`]"
          @open-app="openApp($event)"
          @disable-app="disableApp($event)"
          @enable-app="enableApp($event)"
          @start-app="startApp($event)"
          @uninstall-app="uninstallApp($event)"
        />
      </div>
      <HCButton
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
      </HCButton>
    </div>
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

export default defineComponent({
  name: "InstalledApps",
  components: { InstalledAppsList, HCButton, HCSnackbar },
  data(): {
    snackbarText: string | undefined;
    reportIssueUrl: string;
  } {
    return {
      snackbarText: undefined,
      reportIssueUrl: "https://github.com/holochain/launcher/issues/new",
    };
  },
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);
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
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
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
      const appId = app.webAppInfo.installed_app_info.installed_app_id;

      try {
        await invoke("start_app", { appId, holochainId: app.holochainId });

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
    async reportIssue() {
      await invoke("open_url_cmd", {
        url: this.reportIssueUrl,
      });
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

.top-bar {
  align-items: center;
  height: 64px;
  background: #e8e8eb;
  background: white;
  box-shadow: 0 0px 5px #9b9b9b;
}
</style>
