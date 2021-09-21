<template>
  <mwc-fab
    extended
    icon="add"
    label="Install new app"
    @click="selectWebHappFile()"
    style="
      margin: 16px;
      position: absolute;
      right: 0;
      bottom: 0;
      --mdc-theme-secondary: #4720e3;
    "
  ></mwc-fab>
  <mwc-dialog heading="Install App" :open="!!webAppBundlePath">
    <div
      class="row center"
      style="width: 512px"
      v-if="webAppBundlePath === 'loading'"
    >
      <mwc-circular-progress
        indeterminate
        style="margin-top: 80px; margin-bottom: 60px"
      ></mwc-circular-progress>
    </div>
    <div class="column" style="width: 512px" v-else-if="webAppBundlePath">
      <span style="margin-right: 8px"
        >Bundle: {{ pathToFilename(webAppBundlePath) }}</span
      >

      <div class="column" style="flex: 1">
        <mwc-textfield
          label="App Id"
          outlined
          @change="appId = $event.target.value"
          class="row-item"
          style="flex: 1"
        ></mwc-textfield>
        <mwc-textfield
          label="UID"
          outlined
          @change="uid = $event.target.value"
          class="row-item"
          style="flex: 1"
        ></mwc-textfield>

        <span class="row-item">Slots</span>
        <div
          v-for="appSlot of slots"
          :key="appSlot.name"
          class="row-item row"
          style="flex: 1"
        >
          <span>{{ appSlot.name }}</span>
          <mwc-textarea
            style="flex: 1"
            label="Membrane Proof"
            outlined
            @change="membraneProofs[appSlot.name] = $event.target.value"
          ></mwc-textarea>
        </div>
      </div>
    </div>

    <mwc-button
      v-if="webAppBundlePath !== 'loading'"
      label="Cancel"
      slot="secondaryAction"
      :disabled="installing"
      @click="cancel()"
    >
    </mwc-button>
    <mwc-button
      v-if="webAppBundlePath !== 'loading'"
      slot="primaryAction"
      :disabled="!isAppReadyToInstall() || installing"
      @click="installApp()"
      :label="installing ? 'Installing...' : 'Install app'"
    >
    </mwc-button>
  </mwc-dialog>
  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
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
    webAppBundlePath: string | "loading" | undefined;
    appId: string | undefined;
    uid: string | undefined;
    membraneProofs: { [key: string]: string } | undefined;
    slots: Array<any> | undefined;
    snackbarText: string | undefined;
  } {
    return {
      installing: false,
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

      if (!webAppBundlePath) return;

      this.webAppBundlePath = "loading";

      this.membraneProofs = {};
      this.slots = await invoke("get_slots_to_configure", { webAppBundlePath });

      this.webAppBundlePath = webAppBundlePath;
    },
    cancel() {
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

        this.showMessage(`Installed ${this.appId}`);
        this.webAppBundlePath = undefined;
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
