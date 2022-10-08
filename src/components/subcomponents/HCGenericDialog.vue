<template>
  <HCDialog ref="dialog" :closeOnSideClick="closeOnSideClick">
    <div
      class="column"
      style="align-items: center; padding: 40px 10px 20px 10px"
    >
      <slot></slot>

      <div class="row" style="margin-top: 30px">
        <HCButton
          v-if="!hideCancel"
          style="width: 80px; height: 30px; margin: 4px 6px"
          outlined
          @click="close()"
          >Cancel</HCButton
        >
        <HCButton
          style="min-width: 80px; margin: 4px 6px"
          @click="handleConfirm"
          :disabled="primaryButtonDisabled"
          >{{ primaryButtonLabel }}
        </HCButton>
      </div>
    </div>
  </HCDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import HCButton from "./HCButton.vue";
import HCDialog from "./HCDialog.vue";

export default defineComponent({
  name: "HCGenericDialog",
  components: { HCButton, HCDialog },
  props: {
    primaryButtonLabel: {
      type: String,
      default: "Ok",
    },
    closeOnSideClick: {
      type: Boolean,
      default: false,
    },
    primaryButtonDisabled: {
      type: Boolean,
      default: false,
    },
    closeOnConfirm: {
      type: Boolean,
      default: false,
    },
    hideCancel: {
      type: Boolean,
      default: false,
    },
  },
  data(): {
    isOpen: boolean;
  } {
    return {
      isOpen: false,
    };
  },
  methods: {
    open() {
      this.$nextTick(() => (this.$refs["dialog"] as typeof HCDialog).open());
    },
    close() {
      (this.$refs["dialog"] as typeof HCDialog).close();
      this.$emit("closing");
    },
    handleConfirm() {
      this.$emit("confirm");
      if (this.closeOnConfirm) {
        this.close();
      }
    },
  },
});
</script>
