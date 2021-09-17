<template>
  <mwc-fab
    extended
    icon="add"
    label="Install new app"
    @click="showDialog = true"
  ></mwc-fab>
  <mwc-dialog heading="Install App" :open="showDialog">
    <div class="column">
      <div class="row">
        <mwc-button
          v-if="!webAppBundlePath"
          style="margin-right: 8px"
          @click="selectWebHappFile()"
          raised
        >
          Select Web-hApp bundle
        </mwc-button>
        <span v-else style="margin-right: 8px">{{
          pathToFilename(webAppBundlePath)
        }}</span>
      </div>

      <div v-if="webAppBundlePath">
        <mwc-textfield
          label="App Id"
          outlined
          @change="appId = $event.target.value"
          class="row-item"
        ></mwc-textfield>
        <mwc-textfield
          label="UID"
          outlined
          @change="uid = $event.target.value"
          class="row-item"
        ></mwc-textfield>

        <span class="row-item">Slots</span>
        <div v-for="appSlot of slots" :key="appSlot.name" class="row-item">
          <span>{{ appSlot.name }}</span>
          <mwc-textfield
            label="Membrane Proof"
            outlined
            @change="membraneProofs[appSlot.name] = $event.target.value"
          ></mwc-textfield>
        </div>
      </div>
    </div>

    <mwc-button label="Cancel" slot="secondaryAction" @click="cancel()">
    </mwc-button>
    <mwc-button
      v-if="!installing"
      :disabled="!isAppReadyToInstall()"
      @click="installApp()"
      style="margin-left: 24px"
      label="INSTALL APP"
      raised
      slot="primaryAction"
    >
    </mwc-button>
    <mwc-circular-progress v-else style="width: 80px"></mwc-circular-progress>
  </mwc-dialog>
  <mwc-snackbar :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import AdminUI from "@holochain/admin-ui";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

export default defineComponent({
  name: "InstallApp",
  data(): {
    installing: boolean;
    showDialog: boolean;
    webAppBundlePath: string | undefined;
    appId: string | undefined;
    uid: string | undefined;
    membraneProofs: { [key: string]: string } | undefined;
    slots: Array<any> | undefined;
    snackbarText: string | undefined;
  } {
    return {
      installing: false,
      showDialog: false,
      appId: undefined,
      webAppBundlePath: undefined,
      uid: undefined,
      membraneProofs: undefined,
      slots: undefined,
      snackbarText: undefined,
    };
  },
  methods: {
    async selectWebHappFile() {
      const webAppBundlePath = (await open({
        filters: [{ name: "webhapp", extensions: ["webhapp"] }],
      })) as string;

      this.membraneProofs = {};
      this.slots = await invoke("get_slots_to_configure", { webAppBundlePath });

      this.webAppBundlePath = webAppBundlePath;
    },
    cancel() {
      this.showDialog = false;
      this.webAppBundlePath = undefined;
      this.uid = undefined;
      this.membraneProofs = {};
      this.slots = undefined;
    },
    pathToFilename(path: string) {
      const components = path.split("/");
      return components[components.length - 1];
    },
    isAppReadyToInstall() {
      if (!this.appId) return false;
      if (!this.webAppBundlePath) return false;
      return true;
    },
    async installApp() {
      try {
        this.installing = true;
        await invoke("install_app", {
          appId: this.appId,
          webAppBundlePath: this.webAppBundlePath,
          membraneProofs: this.membraneProofs,
          uid: this.uid,
        });

        this.installing = false;
        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.fetchInstalledApps}`
        );

        this.showMessage(`Installed hApp ${this.appId}`);
        this.showDialog = false;
        this.webAppBundlePath = undefined;
      } catch (e) {
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
