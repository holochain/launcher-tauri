<template>
  <button
    class="btn"
    :class="{ outlined: outlined }"
    :disabled="disabled"
    tabindex="0"
  >
    <slot></slot>
  </button>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCButton",
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
  --hc-secondary-color: #5537fc;
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
  cursor: pointer;
}

.btn:hover {
  color: #ffffff;
}

.btn:focus-visible {
  background-color: var(--hc-secondary-color);
  box-shadow: 0px 0px 4px var(--hc-primary-color);
}

.outlined {
  background-color: transparent;
  color: var(--hc-primary-color);
  border: 2px solid var(--hc-primary-color);
}

.outlined:hover {
  background-color: transparent;
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

.outlined:focus-visible {
  background-color: transparent;
  box-shadow: 0px 0px 4px var(--hc-secondary-color);
}
</style>
