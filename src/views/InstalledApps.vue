<template>
  <div
    v-if="isLoading()"
    style="flex: 1; display: flex; align-items: center; justify-content: center"
  >
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else>
    <div
      v-if="view.type === 'installedApps'"
      style="display: flex; flex: 1; flex-direction: column"
    >
      <span style="margin: 16px; font-size: 1.5em">Installed Apps</span>
      <div class="column" style="flex: 1; align-items: center">
        <div
          v-for="version of $store.getters[`holochainVersions`]"
          :key="version"
          class="column"
          style="flex: 1; width: 600px"
        >
          <span style="margin-bottom: 8px">Holochain v{{ version }}</span>

          <InstalledAppsList
            :installedWebApps="$store.getters[`appsForVersion`](version)"
            @open-app="openApp(version, $event)"
            @app-selected="selectApp(version, $event)"
          />
        </div>
      </div>
      <mwc-fab
        extended
        icon="add"
        label="Install new app"
        @click="$emit('open-app-store')"
        style="
          margin: 16px;
          position: absolute;
          right: 0;
          bottom: 0;
          --mdc-theme-secondary: #4720e3;
        "
      ></mwc-fab>
    </div>

    <div v-else style="flex: 1">
      <div class="row" style="justify-content: start; align-items: center">
        <mwc-icon-button
          icon="arrow_back"
          @click="view = { type: 'installedApps' }"
        ></mwc-icon-button>
        <span style="font-size: 1.5em">App Detail</span>
      </div>

      <div class="column center-content">
        <InstalledAppDetail
          style="width: 800px"
          :installedWebAppInfo="selectedAppInfo"
          @disable-app="disableSelectedApp()"
          @enable-app="enableSelectedApp()"
          @start-app="startSelectedApp()"
          @uninstall-app="uninstallSelectedApp()"
        ></InstalledAppDetail>
      </div>
    </div>
  </div>

  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ActionTypes } from "../store/actions";
import { HolochainVersion, InstalledWebAppInfo } from "../types";
import "@material/mwc-snackbar";
import { invoke } from "@tauri-apps/api/tauri";
import InstalledAppsList from "../components/InstalledAppsList.vue";
import InstalledAppDetail from "../components/InstalledAppDetail.vue";

type View =
  | {
      type: "installedApps";
    }
  | {
      type: "appDetail";
      holochainVersion: HolochainVersion;
      appId: string;
    };

export default defineComponent({
  name: "InstalledApps",
  components: { InstalledAppsList, InstalledAppDetail },
  data(): {
    snackbarText: string | undefined;
    view: View;
  } {
    return { snackbarText: undefined, view: { type: "installedApps" } };
  },
  computed: {
    selectedAppInfo() {
      const view = this.view as View;
      if (view.type !== "appDetail") return undefined;
      if (!this.$store.getters[`appsForVersion`]) return undefined;

      const apps: InstalledWebAppInfo[] = this.$store.getters[`appsForVersion`](
        view.holochainVersion
      );

      return apps.find(
        (app) => app.installed_app_info.installed_app_id === view.appId
      );
    },
  },
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);
  },
  methods: {
    isLoading() {
      return this.$store.state.launcherStateInfo === "loading";
    },
    selectApp(version: HolochainVersion, appId: string) {
      this.view = {
        type: "appDetail",
        holochainVersion: version,
        appId,
      };
    },
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
    async disableSelectedApp() {
      if (this.view.type !== "appDetail") return;
      const { appId, holochainVersion } = this.view;

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
    async enableSelectedApp() {
      if (this.view.type !== "appDetail") return;
      const { appId, holochainVersion } = this.view;

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
    async startSelectedApp() {
      if (this.view.type !== "appDetail") return;
      const { appId, holochainVersion } = this.view;

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
    async uninstallSelectedApp() {
      if (this.view.type !== "appDetail") return;
      const { appId, holochainVersion } = this.view;

      try {
        await invoke("uninstall_app", { appId, holochainVersion });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Uninstalled ${appId}`);
        this.view = { type: "installedApps" };
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
