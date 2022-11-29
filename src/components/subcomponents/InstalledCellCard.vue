<template>
  <div class="column card">
    <div style="text-align: right; font-weight: 600">{{ cell.role_id }}</div>
    <div style="margin-bottom: 15px">
      <span style="margin-right: 20px">Dna Hash:</span>
      <span style="opacity: 0.7; font-family: monospace; font-size: 14px"
        >{{ serializeHash(cell.cell_id[0]) }}
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
              v-if="gossipProgressIncoming"
              title="currently ongoing data exchanges with peers"
              :progress="gossipProgressPercent(gossipProgressIncoming)"
              :style="`--height: 10px; --hc-primary-color:${
                incomingIdle ? '#D0D0D0' : '#482edf'
              };`"
            />
            <div
              v-else
              style="text-align: center; opacity: 0.7; font-size: 0.9em"
              title="currently ongoing data exchanges with peers"
            >
              no ongoing peer synchronization
            </div>
          </div>
          <div
            :style="`width: 25%; text-align: center; ${
              incomingIdle ? 'opacity: 0.7;' : ''
            }`"
            title="actual bytes / expected bytes"
          >
            {{ gossipProgressString(gossipProgressIncoming) }}
          </div>
        </div>

        <!-- active outgoing gossip rounds -->
        <div class="row" style="align-items: center">
          <div
            style="
              width: 10%;
              margin-left: 20px;
              font-size: 0.95em;
              text-align: right;
            "
          >
            outgoing:
          </div>
          <div style="width: 60%; margin: 0 30px; align-items: center">
            <HCProgressBar
              v-if="gossipProgressOutgoing"
              title="currently ongoing data exchanges with peers"
              :progress="gossipProgressPercent(gossipProgressOutgoing)"
              :style="`--height: 10px; --hc-primary-color:${
                outgoingIdle ? '#D0D0D0' : '#482edf'
              };`"
            />
            <div
              v-else
              title="currently ongoing data exchanges with peers"
              style="text-align: center; opacity: 0.7; font-size: 0.9em"
            >
              no ongoing peer synchronization
            </div>
          </div>
          <div
            :style="`width: 25%; text-align: center; ${
              outgoingIdle ? 'opacity: 0.7;' : ''
            }`"
            title="actual bytes / expected bytes"
          >
            {{ gossipProgressString(gossipProgressOutgoing) }}
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
import { HolochainId } from "@/types";
import { serializeHash } from "@holochain-open-dev/utils";
import { gossipProgressPercent, gossipProgressString } from "@/utils";

import { GossipProgress } from "../../types";

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
    gossipProgressIncoming: GossipProgress | undefined;
    gossipProgressOutgoing: GossipProgress | undefined;
    latestIncomingUpdate: number;
    latestOutgoingUpdate: number;
    outgoingIdle: boolean;
    incomingIdle: boolean;
  } {
    return {
      pollInterval: null,
      gossipProgressIncoming: undefined,
      gossipProgressOutgoing: undefined,
      latestIncomingUpdate: 0,
      latestOutgoingUpdate: 0,
      incomingIdle: true,
      outgoingIdle: true,
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
    gossipProgressPercent,
    gossipProgressString,
    serializeHash,
    async getGossipInfo() {
      const port = this.$store.getters["appInterfacePort"](this.holochainId);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      const gossipInfo: DnaGossipInfo[] = await appWs.gossipInfo({
        dnas: [this.cell.cell_id[0]],
      });

      const gossipProgressIncoming = {
        expectedBytes:
          gossipInfo[0].total_historical_gossip_throughput.expected_op_bytes
            .incoming,
        actualBytes:
          gossipInfo[0].total_historical_gossip_throughput.op_bytes.incoming,
      };
      const gossipProgressOutgoing: GossipProgress = {
        expectedBytes:
          gossipInfo[0].total_historical_gossip_throughput.expected_op_bytes
            .outgoing,
        actualBytes:
          gossipInfo[0].total_historical_gossip_throughput.op_bytes.outgoing,
      };

      // Check gossip info. In case expected and actual op bytes are 0, keep the chached values
      if (
        gossipProgressIncoming.expectedBytes != 0 ||
        gossipProgressIncoming.actualBytes != 0
      ) {
        this.incomingIdle = false;
        this.gossipProgressIncoming = gossipProgressIncoming;
        this.latestIncomingUpdate = Date.now();
      }

      if (
        gossipProgressOutgoing.expectedBytes != 0 ||
        gossipProgressOutgoing.actualBytes != 0
      ) {
        this.outgoingIdle = false;
        this.gossipProgressOutgoing = gossipProgressOutgoing;
        this.latestOutgoingUpdate = Date.now();
      }

      // If actual/expected are both zero, set the progress bar to idle state
      if (
        gossipProgressIncoming.expectedBytes == 0 &&
        gossipProgressIncoming.actualBytes == 0
      ) {
        this.incomingIdle = true;
      }
      if (
        gossipProgressIncoming.expectedBytes == 0 &&
        gossipProgressIncoming.actualBytes == 0
      ) {
        this.outgoingIdle = true;
      }

      // if latest updates to gorrsip progress are older than 30 seconds, set them to undefined again
      if (new Date().getTime() - this.latestIncomingUpdate > 30000) {
        this.gossipProgressIncoming = undefined;
      }
      if (new Date().getTime() - this.latestOutgoingUpdate > 30000) {
        this.gossipProgressOutgoing = undefined;
      }
    },
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
</style>
