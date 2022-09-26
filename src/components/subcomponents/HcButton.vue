<template>
  <button class="btn" :disabled="disabled"><slot></slot></button>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HcButton",
  props: {
    disabled: {
      type: Boolean,
      default: false,
    },
  },
  data(): {
    passwordVisible: boolean;
    focus: boolean;
  } {
    return {
      passwordVisible: false,
      focus: false,
    };
  },
});
</script>

<style scoped>
.btn {
  all: unset;
  font-weight: 600;
  background-color: #482edf;
  width: intrinsic;
  height: 34px;
  padding: 0 30px;
  text-align: center;
  color: #ffffff;
  border-radius: 70px;
}

.btn:hover {
  color: #ffffff;
}

.btn:disabled {
  background-color: #d7d7e3;
}
</style>
