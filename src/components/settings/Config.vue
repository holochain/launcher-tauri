<template>
  <HCGenericDialog
    ref="dialog"
    @confirm="saveConfig()"
    primaryButtonLabel="Save and Restart"
    closeOnSideClick
  >
    <div
      class="column"
      style="min-height: 200px; min-width: 400px; margin: -30px 20px 20px 20px"
    >
      <div class="row" style="justify-content: center">
        <div style="font-weight: 600; font-size: 25px; margin: 20px 0 10px 0">
          Configuration
        </div>
      </div>
      <div class="row" style="margin-top: 20px; margin-bottom: 16px">
        <HCSelect
          ref="selectLogLevel"
          style="margin: 5px; width: 360px"
          label="Log Level"
          :items="levels"
          @item-selected="newConfig.log_level = $event"
        >
        </HCSelect>
      </div>

      <div class="row" style="margin: 5px">
        <ToggleSwitch
          ref="slider"
          :sliderOn="customBinary"
          @click="handleSlider"
        />
        <span style="margin-left: 10px">Holochain Custom Binary</span>
      </div>

      <HCTextField
        v-if="customBinary"
        ref="customBinaryField"
        placeholder="/path/to/custom/binary"
        style="margin-top: 20px; width: 100%"
        label="Custom Holochain Binary Path"
        :default="this.customBinaryPath"
      ></HCTextField>
    </div>
  </HCGenericDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { getCurrent } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";

import HCGenericDialog from "../subcomponents/HCGenericDialog.vue";
import HCSelect from "../subcomponents/HCSelect.vue";
import HCTextField from "../subcomponents/HCTextField.vue";
import ToggleSwitch from "../subcomponents/ToggleSwitch.vue";

export default defineComponent({
  name: "Config",
  components: { HCGenericDialog, HCSelect, HCTextField, ToggleSwitch },
  data(): {
    levels: [string, string][];
    newConfig: any;
    customBinary: boolean;
    customBinaryPath: string;
    currentLogLevel: [string, string] | undefined;
  } {
    return {
      levels: [
        ["ERROR", "ERROR"],
        ["WARN (default)", "WARN"],
        ["INFO", "INFO"],
        ["DEBUG", "DEBUG"],
        ["TRACE", "TRACE"],
      ],
      newConfig: {},
      customBinary: false,
      customBinaryPath: "/sample/path",
      currentLogLevel: undefined,
    };
  },
  mounted() {
    this.$nextTick(async () => {
      const dialog = this.$refs.dialog as typeof HCGenericDialog;
      await getCurrent().listen("open-config", async () => {
        //await this.$store.dispatch(ActionTypes.fetchStateInfo);
        const currentConfig = (this.$store.state.launcherStateInfo as any)
          .config;
        this.newConfig = currentConfig;
        this.currentLogLevel = this.levels.find(
          (level) => level[1] === currentConfig.log_level
        );
        (this.$refs.selectLogLevel as typeof HCSelect).select(
          this.currentLogLevel
        );
        const currentCustomBinaryPath = (this.newConfig as any)
          .custom_binary_path;

        if (!this.customBinary) {
          this.customBinary =
            currentCustomBinaryPath && currentCustomBinaryPath !== "";
        }

        if (currentCustomBinaryPath !== "") {
          this.customBinaryPath = currentCustomBinaryPath;
        }

        dialog.open();
      });
    });
  },
  methods: {
    async saveConfig() {
      if (this.customBinary) {
        (this.newConfig as any).custom_binary_path = (
          this.$refs.customBinaryField as typeof HCTextField
        ).value;
      } else {
        (this.newConfig as any).custom_binary_path = null;
      }
      console.log("newConfig: ", this.newConfig);
      await invoke("write_config", { config: this.newConfig });
      window.location.reload();
    },
    handleSlider() {
      this.customBinary = !this.customBinary;
    },
  },
});
</script>
