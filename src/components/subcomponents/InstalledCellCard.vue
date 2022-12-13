<template>
  <div class="column card">
    <div style="text-align: right; font-weight: 600">{{ cell.role_name }}</div>
    <div style="margin-bottom: 15px">
      <span style="margin-right: 20px">Dna Hash:</span>
      <span style="opacity: 0.7; font-family: monospace; font-size: 14px"
        >{{ dnaHashForCell(cell) }}
      </span>
    </div>
    <div>
      <div style="margin-bottom: 10px" title="Historical Gossip Throughput">
        Active Peer Synchronizations:
      </div>
      <div class="column">
        <!-- active incoming gossip rounds -->
        <div class="row" style="align-items: center">
          <div
            style="
              width: 10%;
              margin-left: 20px;
              font-size: 0.95em;
              text-align: right;
            "
          >
            incoming:
          </div>
          <div style="width: 60%; margin: 0 30px">
            <HCProgressBar
              v-if="expectedIncoming"
              title="currently ongoing data exchanges with peers"
              :progress="progressRatio()"
              :style="`--height: 10px; --hc-primary-color:${
                incomingIdle ? '#D0D0D0' : '#482edf'
              };`"
            />
            <div
              v-else
              style="text-align: center; opacity: 0.7; font-size: 0.9em"
            >
              <span title="currently ongoing data exchanges with peers"
                >no ongoing peer synchronization</span
              >
            </div>
          </div>
          <div
            :style="`width: 25%; text-align: center; ${
              incomingIdle ? 'opacity: 0.7;' : ''
            }`"
            title="max expected bytes | remaining expected bytes"
          >
            <span :class="{ highlighted: maxExceeded }">{{
              cachedMaxExpected ? prettyBytes(cachedMaxExpected) : "-"
            }}</span>
            | {{ expectedIncoming ? prettyBytes(expectedIncoming) : "-" }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import {
  AppWebsocket,
  DnaHash,
  DnaGossipInfo,
  InstalledCell,
} from "@holochain/client";
import prettyBytes from "pretty-bytes";

import HCProgressBar from "./HCProgressBar.vue";
import { HolochainId } from "../../types";
import { serializeHash } from "@holochain-open-dev/utils";

export default defineComponent({
  name: "InstalledCellCard",
  components: { HCProgressBar },
  props: {
    cell: {
      type: Object as PropType<InstalledCell>,
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
  async created() {
    // set up polling loop to periodically get gossip progress, global scope (window) seems to
    // be required to clear it again on beforeUnmount()
    await this.getGossipInfo();
    this.pollInterval = window.setInterval(
      async () => await this.getGossipInfo(),
      2000
    );
  },
  beforeUnmount() {
    window.clearInterval(this.pollInterval!);
  },
  methods: {
    prettyBytes,
    serializeHash,
    async getGossipInfo() {
      const port = this.$store.getters["appInterfacePort"](this.holochainId);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      const gossipInfo: DnaGossipInfo[] = await appWs.gossipInfo({
        dnas: [this.cell.cell_id[0]],
      });

      const expectedIncoming =
        gossipInfo[0].total_historical_gossip_throughput.expected_op_bytes
          .incoming;

      // In case expected incoming bytes are 0, keep the chached values, otherwise update
      // expectedIncoming
      if (expectedIncoming != 0) {
        this.incomingIdle = false;
        this.latestIncomingUpdate = Date.now();
        // if the expected incoming bytes are larger then the max cached value or there
        // is no cached max value, replace it
        if (
          !this.cachedMaxExpected ||
          expectedIncoming > this.cachedMaxExpected
        ) {
          this.cachedMaxExpected = expectedIncoming;
          this.maxExceeded = true;
          setTimeout(() => (this.maxExceeded = false), 500);
        }
        // make this call after setting max cached value to ensure it is always <= to it
        this.expectedIncoming = expectedIncoming;
      }

      // If expected incoming is zero, set the progress bar to idle state
      if (expectedIncoming == 0) {
        this.incomingIdle = true;
      }

      // if latest non-zero update to gossip progress is older than 30 seconds, set expected incoming
      // and max cached expected incoming to undefined again
      if (new Date().getTime() - this.latestIncomingUpdate > 30000) {
        this.expectedIncoming = undefined;
        this.cachedMaxExpected = undefined;
      }

      console.log("========================================");
      console.log(
        "expectedIncoming: ",
        this.expectedIncoming ? prettyBytes(this.expectedIncoming!) : undefined
      );
      console.log(
        "cachedMaxExpected: ",
        this.cachedMaxExpected
          ? prettyBytes(this.cachedMaxExpected!)
          : undefined
      );
      console.log("incomingIdle: ", this.incomingIdle);
      console.log("progressRatio(): ", this.progressRatio());
      console.log("========================================");
    },
    progressRatio() {
      if (this.expectedIncoming && this.cachedMaxExpected) {
        return (1 - this.expectedIncoming / this.cachedMaxExpected) * 100;
      } else {
        return undefined;
      }
    },
    dnaHashForCell(cell: InstalledCell) {
      return serializeHash(new Uint8Array(cell.cell_id[0]))
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
}

.highglighted {
  font-weight: bold;
  color: #482edf;
}
</style>
