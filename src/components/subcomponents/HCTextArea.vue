<template>
  <div class="container" style="position: relative">
    <div class="label">{{ label }}</div>
    <textarea
      :required="required"
      :rows="rows"
      :cold="cols"
      class="textarea"
      v-model="value"
      :placeholder="placeholder"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCTextField",
  props: {
    placeholder: {
      type: String,
      default: "Enter text",
    },
    required: {
      type: Boolean,
      default: false,
    },
    rows: {
      type: Number,
      default: 4,
    },
    cols: {
      type: Number,
      default: 50,
    },
    label: {
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
  --active-border-color: rgba(59, 61, 115, 0.15);
  --active-label-color: rgba(59, 61, 115, 0.4);
}

.container:focus-within {
  --active-border-color: var(--hc-primary-color);
  --active-label-color: var(--hc-primary-color);
}

.container {
  /* outline: 2px solid rgba(59, 61, 115, 0.15); */
  border-radius: 10px;
  /* box-shadow: 0 0 0 2px rgba(59, 61, 115, 0.15); */
  border: 2px solid var(--active-border-color);
}
.textarea {
  all: unset;
  resize: vertical;
  margin: 10px 10px 0px 10px;
  height: 100%;
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
</style>
