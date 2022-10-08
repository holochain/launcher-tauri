<template>
  <HCDialog ref="dialog" prohibitEscape>
    <div class="column" style="align-items: center; margin: 30px">
      <div class="row" style="align-items: center; margin-bottom: 20px">
        <HCErrorIcon style="font-size: 1.3em" />
        <div style="margin-left: 12px; font-size: 1.4em">{{ heading }}</div>
      </div>
      <slot></slot>
      <div class="row" style="margin-top: 50px">
        <HCButton
          outlined
          @click="reportIssue()"
          style="margin: 0 8px; height: 30px; width: 140px"
          >Report Issue</HCButton
        >
        <HCButton @click="restartLauncher()" style="margin: 0 8px; width: 140px"
          >Restart</HCButton
        >
      </div>
    </div>
  </HCDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import "@material/mwc-dialog";
import "@material/mwc-button";
import HCErrorIcon from "../subcomponents/HCErrorIcon.vue";
import HCDialog from "../subcomponents/HCDialog.vue";
import HCButton from "../subcomponents/HCButton.vue";

export default defineComponent({
  name: "Error",
  components: { HCErrorIcon, HCDialog, HCButton },
  props: {
    heading: String,
  },
  created() {
    this.$nextTick(() => {
      (this.$refs.dialog as typeof HCDialog).open();
    });
  },
  methods: {
    async reportIssue() {
      await invoke("report_issue_cmd", {});
    },
    async restartLauncher() {
      await invoke("restart");
    },
  },
});
</script>
