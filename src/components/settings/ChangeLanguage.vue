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
          {{ $t("setup.login.changeLanguage") }}
        </div>
      </div>
      <div class="row" style="margin-top: 20px; margin-bottom: 16px">
        <HCSelect
          ref="selectLogLevel"
          style="margin: 5px; width: 360px"
          label="Log Level"
          :items=""
          @item-selected="newConfig.log_level = $event"
        >
        </HCSelect>
      </div>

    </div>
  </HCGenericDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { getCurrent } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/tauri";
import { languageNames } from "../../locale";

import HCGenericDialog from "../subcomponents/HCGenericDialog.vue";
import HCSelect from "../subcomponents/HCSelect.vue";

export default defineComponent({
  name: "ChangeLanguage",
  components: { HCGenericDialog, HCSelect },
  data(): {
    languages: [string, string][];
    selectedLanguage: string | undefined;
  } {
    return {
      languages: [
        // ["ERROR", "ERROR"],
        // ["WARN (default)", "WARN"],
        // ["INFO", "INFO"],
        // ["DEBUG", "DEBUG"],
        // ["TRACE", "TRACE"],
      ],
      selectedLanguage: undefined,
    };
  },
  mounted() {
  },
  methods: {
    async saveLanguage() {
      alert("attempted to save language");
      window.location.reload();
    },
  },
});
</script>
