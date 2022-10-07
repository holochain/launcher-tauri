<template>
  <div
    v-show="isOpen"
    role="dialog"
    ref="container"
    tabindex="0"
    @keydown.esc="close"
  >
    <div @click="closeOnSideClick ? close() : null" class="background"></div>
    <div class="dialog-element">
      <div class="column">
        <div class="row top-bar">
          {{ heading }}
        </div>
        <div class="slot-container">
          <img class="halo" src="/img/Holochain_Halo_complete.svg" />
          <slot></slot>
        </div>
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
    heading: {
      type: String,
      default: "",
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
  z-index: 9998;
}

.dialog-element {
  padding: 0;
  max-height: 90vh;
  max-width: 90vh;
  border: none;
  border-radius: 8px;
  position: absolute;
  left: 50vw;
  top: 50vh;
  transform: translate(-50%, -50%);
  margin: 0; /*reset some browser centering*/
  z-index: 9999;
  border: 4px solid var(--hc-primary-color);
  box-shadow: 0 0px 5px #9b9b9b;
  background: white;
}

.top-bar {
  align-items: center;
  height: 56px;
  background: #301e9a;
  box-shadow: 0 0px 5px #9b9b9b;
  border-radius: 8px 8px 0 0;
  padding-left: 30px;
  font-size: 25px;
  font-weight: 600;
  color: white;
  overflow: hidden;
}

.halo {
  height: 150%;
  position: absolute;
  top: -3%;
  opacity: 50%;
}

.slot-container {
  position: relative;
  overflow: hidden;
}
</style>
