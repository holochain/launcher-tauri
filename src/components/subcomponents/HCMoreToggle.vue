<template>
  <div
    style="width: 28px; height: 28px; cursor: pointer"
    @click="handleClick"
    v-on:keyup.enter="handleClick"
  >
    <img
      style="width: 28px"
      :class="{ rotated: showMore }"
      src="/img/More.svg"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCMoreToggle",
  emits: ["toggle"],
  data(): {
    showMore: boolean;
  } {
    return {
      showMore: false,
    };
  },
  methods: {
    handleClick() {
      this.showMore = !this.showMore;
      this.$emit("toggle");
    },
  },
});
</script>

<style scoped>
.rotated {
  transform: rotate(90deg);
}
</style>
