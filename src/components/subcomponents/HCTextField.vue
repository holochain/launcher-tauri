<template>
  <div class="container" style="position: relative">
    <div class="label">{{ label }}</div>
    <input
      :required="required"
      class="textfield"
      v-model="value"
      :placeholder="placeholder"
    />
    <div v-if="invalid" class="invalidity-note">{{ invalid }}</div>
    <div v-if="helper && !invalid" class="helper-note">{{ helper }}</div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCTextField",
  props: {
    placeholder: {
      type: String,
      default: "Enter password",
    },
    required: {
      type: Boolean,
      default: false,
    },
    label: {
      type: String,
      default: undefined,
    },
    invalid: {
      type: String,
      default: undefined,
    },
    helper: {
      type: String,
      default: undefined,
    },
  },
  data(): {
    focus: boolean;
    value: string;
  } {
    return {
      focus: false,
      value: "",
    };
  },
});
</script>

<style scoped>
.container {
  --hc-primary-color: #482edf;
  --active-border-color: rgba(59, 61, 115, 0.25);
  --active-label-color: rgba(59, 61, 115, 0.4);
}

.container:focus-within {
  --active-border-color: var(--hc-primary-color);
  --active-label-color: var(--hc-primary-color);
}

.textfield {
  all: unset;
  height: 48px;
  padding: 0 10px;
  width: 300px;
  /* outline: 2px solid rgba(59, 61, 115, 0.25); */
  border-radius: 10px;
  /* box-shadow: 0 0 0 2px rgba(59, 61, 115, 0.25); */
  border: 2px solid var(--active-border-color);
}

.textfield:focus {
  border: 2px solid var(--hc-primary-color);
}

.label {
  color: var(--active-label-color);
  padding: 0 4px;
  font-size: 14px;
  background: white;
  position: absolute;
  top: -10px;
  left: 12px;
  margin: 0 10px;
}

.invalidity-note {
  margin-top: -1px;
  margin-left: 15px;
  font-size: 13px;
  color: #e00000;
}

.helper-note {
  margin-left: 15px;
  margin-top: -1px;
  font-size: 13px;
  color: rgba(59, 61, 115, 0.5);
}
</style>
