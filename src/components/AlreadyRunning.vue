<template>
  <mwc-dialog
    heading="Holochain Launcher is already running"
    scrimClickAction=""
    escapeKeyAction=""
    open
  >
    <div class="column">
      <span style="margin-top: 8px">
        Holochain Launcher is already running in the background. Check the
        system tray to see the Holochain Launcher icon, click it, and select
        "Show Admin".
      </span>
      <span style="margin-top: 8px">
        If you can't find the other instance of the Holochain Launcher, restart
        your computer to clear the Holochain background processes.</span
      >
      <span style="margin-top: 8px"> You can safely close this window. </span>
    </div>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "AlreadyRunning",
});
</script>
