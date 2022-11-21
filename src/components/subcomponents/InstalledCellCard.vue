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
        Peer Synchronization Progress:
      </div>
      <div v-if="gossipInfo" class="column">
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
              :progress="gossipProgressIncoming(gossipInfo)"
              style="--height: 10px"
            />
          </div>
          <div
            style="width: 20%; text-align: left"
            title="actual bytes / expected bytes"
          >
            {{ gossipProgressIncomingString(gossipInfo) }}
          </div>
        </div>
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
              :progress="gossipProgressOutgoing(gossipInfo)"
              style="--height: 10px"
            />
          </div>
          <div
            style="width: 20%; text-align: left"
            title="actual bytes / expected bytes"
          >
            {{ gossipProgressOutgoingString(gossipInfo) }}
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
import {
  gossipProgressIncoming,
  gossipProgressIncomingString,
  gossipProgressOutgoing,
  gossipProgressOutgoingString,
} from "@/utils";

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
    gossipInfo: DnaGossipInfo | undefined;
    pollInterval: number | null;
  } {
    return {
      gossipInfo: undefined,
      pollInterval: null,
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
    gossipProgressIncoming,
    gossipProgressOutgoing,
    gossipProgressIncomingString,
    gossipProgressOutgoingString,
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
      this.gossipInfo = gossipInfo[0];
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
