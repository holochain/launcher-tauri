<template>
  <mwc-dialog heading="About" ref="dialog">
    <div class="column">
      <span> Holochain Launcher v0.3.10 </span>
      <span style="margin-top: 8px"> Holochain v0.0.127 </span>
      <span style="margin-top: 8px"> Lair Keystore v0.1.0 </span>
    </div>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { getCurrent } from "@tauri-apps/api/window";
import type { Dialog } from "@material/mwc-dialog";

export default defineComponent({
  name: "About",
  data(): {
    snackbarText: string | undefined;
  } {
    return {
      snackbarText: undefined,
    };
  },
  async mounted() {
    const current = await getCurrent().listen("about", () =>
      (this.$refs.dialog as Dialog).show()
    );
  },
});
</script>
