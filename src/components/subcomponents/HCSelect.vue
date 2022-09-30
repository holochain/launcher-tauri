<template>
  <div
    class="container"
    style="position: relative"
    @click="showSelection = !showSelection"
    tabindex="0"
  >
    <div class="label">{{ label }}</div>
    <div class="input-field row" style="align-items: center">
      <div :class="{ greyedOut: selectedKey ? false : true }">
        {{ selectedKey ? selectedKey : "Select..." }}
      </div>
      <span style="display: flex; flex: 1"></span>
      <img
        src="/img/select_arrow.svg"
        style="width: 12px; margin-right: 5px"
        :class="{ rotated: showSelection }"
      />
    </div>
    <div v-if="helper" class="helper-note">{{ helper }}</div>
    <div v-if="showSelection" class="items-list">
      <div
        class="item row"
        v-for="(item, index) of items"
        :key="index"
        @click="handleSelect(item)"
      >
        {{ item[0] }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

export default defineComponent({
  name: "HCSelect",
  props: {
    placeholder: {
      type: String,
      default: "Select",
    },
    required: {
      type: Boolean,
      default: false,
    },
    label: {
      type: String,
      default: undefined,
    },
    items: {
      type: Object as PropType<[string, any][]>,
      required: true,
    },
    helper: {
      type: String,
      default: undefined,
    },
  },
  data(): {
    focus: boolean;
    selectedKey: string | undefined;
    showSelection: boolean;
    value: any;
  } {
    return {
      focus: false,
      selectedKey: undefined,
      showSelection: false,
      value: undefined,
    };
  },
  methods: {
    handleSelect(item: [string, any]) {
      this.value = item[1];
      this.selectedKey = item[0];
      this.$emit("item-selected", item[1]);
    },
  },
});
</script>

<style scoped>
.container {
  --hc-primary-color: #482edf;
  --active-border-color: rgba(59, 61, 115, 0.25);
  --active-label-color: rgba(59, 61, 115, 0.5);
}

.container:focus-within {
  --active-border-color: var(--hc-primary-color);
  --active-label-color: var(--hc-primary-color);
}

.input-field {
  cursor: pointer;
  height: 48px;
  padding: 0 10px;
  width: 300px;
  /* outline: 2px solid rgba(59, 61, 115, 0.25); */
  border-radius: 10px;
  /* box-shadow: 0 0 0 2px rgba(59, 61, 115, 0.25); */
  border: 2px solid var(--active-border-color);
}

.flatBottom {
  border-radius: 5px 5px 0 0;
}

.input-field:focus {
  border: 2px solid var(--hc-primary-color);
}

.items-list {
  background-color: white;
  width: 100%;
  position: absolute;
  top: 53px;
  box-shadow: 0 0 2px rgb(131, 128, 176);
  border-radius: 8px 8px 10px 10px;
}

.item {
  align-items: center;
  cursor: pointer;
  padding: 12px;
  border-radius: 10px;
}

.item:hover {
  background: #e8e8eb;
}

.greyedOut {
  color: var(--active-label-color);
}

.rotated {
  transform: rotate(180deg);
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

.helper-note {
  margin-left: 15px;
  margin-top: -1px;
  font-size: 13px;
  color: rgba(59, 61, 115, 0.5);
}
</style>
