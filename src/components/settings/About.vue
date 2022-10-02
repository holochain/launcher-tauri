<template>
  <HCDialog heading="About" ref="dialog" closeOnSideClick>
    <div class="row center" style="width: 512px" v-if="!holochainVersions">
      <mwc-circular-progress
        indeterminate
        style="margin-top: 80px; margin-bottom: 60px"
      ></mwc-circular-progress>
    </div>
    <div
      class="column"
      style="width: 312px; margin: 20px 30px"
      v-else-if="holochainVersions"
    >
      <span
        style="
          font-weight: 600;
          font-size: 1.5em;
          width: 100%;
          text-align: center;
          margin-bottom: 25px;
          margin-top: -10px;
        "
        >About</span
      >
      <div class="column">
        <span> Holochain Launcher v0.6.1 </span>
        <span
          style="margin-top: 8px"
          v-for="version in holochainVersions"
          :key="version"
        >
          Holochain v{{ version }}
        </span>
        <span style="margin-top: 8px"> Lair Keystore v0.2.1 </span>
      </div>
    </div>
  </HCDialog>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { getCurrent } from "@tauri-apps/api/window";
import type { Dialog } from "@material/mwc-dialog";
import { HolochainVersion } from "@/types";
import { invoke } from "@tauri-apps/api/tauri";
import HCDialog from "../subcomponents/HCDialog.vue";

export default defineComponent({
  name: "About",
  components: {
    HCDialog,
  },
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
      (this.$refs.dialog as typeof HCDialog).open()
    );

    const { holochain_versions }: { holochain_versions: HolochainVersion[] } =
      await invoke("get_supported_versions", {});

    this.holochainVersions = holochain_versions;
  },
});
</script>
