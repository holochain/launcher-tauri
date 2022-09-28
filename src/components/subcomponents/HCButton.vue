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
button {
  --hc-primary-color: #482edf;
}
.btn {
  all: unset;
  font-weight: 600;
  background-color: var(--hc-primary-color);
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
  color: var(--hc-primary-color);
  border: 2px solid var(--hc-primary-color);
}

.outlined:hover {
  color: var(--hc-primary-color);
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
