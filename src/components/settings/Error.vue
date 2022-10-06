<template>
  <HCGenericDialog
    @confirm="reportIssue()"
    hideCancel
    primaryButtonLabel="Report Issue"
    ref="dialog"
  >
    <div class="column" style="align-items: center">
      <div
        class="row"
        style="align-items: center; margin-top: -20px; margin-bottom: 20px"
      >
        <HCErrorIcon style="font-size: 1.3em" />
        <div style="margin-left: 12px; font-size: 1.4em">{{ heading }}</div>
      </div>
      <slot></slot>
    </div>
  </HCGenericDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import "@material/mwc-dialog";
import "@material/mwc-button";
import HCErrorIcon from "../subcomponents/HCErrorIcon.vue";
import HCGenericDialog from "../subcomponents/HCGenericDialog.vue";

export default defineComponent({
  name: "Error",
  components: { HCErrorIcon, HCGenericDialog },
  props: {
    heading: String,
  },
  created() {
    this.$nextTick(() => {
      (this.$refs.dialog as typeof HCGenericDialog).open();
    });
  },
  methods: {
    async reportIssue() {
      await invoke("report_issue_cmd", {});
    },
  },
});
</script>
