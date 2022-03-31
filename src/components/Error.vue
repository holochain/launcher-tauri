<template>
  <mwc-dialog :heading="heading" scrimClickAction="" escapeKeyAction="" open>
    <slot></slot>

    <mwc-button
      label="Report Issue"
      slot="primaryAction"
      @click="reportIssue()"
    >
    </mwc-button>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import "@material/mwc-dialog";
import "@material/mwc-button";

export default defineComponent({
  name: "Error",
  props: {
    heading: String,
  },
  methods: {
    async reportIssue() {
      await invoke("report_issue", {});
    },
  },
});
</script>
