<template>
  <HCGenericDialog
    ref="dialog"
    @confirm="close()"
    :primaryButtonLabel="$t('buttons.close')"
    closeOnSideClick
    hideCancel
  >
    <div
      class="column"
      style="min-height: 200px; min-width: 400px; margin: -30px 20px 20px 20px"
    >
      <div class="row" style="justify-content: center">
        <div style="font-weight: 600; font-size: 25px; margin: 20px 0 10px 0">
          {{ $t("dialogs.networkStats.networkStats") }}
        </div>
      </div>
      <div class="row" style="margin-top: 20px; margin-bottom: 16px">
        <HCSelect
          ref="selectHolochainVersion"
          style="margin: 5px; width: 360px"
          :label="$t('dialogs.networkStats.changeHolochainVersion')"
          :items="holochainVersions"
          @item-selected="changedHolochainVersion($event)"
        >
        </HCSelect>
      </div>

      <div style="background: rgb(236, 236, 236); border-radius: 8px; padding: 10px 20px; font-size: 15px; overflow-x: auto;">
        <pre>{{ networkStats ? networkStats : "Loading network statistics..." }}</pre>
      </div>

    </div>
  </HCGenericDialog>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { getCurrent } from "@tauri-apps/api/window";
import { defineComponent } from "vue";
import { HolochainId } from "../../types";
import { syntaxHighlight } from "../../utils";

import HCGenericDialog from "../subcomponents/HCGenericDialog.vue";
import HCSelect from "../subcomponents/HCSelect.vue";

export default defineComponent({
  name: "NetworkStats",
  components: { HCGenericDialog, HCSelect },
  data(): {
    holochainVersions: [string, HolochainId][];
    selectedHolochainVersion: HolochainId | undefined;
    networkStats: string | undefined;
  } {
    return {
      holochainVersions: [],
      selectedHolochainVersion: undefined,
      networkStats: undefined,
    };
  },
  mounted() {
    let holochainVersions: [string, HolochainId][] = [];

    this.$store.getters["runningHolochainIds"].forEach(async (id: HolochainId) => {
      holochainVersions.push([id.type == "HolochainVersion" ? id.content : id.type, id]);
    })
    this.holochainVersions = holochainVersions;

    this.$nextTick(async () => {
      const dialog = this.$refs.dialog as typeof HCGenericDialog;
      await getCurrent().listen("open-network-stats", async () => {
        this.open();
      });
    });
  },
  methods: {
    syntaxHighlight,
    async open() {
      let holochainVersions: [string, HolochainId][] = [];
      this.$store.getters["runningHolochainIds"].forEach(async (id: HolochainId) => {
        holochainVersions.push([id.type == "HolochainVersion" ? id.content : id.type, id]);
      })

      this.holochainVersions = holochainVersions;

      (this.$refs.dialog as typeof HCGenericDialog).open();

      if (holochainVersions.length > 0) {
        this.selectedHolochainVersion = holochainVersions[0][1];
        (this.$refs.selectHolochainVersion as typeof HCSelect)
          .select([this.selectedHolochainVersion.type == "HolochainVersion" ? this.selectedHolochainVersion.content : this.selectedHolochainVersion.type, this.selectedHolochainVersion]);
        this.networkStats = await invoke('dump_network_stats', { holochainId: holochainVersions[0][1] });
      };
    },
    close() {
      (this.$refs.dialog as typeof HCGenericDialog).close();
    },
    async changedHolochainVersion(holochainId: HolochainId) {
      this.selectedHolochainVersion = holochainId;
      this.networkStats = await invoke('dump_network_stats', { holochainId });
    }
  },
});
</script>
