<template>
  <mwc-dialog
    :heading="$data.heading"
    ref="dialog"
    scrimClickAction=""
    escapeKeyAction=""
  >
    <div class="column">
      <div v-if="oldFiles" class="column">
        <span>
          It seems you have old files from older installations of the launcher.
          It's recommended that you do a factory reset and start afresh.
        </span>
        <span style="margin-top: 8px">
          This new version of the Launcher already comes with support for
          multiple versions, so from now on this will not be needed anymore when
          upgrading from one version of the Launcher to the next.
        </span>

        <span style="margin-top: 8px">
          <b
            >Notice! This will uninstall all the Holochain apps that were
            installed in this computer, and also remove all previous stored
            data.
          </b>
        </span>
      </div>

      <div v-else-if="dbFileTypeError" class="column">
        <span>
          It seems that the database of your conductor is not recognized
          properly.
        </span>
        <span style="margin-top: 8px">
          If you haven't changed the database files yourself, this is most
          probably because the Holochain Launcher switched to encrypting your
          database at rest as of version 0.5.0 in order to anticipate the same
          upcoming change of the official Holochain repository.
        </span>
        <span style="margin-top: 8px">
          Unfortunately, the easiest way to fix this and still use the latest
          version of the Holochain Launcher is to do a factory reset.
        </span>
        <span style="margin-top: 8px">
          <b>
            This will uninstall all the Holochain apps that were installed on
            this computer as well as remove all their previously stored data.
          </b>
        </span>
      </div>

      <span v-else style="margin-top: 8px">
        <b>If you don't mind losing old Holochain data</b>, you can execute a
        factory reset.
        <b>
          This will uninstall all the Holochain apps that were installed on this
          computer as well as remove all their previously stored data.
        </b>
      </span>
    </div>

    <mwc-button
      v-if="!oldFiles"
      slot="secondaryAction"
      :disabled="executing"
      dialogAction="close"
      label="Cancel"
    ></mwc-button>
    <mwc-button
      slot="primaryAction"
      @click="executeFactoryReset()"
      :disabled="executing"
      :label="executing ? 'Executing...' : 'Execute Factory Reset'"
    ></mwc-button>
  </mwc-dialog>
  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { getCurrent } from "@tauri-apps/api/window";
import type { Dialog } from "@material/mwc-dialog";
import { ActionTypes } from "@/store/actions";
import { listen } from "@tauri-apps/api/event";

export default defineComponent({
  name: "FactoryReset",
  data(): {
    snackbarText: string | undefined;
    executing: boolean;
    oldFiles: boolean;
    heading: string;
    dbFileTypeError: boolean;
  } {
    return {
      snackbarText: undefined,
      executing: false,
      oldFiles: false,
      heading: "Factory Reset",
      dbFileTypeError: false,
    };
  },
  async mounted() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);

    this.$nextTick(async () => {
      if (this.$store.getters["oldFiles"]) {
        this.oldFiles = true;
        this.showDialog();
      }

      // Doesn't work because when the component is mounted, the database error
      // is not yet propagated.
      // if (this.$store.getters["databaseFileTypeError"]) {
      //   this.dbFileTypeError = true;
      //   this.showDialog();
      // }

      await getCurrent().listen("request-factory-reset", () =>
        this.showDialog()
      );
    });

    listen("WrongDatabaseFileType", (event) => {
      this.heading = "Database File Type Error";
      this.dbFileTypeError = true;
      this.showDialog();
    });
  },
  methods: {
    showDialog() {
      (this.$refs.dialog as Dialog).show();
    },
    async executeFactoryReset() {
      try {
        this.executing = true;
        await invoke("execute_factory_reset");
        this.executing = false;
        window.location.reload();
      } catch (e) {
        this.executing = false;
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
