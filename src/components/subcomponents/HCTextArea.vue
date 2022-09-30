<template>
  <div class="container">
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
  /* outline: 2px solid rgba(59, 61, 115, 0.15); */
  border-radius: 10px;
  /* box-shadow: 0 0 0 2px rgba(59, 61, 115, 0.15); */
  border: 2px solid rgba(59, 61, 115, 0.15);
}
.textarea {
  all: unset;
  resize: vertical;
  margin: 10px 10px 0px 10px;
  height: 100%;
}

.container:focus-within {
  border: 2px solid #482edf;
}
</style>
