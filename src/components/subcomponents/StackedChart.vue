<template>
  <div class="row container">
    <div
      v-for="(fraction, idx) in fractions"
      :key="idx"
      :style="`width: ${fraction}%; background-color: ${colors[idx]}; margin: 0; padding: 0; position: relative;`"
      class="section"
      @mouseenter="tooltipIndex = idx"
      @mouseleave="tooltipIndex = undefined"
    >
      <div v-if="tooltipIndex == idx && labels" class="tooltip">
        {{ labels[idx] }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
  name: "StackedChart",
  props: {
    fractions: {
      // fractions in percent.
      type: Object as PropType<Array<number>>,
    },
    labels: {
      type: Object as PropType<Array<string>>,
    },
  },
  data(): {
    colors: string[];
    tooltipIndex: number | undefined;
  } {
    return {
      colors: [
        "#bf4bf4",
        "#f44b4b",
        "#e9f44b",
        "#5cf44b",
        "#4bf4d0",
        "#4bb6f4",
        "#4b4bf4",
      ],
      tooltipIndex: undefined,
    };
  },
});
</script>

<style scoped>
.container {
  --height: 30px;
  height: var(--height);
}

.section:hover {
  opacity: 0.7;
}

.tooltip {
  position: absolute;
  white-space: nowrap;
  /* color: #482edf; */
  color: white;
  bottom: calc(var(--height) + 15px);
  left: 20px;
  background: black;
  border-radius: 5px;
  /* border: 2px solid #482edf; */
  padding: 1px 7px;
}
</style>
