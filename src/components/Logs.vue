<template>
  <div class="column" style="background-color: lightgrey; padding: 16px">
    <span class="title" style="margin-bottom: 8px">Logs</span>
    <div class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <div class="column">
            <span
              v-for="([date, log], i) in orderedLogs"
              :key="i"
              style="margin-bottom: 8px"
              ref="logs"
              >{{ new Date(parseInt(date)).toLocaleTimeString() }} -
              {{ log }}</span
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
  computed: {
    orderedLogs() {
      return Object.entries(this.$store.state.logs).sort(
        ([dateA, _], [dateB, __]) => parseInt(dateA) - parseInt(dateB)
      );
    },
  },
});
</script>
