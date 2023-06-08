<template>
  <HCGenericDialog
    ref="dialog"
    @confirm="saveConfig()"
    @closing="restoreCurrent()"
    :primaryButtonLabel="$t('dialogs.config.saveAndRestart')"
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
          style="margin: 5px; width: 380px;"
          label="Log Level"
          :items="levels"
          @item-selected="newConfig.log_level = $event"
        >
        </HCSelect>
      </div>

      <HCTextField
        ref="bootstrapServerField"
        placeholder="https://bootstrap.holo.host"
        style="margin: 5px; width: 380px;"
        label="Bootstrap Server URL"
      ></HCTextField>

      <HCTextField
        ref="proxyServerField"
        placeholder="kitsune-proxy://f3gH2VMkJ4qvZJOXx0ccL_Zo5n-s_CnBjSzAsEHHDCA/kitsune-quic/h/137.184.142.208/p/5788/--"
        style="margin: 5px; margin-top: 20px; width: 380px;"
        label="Proxy Server URL"
      ></HCTextField>

      <div class="row" style="margin: 5px; margin-top: 20px; width: 380px;">
        <ToggleSwitch
          ref="sliderMdns"
          :sliderOn="mdns"
          @click="handleSliderMdns"
        />
        <span style="margin-left: 10px">{{ $t('dialogs.config.useMdns') }} <i>{{ $t('dialogs.config.useMdnsNote') }}</i></span>
      </div>

      <div class="row" style="margin: 5px; margin-top: 20px; width: 380px;">
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
        :default="customBinaryPath"
      ></HCTextField>
    </div>

    <HCButton
      outlined
      style="min-width: 270px; height: 30px; margin-bottom: -15px; margin-top: 10px;"
      @click="restoreDefaults()"
    >{{ $t('dialogs.config.restoreDefaults') }}
    </HCButton>

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
import HCButton from "../subcomponents/HCButton.vue";

export default defineComponent({
  name: "Config",
  components: { HCButton, HCGenericDialog, HCSelect, HCTextField, ToggleSwitch },
  data(): {
    levels: [string, string][];
    newConfig: any;
    customBinary: boolean;
    customBinaryPath: string;
    currentLogLevel: [string, string] | undefined;
    mdns: boolean;
    proxyUrl: string | undefined;
    bootstrapUrl: string | undefined;
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
      mdns: false,
      proxyUrl: undefined,
      bootstrapUrl: undefined,
    };
  },
  mounted() {
    this.$nextTick(async () => {
      const dialog = this.$refs.dialog as typeof HCGenericDialog;
      await getCurrent().listen("open-config", async () => {
        //await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.restoreCurrent();

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

      (this.newConfig as any).mdns = this.mdns;

      if (this.proxyUrl) {
        (this.newConfig as any).proxy_server_url = (this.$refs.proxyServerField as typeof HCTextField).value;
      }

      if (this.bootstrapUrl) {
        (this.newConfig as any).bootstrap_server_url = (this.$refs.bootstrapServerField as typeof HCTextField).value;
      }

      // console.log("newConfig: ", this.newConfig);
      await invoke("write_config", { config: this.newConfig });
      window.location.reload();
    },
    handleSlider() {
      this.customBinary = !this.customBinary;
    },
    handleSliderMdns() {
      this.mdns = !this.mdns;
    },
    /**
     * Restoring to the values as they are currently stored in the launcher-config.yaml
     */
    restoreCurrent() {
      const currentConfig = (this.$store.state.launcherStateInfo as any).config;

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

      const mdns = (this.newConfig as any).mdns;

      if (mdns) {
        this.mdns = true;
      }

      const currentProxyUrl = (this.newConfig as any)
        .proxy_server_url;

      if (currentProxyUrl && currentProxyUrl !== "") {
        (this.$refs.proxyServerField as typeof HCTextField).setValue(currentProxyUrl);
        this.proxyUrl = currentProxyUrl;
      }

      const currentBootstrapUrl = (this.newConfig as any)
        .bootstrap_server_url;

      if (currentBootstrapUrl && currentBootstrapUrl !== "") {
        (this.$refs.bootstrapServerField as typeof HCTextField).setValue(currentBootstrapUrl);
        this.bootstrapUrl = currentBootstrapUrl;
      }
    },
    async restoreDefaults() {
      this.currentLogLevel = this.levels[1];
      (this.$refs.selectLogLevel as typeof HCSelect).select(
        this.currentLogLevel
      );

      const bootstrapUrl: string = await invoke("get_default_bootstrap", {});
      this.bootstrapUrl = bootstrapUrl;
      (this.$refs.bootstrapServerField as typeof HCTextField).setValue(bootstrapUrl);

      const proxyUrl: string = await invoke("get_default_proxy", {});
      this.proxyUrl = proxyUrl;
      (this.$refs.proxyServerField as typeof HCTextField).setValue(proxyUrl);

      this.mdns = false;
      this.customBinary = false;
    },
  },
});
</script>
