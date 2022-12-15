<template>
  <div class="substrate" :style="`--width: ${progress}%`">
    <div class="progress"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCProgressBar",
  props: {
    progress: {
      type: Number, // between 0 and 100
      default: 50,
    },
  },
});
</script>

<style scoped>
.substrate {
  --hc-primary-color: #482edf;
  --hc-secondary-color: #5537fc;
  --height: 20px;
  --width: 100%;
  height: var(--height);
  background: #e8e8eb;
  border-radius: var(--height);
}

.progress {
  width: var(--width);
  height: var(--height);
  background-color: var(--hc-primary-color);
  border-radius: var(--height);
}
</style>
