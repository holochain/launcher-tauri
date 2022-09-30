<template>
  <div ref="container" tabindex="0" @keydown.esc="close">
    <dialog
      @click="closeOnSideClick ? close() : null"
      class="background"
      :open="isOpen"
    ></dialog>
    <dialog class="dialog-element" :open="isOpen">
      <slot></slot>
      <slot name="secondaryAction"></slot>
      <slot name="primaryAction"></slot>
    </dialog>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCDialog",
  props: {
    placeholder: {
      type: String,
      default: "Enter password",
    },
    required: {
      type: Boolean,
      default: true,
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
      (this.$refs.container as HTMLElement).focus();
    },
    close() {
      this.isOpen = false;
      this.$emit("closing");
    },
  },
});
</script>

<style scoped>
.background {
  border: none;
  background-color: rgba(0, 0, 0, 0.6);
  width: 100vw;
  height: 100vh;
  z-index: 1;
  padding: 0;
  position: absolute;
  top: 0;
}

.dialog-element {
  border: none;
  border-radius: 15px;
  position: absolute;
  left: 50vw;
  top: 50vh;
  transform: translate(-50%, -50%);
  margin: 0; /*reset some browser centering*/
  z-index: 1;
}
</style>
