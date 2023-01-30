<template>
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
    <div>
      <div style="margin-bottom: 10px" title="Historical Gossip Throughput">
        Active Peer Synchronization:
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
              v-if="(expectedIncoming || expectedIncoming === 0) && cachedMaxExpected"
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
            title="received bytes | expected bytes"
          >
            {{ ((expectedIncoming || expectedIncoming === 0) && cachedMaxExpected) ? prettyBytes(cachedMaxExpected - expectedIncoming) : "-" }}
            | <span :class="{ highlighted: maxExceeded }">
            {{ cachedMaxExpected ? prettyBytes(cachedMaxExpected) : "-" }}
            </span>
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
  NetworkInfo,
  CellInfo,
  encodeHashToBase64,
} from "@holochain/client";
import prettyBytes from "pretty-bytes";

import HCProgressBar from "./HCProgressBar.vue";
import { HolochainId } from "../../types";
import { getCellId, getCellName } from "../../utils";

export default defineComponent({
  name: "InstalledCellCard",
  components: { HCProgressBar },
  props: {
    cellInfo: {
      type: Object as PropType<CellInfo>,
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
    appWebsocket: AppWebsocket | undefined;
  } {
    return {
      pollInterval: null,
      expectedIncoming: undefined,
      latestIncomingUpdate: 0,
      incomingIdle: true,
      cachedMaxExpected: undefined,
      maxExceeded: false,
      appWebsocket: undefined,
    };
  },
  async created() {
    const port = this.$store.getters["appInterfacePort"](this.holochainId);
    this. appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
    // set up polling loop to periodically get gossip progress, global scope (window) seems to
    // be required to clear it again on beforeUnmount()
    await this.getNetworkInfo();
    this.pollInterval = window.setInterval(
      async () => await this.getNetworkInfo(),
      2000
    );
  },
  beforeUnmount() {
    window.clearInterval(this.pollInterval!);
  },
  methods: {
    prettyBytes,
    encodeHashToBase64,
    getCellName,
    getCellId,
    async connectAppWebsocket() {
      const port = this.$store.getters["appInterfacePort"](this.holochainId);
      this. appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      // console.log("Connected to AppWebsocket.");
    },
    async getNetworkInfo() {

      if (!this.appWebsocket) {
        await this.connectAppWebsocket();
      }

      const networkInfo: NetworkInfo[] = await this.appWebsocket!.networkInfo({
        dnas: [getCellId(this.cellInfo)![0]],
      });

      // console.log("========================================");
      // console.log("Received NetworkInfo: ", networkInfo);

      const expectedIncoming =
        networkInfo[0].fetch_pool_info.op_bytes_to_fetch;

      // console.log("expectedIncoming: ", expectedIncoming);

      // In case expected incoming bytes are undefined, keep the chached values, otherwise update
      // expectedIncoming
      if (expectedIncoming || expectedIncoming === 0) {
        // console.log("expectedIncoming inside: ", expectedIncoming);
        // if the expected incoming bytes are larger then the max cached value or there
        // is no cached max value, replace it
        if (
          (!this.cachedMaxExpected && this.cachedMaxExpected !== 0) ||
          expectedIncoming > this.cachedMaxExpected
        ) {
          this.cachedMaxExpected = expectedIncoming;
          this.maxExceeded = true;
          setTimeout(() => (this.maxExceeded = false), 500);
        }

        if (expectedIncoming !== this.expectedIncoming) {
          this.incomingIdle = false;
          this.latestIncomingUpdate = Date.now();
        }


        // make this call after setting max cached value to ensure it is always <= to it
        this.expectedIncoming = expectedIncoming;
      }

      // if expected incoming remains the same for > 10 seconds, set to idle. Except expectedIncoming
      // is below 16MB, in this case transmission may already be finished.
      if (new Date().getTime() - this.latestIncomingUpdate > 10000) {
        if ((this.expectedIncoming || this.expectedIncoming === 0) && this.expectedIncoming > 16000000) {
          this.incomingIdle = false
        } else {
          this.incomingIdle = true;
        }
      }

      // if latest non-zero update to network info is older than 80 seconds, set expected incoming
      // and max cached expected incoming to undefined again
      if (new Date().getTime() - this.latestIncomingUpdate > 80000) {
        this.expectedIncoming = undefined;
        this.cachedMaxExpected = undefined;
      }

      // console.log("========================================");
      // console.log(
      //   "this.expectedIncoming: ",
      //   this.expectedIncoming
      // );
      // console.log(
      //   "cachedMaxExpected: ",
      //   this.cachedMaxExpected
      // );
      // console.log("incomingIdle: ", this.incomingIdle);
      // console.log("progressRatio(): ", this.progressRatio());
      // console.log("========================================");
    },
    progressRatio() {
      // console.log("@progressRatio: this.expectedIncoming", this.expectedIncoming);
      // console.log("@progressRatio: this.cachedMaxExpected", this.cachedMaxExpected);

      if ((this.expectedIncoming || this.expectedIncoming === 0) && this.cachedMaxExpected) {
        // console.log("@progressRatio: ratio",(1 - this.expectedIncoming / this.cachedMaxExpected) * 100);
        return (1 - this.expectedIncoming / this.cachedMaxExpected) * 100;
      } else {
        // console.log("@progressRatio: NO PROGRESS RATIO CALCULATED!!");
        return undefined;
      }
    },
    dnaHashForCell(cell: CellInfo) {
      return encodeHashToBase64(new Uint8Array(getCellId(cell)![0]))
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

.highlighted {
  font-weight: bold;
  color: #482edf;
}
</style>
