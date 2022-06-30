<template>
  <mwc-dialog heading="About" ref="dialog">
    <div class="row center" style="width: 512px" v-if="!holochainVersions">
      <mwc-circular-progress
        indeterminate
        style="margin-top: 80px; margin-bottom: 60px"
      ></mwc-circular-progress>
    </div>
    <div class="column" style="width: 512px" v-else-if="holochainVersions">
      <div class="column">
        <span> Holochain Launcher v0.4.9 </span>
        <span
          style="margin-top: 8px"
          v-for="version in holochainVersions"
          :key="version"
        >
          Holochain v{{ version }}
        </span>
        <span style="margin-top: 8px"> Lair Keystore v0.1.3 </span>
      </div>
    </div>
  </mwc-dialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { getCurrent } from "@tauri-apps/api/window";
import type { Dialog } from "@material/mwc-dialog";
import { HolochainVersion } from "@/types";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "About",
  data(): {
    snackbarText: string | undefined;
    holochainVersions: HolochainVersion[] | undefined;
  } {
    return {
      snackbarText: undefined,
      holochainVersions: undefined,
    };
  },
  async mounted() {
    const current = await getCurrent().listen("about", () =>
      (this.$refs.dialog as Dialog).show()
    );

    const { holochain_versions }: { holochain_versions: HolochainVersion[] } =
      await invoke("get_supported_versions", {});

    this.holochainVersions = holochain_versions;
  },
});
</script>
