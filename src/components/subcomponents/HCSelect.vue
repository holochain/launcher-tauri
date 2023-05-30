<template>
  <div
    class="container column"
    style="position: relative"
    @click="showSelection = !showSelection"
    @keydown.enter="handleEnter"
    @keydown.esc="showSelection = false"
    @keydown.down.prevent="handleDownKey"
    @keydown.up.prevent="handleUpKey"
    tabindex="0"
    @blur="showSelection = false"
  >
    <div class="label">{{ label }}</div>
    <div class="input-field row" style="align-items: center">
      <input
        type="text"
        disabled
        class="selected-field"
        :class="{ greyedOut: !selectedKey }"
        :value="selectedKey ? selectedKey : placeholder"
        :title="selectedKey ? selectedKey : undefined"
      />
      <span style="display: flex; flex: 1"></span>
      <img
        src="/img/select_arrow.svg"
        style="width: 12px; margin-right: 5px; margin-left: 5px"
        :class="{ rotated: showSelection }"
      />
    </div>
    <div v-if="invalid" class="invalidity-note">{{ invalid }}</div>
    <div v-if="helper && !invalid" class="helper-note">{{ helper }}</div>
    <div v-if="showSelection" class="items-list" ref="items-list">
      <div
        class="item row"
        :class="{ selected: selectedIndex === index }"
        v-for="(item, index) of items"
        :title="item[0]"
        :key="index"
        @click="handleSelect(item)"
        @mouseover="selectedIndex = index"
        @mouseleave="selectedIndex = undefined"
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
      default: "Select...",
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
    invalid: {
      type: String,
      default: undefined,
    },
  },
  data(): {
    focus: boolean;
    selectedKey: string | undefined;
    showSelection: boolean;
    value: any;
    selectedIndex: number | undefined;
  } {
    return {
      focus: false,
      selectedKey: undefined,
      showSelection: false,
      value: undefined,
      selectedIndex: undefined,
    };
  },
  methods: {
    handleSelect(item: [string, any]) {
      this.value = item[1];
      this.selectedKey = item[0];
      this.$emit("item-selected", item[1]);
      this.selectedIndex = undefined;
    },
    handleEnter() {
      if (this.selectedIndex === undefined) {
        this.showSelection = !this.showSelection;
      } else {
        this.value = this.items[this.selectedIndex][1];
        this.selectedKey = this.items[this.selectedIndex][0];
        this.$emit("item-selected", this.items[this.selectedIndex][1]);
        this.showSelection = false;
        this.selectedIndex = undefined;
      }
    },
    handleDownKey() {
      if (this.selectedIndex === undefined) {
        this.selectedIndex = 0;
        return;
      } else if (this.selectedIndex > this.items.length - 2) {
        this.selectedIndex = 0;
        return;
      }

      this.selectedIndex += 1;
    },
    handleUpKey() {
      if (this.selectedIndex === undefined) {
        this.selectedIndex = this.items.length - 1;
        return;
      } else if (this.selectedIndex === 0) {
        this.selectedIndex = this.items.length - 1;
        return;
      }

      this.selectedIndex -= 1;
    },
    select(value: [string, any]) {
      this.handleSelect(value);
    },
  },
});
</script>

<style scoped>
.container {
  --hc-primary-color: #482edf;
  --active-border-color: rgba(59, 61, 115, 0.8);
  --active-label-color: rgba(59, 61, 115, 1);
  --active-shadow-color: transparent;
  --hc-label-background: white;
  width: 300px;
  border-radius: 10px;
}

.container:focus-within {
  --active-border-color: var(--hc-primary-color);
  --active-label-color: var(--hc-primary-color);
  --active-shadow-color: var(--hc-primary-color);
}

.input-field {
  cursor: pointer;
  height: 40px;
  padding: 0 10px;
  /* outline: 2px solid rgba(59, 61, 115, 0.25); */
  border-radius: 10px;
  /* box-shadow: 0 0 0 2px rgba(59, 61, 115, 0.25); */
  border: 2px solid var(--active-border-color);
  box-shadow: 0 0 3px var(--active-shadow-color);
}

.selected-field {
  all: unset;
  width: 100%;
  color: black;
}

.flatBottom {
  border-radius: 5px 5px 0 0;
}

.items-list {
  background-color: white;
  width: 100%;
  position: absolute;
  top: 53px;
  box-shadow: 0 0 2px rgb(131, 128, 176);
  border-radius: 8px 8px 10px 10px;
  overflow: hidden;
  z-index: 1;
}

.item {
  align-items: center;
  cursor: pointer;
  padding: 12px;
  border-radius: 10px;
  overflow: hidden;
}

/* .item:hover {
  background: #e8e8eb;
} */

.selected {
  background: #e8e8eb;
}

.greyedOut {
  color: rgb(75, 75, 75);
}

.rotated {
  transform: rotate(180deg);
}

.label {
  color: var(--active-label-color);
  padding: 0 4px;
  font-size: 14px;
  background: var(--hc-label-background);
  position: absolute;
  top: -10px;
  left: 12px;
  margin: 0 10px;
}

.helper-note {
  margin-left: 10px;
  margin-top: -1px;
  font-size: 13px;
  color: rgba(59, 61, 115, 0.5);
}

.invalidity-note {
  margin-top: -1px;
  margin-left: 15px;
  font-size: 13px;
  color: #e00000;
}
</style>
