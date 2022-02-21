<template>
  <mwc-dialog
    :heading="'Install Plugin'"
    ref="dialog"
    scrimClickAction=""
    escapeKeyAction=""
  >
    <div class="column">
      <span style="margin-top: 8px">
        If you want to install a plugin which is compatible with Holochain
        Launcher, input the link of the binary and click download button.
        <b>
          Once the plugin is downloaded, you can restart the launcher to run it.
        </b>
      </span>
    </div>

    <mwc-textfield
      label="Plugin Download Link"
      outlined
      @input="pluginLink = $event.target.value"
      class="row-item"
      style="flex: 1"
    ></mwc-textfield>

    <mwc-button
      slot="secondaryAction"
      :disabled="isInstalling"
      dialogAction="close"
      label="Cancel"
    ></mwc-button>
    <mwc-button
      slot="primaryAction"
      @click="installPlugin()"
      :disabled="isInstalling"
      :label="isInstalling ? 'Installing...' : 'Install Plugin'"
    ></mwc-button>
  </mwc-dialog>

  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { getCurrent } from "@tauri-apps/api/window";
import type { Dialog } from "@material/mwc-dialog";

export default defineComponent({
  name: "PluginInstall",
  data(): {
    snackbarText: string | undefined;
    isInstalling: boolean;
    pluginLink: string | undefined;
  } {
    return {
      snackbarText: undefined,
      isInstalling: false,
      pluginLink: undefined,
    };
  },
  async mounted() {
    this.$nextTick(async () => {
      await getCurrent().listen("request-plugin-install", () =>
        this.showDialog()
      );
    });
  },
  methods: {
    showDialog() {
      (this.$refs.dialog as Dialog).show();
    },
    async installPlugin() {
      try {
        this.isInstalling = true;
        await invoke("execute_plugin_install", { url: this.pluginLink });
        this.isInstalling = false;
        this.showMessage(`Installed ${this.pluginLink}`);
        window.location.reload();
      } catch (e) {
        this.isInstalling = false;
        const error = `Error install the plugin: ${JSON.stringify(e)}`;
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
