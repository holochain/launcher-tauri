<template>
  <input
    :required="this.required"
    class="textfield"
    v-model="value"
    :placeholder="placeholder"
  />
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
      default: true,
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
.textfield {
  all: unset;
  height: 42px;
  padding: 0 10px;
  width: 300px;
  /* outline: 2px solid rgba(59, 61, 115, 0.15); */
  border-radius: 10px;
  /* box-shadow: 0 0 0 2px rgba(59, 61, 115, 0.15); */
  border: 2px solid rgba(59, 61, 115, 0.15);
}

.textfield:focus {
  border: 2px solid #482edf;
}
</style>
