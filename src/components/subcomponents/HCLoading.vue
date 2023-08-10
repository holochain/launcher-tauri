<template>
  <HCDialog
    style="padding: 0"
    ref="dialog"
    :closeOnSideClick="closeOnSideClick"
  >
    <div
      class="column"
      style="
        align-items: center;
        justify-content: center;
        width: 312px;
        overflow: hidden;
        position: relative;
      "
    >
      <!-- <img class="halo" src="/img/Holochain_Halo_no_gradient.svg" /> -->
      <HCCircularProgress
        style="margin-top: 40px; margin-bottom: 40px"
      ></HCCircularProgress>
      <div style="margin-bottom: 10px; text-align: center; padding: 0 30px;">{{ text }}</div>
    </div>
  </HCDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import HCDialog from "../subcomponents/HCDialog.vue";
import HCCircularProgress from "./HCCircularProgress.vue";

export default defineComponent({
  name: "HCLoading",
  components: { HCCircularProgress, HCDialog },
  props: {
    text: {
      type: String,
      default: "Loading...",
    },
    closeOnSideClick: {
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
      this.isOpen = true;
      (this.$refs.dialog as typeof HCDialog).open();
    },
    close() {
      this.isOpen = false;
      (this.$refs.dialog as typeof HCDialog).close();
      this.$emit("closing");
    },
  },
});
</script>

<style scoped>
.halo {
  height: 130%;
  top: -3%;
  position: absolute;
  opacity: 50%;
}
</style>
