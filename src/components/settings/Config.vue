<template>
  <mwc-dialog
    heading="Configuration"
    ref="dialog"
    scrimClickAction=""
    escapeKeyAction=""
  >
    <div class="column" style="min-height: 400px; min-width: 400px">
      <div class="row">
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

export default defineComponent({
  name: "Config",
  data() {
    return {
      levels: ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"],
      newConfig: {},
    };
  },
  mounted() {
    this.$nextTick(async () => {
      const dialog = this.$refs.dialog as Dialog;
      await getCurrent().listen("open-config", async () => {
        //await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.newConfig = (this.$store.state.launcherStateInfo as any).config;
        dialog.show();
      });
    });
  },
  methods: {
    async saveConfig() {
      await invoke("write_config", { config: this.newConfig });
      window.location.reload();
    },
  },
});
</script>
