<template>
  <mwc-dialog open heading="Launch Error">
    <div class="column">
      <span>There was an error launching Holochain.</span>
      <span style="margin-top: 8px">
        If you are upgrading from a previous version of Holochain, it is most
        likely that the new version is not compatible with the data that this
        computer has stored.
      </span>
      <span style="margin-top: 8px">
        In the future, Holochain will include a smooth mechanism to upgrade from
        one version to the next. Unfortunately, at the moment there is no way of
        upgrade old Holochain apps to be compatible with newer versions of
        Holochain.
      </span>
      <span style="margin-top: 8px">
        <b>If you don't want to lose old data,</b> uninstall this version of the
        Holochain Launcher and downgrade with the version that you were already
        using.
      </span>
      <span style="margin-top: 8px">
        <b>If you don't mind losing old data</b>, you can execute a factory
        reset.
        <b>
          This will uninstall all the Holochain apps that were installed in this
          computer, and also remove all previous stored data.
        </b>
      </span>
    </div>

    <mwc-button
      slot="primaryAction"
      @click="executeFactoryReset"
      label="Execute Factory Reset"
    ></mwc-button>
  </mwc-dialog>
  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "LaunchError",
  data(): {
    snackbarText: string | undefined;
  } {
    return {
      snackbarText: undefined,
    };
  },
  methods: {
    async executeFactoryReset() {
      try {
        await invoke("factory_reset");
        window.location.reload();
      } catch (e) {
        const error = `Error executing the factory reset: ${JSON.stringify(e)}`;
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
