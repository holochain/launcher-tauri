<template>
  <div class="container">
    <div class="row snackbar" :class="{ invisible: !showSnackbar }">
      {{ labelText }}
      <!-- <div style="margin-right: 40px;">{{ labelText }}</div>
      <div class="close" @click="close"></div> -->
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCSnackbar",
  props: {
    labelText: {
      type: String,
      default: "I'm a Snackbar.",
    },
    timeoutMs: {
      type: Number,
      default: 6000,
    },
  },
  data(): {
    showSnackbar: boolean;
    latestTriggerCounter: number;
  } {
    return {
      showSnackbar: false,
      latestTriggerCounter: 0,
    };
  },
  methods: {
    show() {
      const triggerCounter = this.latestTriggerCounter;
      this.latestTriggerCounter += 1;
      this.showSnackbar = true;
      setTimeout(() => {
        // only set it to false if show has not been called again before this callback gets called
        if (triggerCounter === this.latestTriggerCounter - 1)
          this.showSnackbar = false;
      }, this.timeoutMs);
    },
    close() {
      this.showSnackbar = false;
    },
  },
});
</script>

<style scoped>
.container {
  --hc-primary-color: #482edf;
  --hc-text-color: white;
  position: fixed;
  bottom: 5vh;
  left: 50%;
  transform: translateX(-50%);
  z-index: 10000; /* make sure this is higher than the z-index of HCDialog! */
}
.invisible {
  display: none;
}
.snackbar {
  background-color: var(--hc-primary-color);
  color: var(--hc-text-color);
  border-radius: 5px;
  /* box-shadow: 0 0px 5px #9b9b9b; */
  box-shadow: 0 0px 5px rgb(65, 65, 65);
  max-height: 100px;
  max-width: 90vw;
  padding: 20px;
  overflow: auto;
}
.close {
  cursor: pointer;
  width: 35px;
  height: 35px;
  color: var(--hc-text-color);
  position: absolute;
  top: 0;
  right: 0;
  margin: 8px;
  border-radius: 5px;
}
.close:hover {
  background-color: rgba(255, 255, 255, 0.3);
}
</style>
