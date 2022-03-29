<template>
  <mwc-dialog :heading="heading" scrimClickAction="" escapeKeyAction="" open>
    <slot></slot>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Error",
  props: {
    heading: String,
  },
});
</script>
