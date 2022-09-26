<template>
  <button class="btn" :class="{ outlined: this.outlined }" :disabled="disabled">
    <slot></slot>
  </button>
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
    outlined: {
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

.outlined {
  background-color: transparent;
  color: #482edf;
  border: 2px solid #482edf;
}

.outlined:hover {
  color: #482edf;
}

.btn:disabled {
  background-color: #d7d7e3;
}

.outlined:disabled {
  background-color: transparent;
  color: #d7d7e3;
  border: 2px solid #d7d7e3;
}
</style>
