<template>
  <HCGenericDialog
    @confirm="deleteCell()"
    closeOnSideClick
    ref="delete-cell-dialog"
    primaryButtonLabel="Delete"
    ><div style="text-align: center; margin: 0 10px;">
      Are you sure you want to delete this cell? This will irrevocably delete
      all data stored in it.
    </div>
  </HCGenericDialog>

  <div class="column card">
    <div style="text-align: right; font-weight: 600">{{ roleName }}</div>
    <div>
      <span style="margin-right: 47px;">Name:</span>
      <span style="font-size: 14px;"
        >{{ getCellName(cellInfo) }}
      </span>
    </div>
    <div style="margin-bottom: 15px">
      <span style="margin-right: 20px">Dna Hash:</span>
      <span style="opacity: 0.7; font-family: monospace; font-size: 14px"
        >{{ dnaHashForCell(cellInfo) }}
      </span>
    </div>
    <div style="display: flex; flex-direction: row; align-items: center;">
      <span style="display: flex; flex: 1;"></span>
      <div style="display: flex; flex-direction: row; align-items: center;">
        <img src="/img/trashcan.svg" style="height: 22px;">
        <span @click="openDeleteDialog()" style="cursor: pointer; color: #d80d0dff; margin-left: 7px;">Delete Cell</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import {
  AppWebsocket,
  NetworkInfo,
  CellInfo,
  encodeHashToBase64,
  ClonedCell,
  CellType,
} from "@holochain/client";
import prettyBytes from "pretty-bytes";

import HCProgressBar from "./HCProgressBar.vue";
import { HolochainId } from "../../types";
import { getCellId, getCellName } from "../../utils";
import { invoke } from "@tauri-apps/api/tauri";
import { ActionTypes } from "../../store/actions";
import HCGenericDialog from "./HCGenericDialog.vue";

export default defineComponent({
  name: "DisabledCloneCard",
  components: { HCProgressBar, HCGenericDialog },
  props: {
    cellInfo: {
      type: Object as PropType<CellInfo>,
      required: true,
    },
    appId: {
      type: String,
      required: true,
    },
    roleName: {
      type: String,
      required: true,
    },
    holochainId: {
      type: Object as PropType<HolochainId>,
      required: true,
    },
  },
  data(): {
    pollInterval: number | null;
    expectedIncoming: number | undefined;
    latestIncomingUpdate: number;
    incomingIdle: boolean;
    cachedMaxExpected: number | undefined;
    maxExceeded: boolean;
  } {
    return {
      pollInterval: null,
      expectedIncoming: undefined,
      latestIncomingUpdate: 0,
      incomingIdle: true,
      cachedMaxExpected: undefined,
      maxExceeded: false,
    };
  },
  beforeUnmount() {
    window.clearInterval(this.pollInterval!);
  },
  methods: {
    prettyBytes,
    encodeHashToBase64,
    getCellName,
    getCellId,
    dnaHashForCell(cell: CellInfo) {
      return encodeHashToBase64(new Uint8Array(getCellId(cell)![0]))
    },
    openDeleteDialog() {
      (this.$refs["delete-cell-dialog"] as typeof HCGenericDialog).open();
    },
    async deleteCell() {
      const clonedCell = (this.cellInfo as { [CellType.Cloned]: ClonedCell }).cloned;

      await invoke("delete_clone", { holochainId: this.holochainId, appId: this.appId, cellId: clonedCell.cell_id })

      await this.$store.dispatch(ActionTypes.fetchStateInfo);

    }
  },
});
</script>

<style scoped>
.card {
  min-width: 630px;
  background: #f6f6fa;
  border-radius: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
  /* border: 2px solid #e1e1e1; */
  padding: 9px 20px 20px 25px;
  margin: 12px 0;
}

.highlighted {
  font-weight: bold;
  color: #482edf;
}
</style>
