<template>
  <mwc-dialog heading="Install App" open scrimClickAction="" escapeKeyAction="">
    <div v-if="!appBundlePath">
      <InstallableApps
        @selected-app-bundle="appBundlePath = $event"
      ></InstallableApps>

      <mwc-button
        label="Select app from FileSystem"
        @click="selectFromFileSystem()"
      >
      </mwc-button>
    </div>

    <div v-else>
      <SetupApp
        :appBundlePath="appBundlePath"
        @setup-changed="appSetup = $event"
      ></SetupApp>
    </div>

    <mwc-button
      label="Cancel"
      slot="secondaryAction"
      dialogAction="close"
      :disabled="installing"
    >
    </mwc-button>
    <mwc-button
      v-if="appBundlePath"
      slot="primaryAction"
      :disabled="!appSetup || installing"
      @click="installApp()"
      :label="installing ? 'Installing...' : 'Install app'"
    >
    </mwc-button>
  </mwc-dialog>
  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { AppSetup, HolochainVersion, WebAppInfo } from "../../types";
import { ActionTypes } from "@/store/actions";
import InstallableApps from "./InstallableApps.vue";
import SetupApp from "./SetupApp.vue";
import { open } from "@tauri-apps/api/dialog";
import "@material/mwc-dialog";

export default defineComponent({
  name: "InstallApp",
  components: {
    InstallableApps,
    SetupApp,
  },
  data(): {
    installing: boolean;
    appSetup: AppSetup | undefined;
    appBundlePath: string | undefined;
    snackbarText: string | undefined;
  } {
    return {
      installing: false,
      appBundlePath: undefined,
      appSetup: undefined,
      snackbarText: undefined,
    };
  },
  methods: {
    async selectFromFileSystem() {
      this.appBundlePath = (await open({
        filters: [
          { name: "Holochain Application", extensions: ["webhapp", "happ"] },
        ],
      })) as string;
    },
    async installApp() {
      if (!this.appSetup) return;

      try {
        this.installing = true;
        const holochainVersions: HolochainVersion[] =
          this.$store.getters["holochainVersions"];

        await invoke("install_app", {
          appId: this.appSetup.appId,
          appBundlePath: this.appBundlePath,
          membraneProofs: this.appSetup.membraneProofs,
          uid: this.appSetup.uid,
          holochainVersion: holochainVersions[0],
        });

        this.installing = false;
        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Installed ${this.appSetup.appId}`);

        this.$emit("app-installed", this.appSetup.appId);
      } catch (e) {
        this.installing = false;
        this.showMessage(JSON.stringify(e));
      }
    },
    showMessage(message: string) {
      this.snackbarText = message;
      (this.$refs as any).snackbar.show();
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.app-row {
  margin-bottom: 16px;
}

.app-title {
  font-size: 24px;
}

.cell-row {
  margin-top: 8px;
}
.center {
  align-items: center;
  justify-content: center;
}

.row-item {
  margin-top: 16px;
}
</style>
