<template>
  <HCDialog ref="dialog">
    <div class="column" style="align-items: center; margin: 10px 15px">
      <div style="font-weight: 700; font-size: 25px; margin: 20px 0 30px 0">
        Install App
      </div>
      <HCTextField placeholder="App Id" style="margin: 5px" label="App Id" />
      <HCTextField placeholder="Field 1" style="margin: 5px" />
      <div class="column" style="width: 100%">
        <div class="row" style="margin-top: 20px; align-items: center">
          <div
            @click="showAdvanced = !showAdvanced"
            style="
              text-align: center;
              font-size: 24px;
              width: 20px;
              cursor: pointer;
            "
          >
            {{ showAdvanced ? "-" : "+" }}
          </div>
          <div style="font-size: 20px; margin-left: 10px">Advanced</div>
        </div>
      </div>

      <div v-if="showAdvanced">
        <HCTextArea
          placeholder="Membrane Proof"
          style="margin: 5px"
          :cols="29"
          label="Membrane Proof"
        />
      </div>

      <div class="row" style="margin-top: 30px">
        <HCButton style="width: 80px; height: 30px; margin: 4px" outlined
          >Cancel</HCButton
        >
        <HCButton style="width: 80px; margin: 4px">Install</HCButton>
      </div>
    </div>
  </HCDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import HCButton from "./subcomponents/HCButton.vue";
import HCDialog from "./subcomponents/HCDialog.vue";
import HCTextField from "./subcomponents/HCTextField.vue";
import HCTextArea from "./subcomponents/HCTextArea.vue";

export default defineComponent({
  name: "InstallAppDialog",
  components: { HCDialog, HCTextField, HCTextArea, HCButton },
  data(): {
    showAdvanced: boolean;
  } {
    return {
      showAdvanced: false,
    };
  },
  methods: {
    open() {
      (this.$refs.dialog as typeof HCDialog).open();
    },
  },
});
</script>
