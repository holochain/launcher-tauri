<template>
  <HCGenericDialog
    ref="dialog"
    @confirm="executeFactoryReset()"
    closeOnSideClick
    :primaryButtonDisabled="executing"
    :primaryButtonLabel="
      executing
        ? $t('dialogs.factoryReset.executing')
        : $t('dialogs.factoryReset.primaryButton')
    "
  >
    <div class="column" style="margin: 0 20px; max-width: 500px">
      <div
        style="
          font-weight: 600;
          font-size: 1.5em;
          width: 100%;
          text-align: center;
          margin-bottom: 25px;
          margin-top: -10px;
        "
        >
        {{ $data.heading }}
      </div>

      <div style="margin-top: 8px; text-align: center">
        {{ $t("dialogs.factoryReset.part1") }}
        <b>{{ $t("dialogs.factoryReset.bold1") }}</b>
        {{ $t("dialogs.factoryReset.part2") }}<br /><br />
        {{ $t("dialogs.factoryReset.part3")
        }}<b>{{ $t("dialogs.factoryReset.bold2") }}</b>
      </div>

      <div style="margin-top: 40px; margin-left: 20px; text-align: left:">
        <div style="font-weight: bold;">{{ $t("dialogs.factoryReset.optionalDeletions") }}</div>
        <div class="row" style="margin-top: 8px;">
          <ToggleSwitch
            :sliderOn="deleteLair"
            @click="() => deleteLair = !deleteLair"
            @keydown.enter="deleteLair = !deleteLair"
          />
          <span style="margin-left: 10px;">{{ $t("dialogs.factoryReset.deleteLair") }}</span>
        </div>
        <div class="row" style="margin-top: 5px;">
          <ToggleSwitch
            :sliderOn="deleteLogs"
            @click="() => deleteLogs = !deleteLogs"
            @keydown.enter="() => deleteLogs = !deleteLogs"
          />
          <span style="margin-left: 10px;">{{ $t("dialogs.factoryReset.deleteLogs") }}</span>
        </div>
      </div>

    </div>
  </HCGenericDialog>
  <HCSnackbar :labelText="snackbarText" ref="snackbar"></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { getCurrent } from "@tauri-apps/api/window";
import { ActionTypes } from "../store/actions";
import { listen } from "@tauri-apps/api/event";

import HCGenericDialog from "../components/subcomponents/HCGenericDialog.vue";
import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import ToggleSwitch from "../components/subcomponents/ToggleSwitch.vue";

export default defineComponent({
  name: "FactoryReset",
  components: { HCGenericDialog, HCSnackbar, ToggleSwitch },
  data(): {
    snackbarText: string | undefined;
    executing: boolean;
    oldFiles: boolean;
    heading: string;
    dbFileTypeError: boolean;
    deleteLair: boolean;
    deleteLogs: boolean;
  } {
    return {
      snackbarText: undefined,
      executing: false,
      oldFiles: false,
      heading: "Factory Reset",
      dbFileTypeError: false,
      deleteLair: false,
      deleteLogs: false,
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
      (this.$refs.dialog as typeof HCGenericDialog).open();
    },
    async executeFactoryReset() {
      try {
        this.executing = true;
        await invoke("execute_factory_reset", { deleteLair: this.deleteLair, deleteLogs: this.deleteLogs });
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
