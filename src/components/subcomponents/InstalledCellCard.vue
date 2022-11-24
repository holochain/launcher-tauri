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
          <div style="width: 65%; margin: 0 30px">
            <HCProgressBar
              v-if="gossipProgressIncoming"
              :progress="gossipProgressPercent(gossipProgressIncoming)"
              style="--height: 10px"
            />
            <div v-else style="text-align: center">No active gossip rounds</div>
          </div>
          <div
            style="width: 20%; text-align: left"
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
          <div style="width: 65%; margin: 0 30px; align-items: center">
            <HCProgressBar
              v-if="gossipProgressOutgoing"
              :progress="gossipProgressPercent(gossipProgressOutgoing)"
              style="--height: 10px"
            />
            <div v-else>No active gossip rounds</div>
          </div>
          <div
            style="width: 20%; text-align: left"
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
  } {
    return {
      pollInterval: null,
      gossipProgressIncoming: undefined,
      gossipProgressOutgoing: undefined,
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
    clearInterval(this.pollInterval!);
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
      console.log(
        "Gossip Info fetched for ",
        this.cell.role_id,
        ": ",
        gossipInfo
      );

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
        this.gossipProgressIncoming = gossipProgressIncoming;
      }

      if (
        gossipProgressOutgoing.expectedBytes != 0 ||
        gossipProgressOutgoing.actualBytes != 0
      ) {
        this.gossipProgressOutgoing = gossipProgressOutgoing;
      }
    },
  },
});
</script>

<style scoped>
.card {
  min-width: 630px;
  background: white;
  border-radius: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
  padding: 9px 20px 20px 25px;
}
</style>
