<template>
  <div
    v-show="isOpen"
    role="dialog"
    ref="container"
    tabindex="0"
    @keydown.esc="handleEscape"
  >
    <div @click="closeOnSideClick ? close() : null" class="background"></div>
    <div class="dialog-element">
      <div class="column">
        <slot></slot>
      </div>
      <slot name="secondaryAction"></slot>
      <slot name="primaryAction"></slot>
    </div>
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
    prohibitEscape: {
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
    handleEscape() {
      if (this.prohibitEscape) {
        return;
      }
      this.close();
    },
  },
});
</script>

<style scoped>
.container {
  --hc-primary-color: #482edf;
  --hc-secondary-color: #674df9;
}

.background {
  border: none;
  background-color: #e8e8eb;
  opacity: 0.9;
  width: 100vw;
  height: 100vh;
  padding: 0;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 9999;
}

.dialog-element {
  padding: 0;
  max-height: 90vh;
  max-width: 90vw;
  border: none;
  border-radius: 12px;
  background-color: white;
  position: fixed;
  left: 50vw;
  top: 50vh;
  transform: translate(-50%, -50%);
  margin: 0; /*reset some browser centering*/
  border: 4px solid var(--hc-primary-color);
  box-shadow: 0 0px 5px #9b9b9b;
  overflow-y: auto;
  z-index: 9999;
}
</style>
