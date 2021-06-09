<template>
  <div class="column" style="background-color: lightgrey; padding: 16px">
    <span class="title" style="margin-bottom: 8px">Logs</span>
    <div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <div class="column">
            <span
              v-for="(log, i) in $store.state.logs"
              :key="i"
              style="margin-bottom: 8px"
              ref="logs"
              >{{ log }}</span
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Logs",
  updated() {
    let logs = this.$refs.logs as HTMLElement;

    if (logs) {
      logs.scrollIntoView(false);
    }
  },
});
</script>
