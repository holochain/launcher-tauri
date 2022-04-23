<template>
  <mwc-dialog
    heading="Configuration"
    ref="dialog"
    scrimClickAction=""
    escapeKeyAction=""
  >
    <div class="column" style="min-height: 200px; min-width: 400px">
      <div class="row" style="margin-top: 8px; margin-bottom: 16px">
        <mwc-select
          outlined
          :fixedMenuPosition="true"
          label="Log Level"
          @selected="newConfig.log_level = levels[$event.detail.index]"
        >
          <mwc-list-item
            v-for="level of levels"
            :key="level"
            :selected="newConfig.log_level === level"
            :value="level"
            >{{ level }}</mwc-list-item
          >
        </mwc-select>
      </div>

      <mwc-formfield label="Custom Holochain Binary">
        <mwc-checkbox
          :checked="customBinary"
          @change="toggleCustomBinary($event.target.checked)"
        ></mwc-checkbox>
      </mwc-formfield>
      <mwc-textfield
        v-if="customBinary"
        label="Custom Holochain Binary Path"
        :value="this.newConfig.custom_binary_path || ''"
        outlined
        @input="this.newConfig.custom_binary_path = $event.target.value"
      ></mwc-textfield>
    </div>

    <mwc-button
      slot="secondaryAction"
      dialogAction="close"
      label="Cancel"
    ></mwc-button>
    <mwc-button
      slot="primaryAction"
      @click="saveConfig()"
      label="Save and Restart"
    ></mwc-button>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { getCurrent } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import type { Dialog } from "@material/mwc-dialog";
import "@material/mwc-dialog";
import "@material/mwc-select";
import "@material/mwc-button";
import "@material/mwc-textfield";
import "@material/mwc-checkbox";
import "@material/mwc-formfield";

export default defineComponent({
  name: "Config",
  data() {
    return {
      levels: ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"],
      newConfig: {},
      customBinary: false,
    };
  },
  mounted() {
    this.$nextTick(async () => {
      const dialog = this.$refs.dialog as Dialog;
      await getCurrent().listen("open-config", async () => {
        //await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.newConfig = (this.$store.state.launcherStateInfo as any).config;
        console.log(this.newConfig);
        this.customBinary =
          (this.newConfig as any).custom_binary_path &&
          (this.newConfig as any).custom_binary_path !== "";
        dialog.show();
      });
    });
  },
  methods: {
    toggleCustomBinary(enabled: boolean) {
      this.customBinary = enabled;
      if (enabled) (this.newConfig as any).custom_binary_path = "/sample/path";
      else (this.newConfig as any).custom_binary_path = undefined;
    },
    async saveConfig() {
      await invoke("write_config", { config: this.newConfig });
      window.location.reload();
    },
  },
});
</script>
