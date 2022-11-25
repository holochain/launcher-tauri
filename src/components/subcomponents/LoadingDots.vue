<template>
  <div class="container row">
    <div class="dot"></div>
    <div class="dot"></div>
    <div class="dot"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "LoadingDots",
});
</script>

<style scoped>
.container {
  --fill-color: white;
  --radius: 10px;
  --dim-color: #6b66c9;
  --light-color: var(--fill-color);
}

.dot {
  height: var(--radius);
  width: var(--radius);
  background-color: var(--fill-color);
  border-radius: 50%;
  margin: calc(var(--radius) * 0.25);
}

@keyframes blink {
  0% {
    opacity: 0.2;
  }
  20% {
    opacity: 1;
  }
  100% {
    opacity: 0.2;
  }
}

.container .dot {
  animation-name: blink;
  animation-duration: 2s;
  animation-iteration-count: infinite;
  /**
     * This makes sure that the starting style (opacity: .2)
     * of the animation is applied before the animation starts.
     * Otherwise we would see a short flash or would have
     * to set the default styling of the dots to the same
     * as the animation. Same applies for the ending styles.
     */
  animation-fill-mode: both;
}

.container .dot:nth-child(2) {
  animation-delay: 0.3s;
}

.container .dot:nth-child(3) {
  animation-delay: 0.6s;
}
</style>
