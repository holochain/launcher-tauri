<template>
  <sl-badge type="success" v-if="isAppRunning(installedAppInfo)"
    >Running</sl-badge
  >
  <sl-badge type="warning" v-if="isAppPaused(installedAppInfo)"
    >Paused</sl-badge
  >
  <sl-badge type="danger" v-if="isAppDisabled(installedAppInfo)"
    >Disabled</sl-badge
  >
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { isAppRunning, isAppDisabled, isAppPaused } from "../utils";
import { deserializeHash, serializeHash } from "@holochain-open-dev/core-types";
import "@shoelace-style/shoelace/dist/themes/light.css";
import "@shoelace-style/shoelace/dist/components/badge/badge.js";
import { InstalledAppInfo } from "@holochain/client";

export default defineComponent({
  name: "InstalledAppStatus",
  props: {
    installedAppInfo: {
      type: Object as PropType<InstalledAppInfo>,
      required: true,
    },
  },
  methods: {
    deserializeHash,
    serializeHash,
    isAppRunning,
    isAppDisabled,
    isAppPaused,
  },
});
</script>
