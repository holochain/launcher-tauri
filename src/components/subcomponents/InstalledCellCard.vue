<template>
  <div class="column card">
    <div style="text-align: right; font-weight: 600">{{ roleName }}</div>
    <div>
      <span style="margin-right: 78px;">{{ $t('main.cloneName') }}:</span>
      <span style="font-size: 14px;"
        >{{ getCellName(cellInfo) }}
      </span>
    </div>
    <div style="display: flex; flex-direction: row; align-items: center; min-width: 500px; position: relative;">
      <span style="margin-right: 50px">Dna Hash:</span>
      <span style="opacity: 0.7; font-family: monospace; font-size: 15px"
        >{{ dnaHashForCell(cellInfo) }}
      </span>
      <div
          v-show="showDnaHashCopiedTooltip"
          style="background: black; color: white; position: absolute; bottom: 35px; right: -10px; border-radius: 5px; padding: 2px 5px;"
        >{{ $t('main.copied') }}!</div>
      <img
        src="/img/copy_icon.svg"
        :title="$t('main.copyDnaHash')"
        style="height: 23px; margin-left: 53px; margin-bottom: 5px; cursor: pointer; opacity: 0.9;"
        @click="copyDnaHash"
      />

    </div>
    <div style="margin-bottom: 15px; display: flex; flex-direction: row; align-items: center; position: relative;">
      <span style="margin-right: 20px">{{ $t('main.networkSeed') }}:</span>
      <div
        v-if="networkSeedVisible"
        style="opacity: 0.7; font-family: monospace; font-size: 15px; min-width: 500px;"
        >{{ getCellNetworkSeed(cellInfo) }}
      </div>
      <div
        v-else
        style="opacity: 0.8; font-size: 15px; min-width: 500px;"
        >•••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••
      </div>
      <span>
        <svg
          v-if="!networkSeedVisible"
          @click="networkSeedVisible = true"
          :aria-label="$t('main.showNetworkSeed')"
          width="23"
          height="15"
          viewBox="0 0 26 18"
          fill="none"
          style="cursor: pointer; fill: black; opacity: 0.8;"
          xmlns="http://www.w3.org/2000/svg"
        >
          <title>{{ $t('main.showNetworkSeed') }}</title>
          <path
            d="M13.3079 2.38045C17.6075 2.38045 21.4419 4.79681 23.3137 8.61987C21.4419 12.4429 17.6075 14.8593 13.3079 14.8593C9.00841 14.8593 5.17401 12.4429 3.30218 8.61987C5.17401 4.79681 9.00841 2.38045 13.3079 2.38045ZM13.3079 0.111572C7.63574 0.111572 2.79168 3.63968 0.829102 8.61987C2.79168 13.6001 7.63574 17.1282 13.3079 17.1282C18.9801 17.1282 23.8242 13.6001 25.7868 8.61987C23.8242 3.63968 18.9801 0.111572 13.3079 0.111572ZM13.3079 5.78377C14.8735 5.78377 16.144 7.05434 16.144 8.61987C16.144 10.1854 14.8735 11.456 13.3079 11.456C11.7424 11.456 10.4718 10.1854 10.4718 8.61987C10.4718 7.05434 11.7424 5.78377 13.3079 5.78377ZM13.3079 3.51489C10.4945 3.51489 8.20296 5.80646 8.20296 8.61987C8.20296 11.4333 10.4945 13.7248 13.3079 13.7248C16.1213 13.7248 18.4129 11.4333 18.4129 8.61987C18.4129 5.80646 16.1213 3.51489 13.3079 3.51489Z"
          />
        </svg>
        <svg
          v-else
          @click="networkSeedVisible = false"
          :aria-label="$t('main.hideNetworkSeed')"
          width="23"
          height="15"
          viewBox="0 0 26 18"
          fill="none"
          style="cursor: pointer; fill: black; opacity: 0.8;"
          version="1.1"
          xmlns="http://www.w3.org/2000/svg"
          xmlns:svg="http://www.w3.org/2000/svg"
        >
          <title>{{ $t('main.hideNetworkSeed') }}</title>
          <path
            d="M 13.308594 0.11132812 C 10.517021 0.11132812 7.9268891 0.96804285 5.78125 2.4296875 L 2.2285156 0.171875 L 0.91796875 2.234375 L 3.8066406 4.0703125 C 2.5276205 5.3576952 1.5072856 6.9006754 0.83007812 8.6191406 C 2.7926561 13.599371 7.6364338 17.128906 13.308594 17.128906 C 16.073791 17.128906 18.639748 16.287996 20.773438 14.851562 L 24.345703 17.123047 L 25.65625 15.058594 L 22.759766 13.21875 C 24.061669 11.921674 25.101299 10.359432 25.787109 8.6191406 C 23.824509 3.6389506 18.980794 0.11132813 13.308594 0.11132812 z M 13.308594 2.3808594 C 17.608194 2.3808594 21.442653 4.7960806 23.314453 8.6191406 C 22.681794 9.9113054 21.820132 11.038917 20.796875 11.970703 L 18.130859 10.277344 C 18.310763 9.7564004 18.412109 9.2001985 18.412109 8.6191406 C 18.412109 5.8057306 16.121994 3.515625 13.308594 3.515625 C 11.930689 3.515625 10.679412 4.0655827 9.7597656 4.9570312 L 7.8984375 3.7753906 C 9.5146174 2.8796069 11.363302 2.3808594 13.308594 2.3808594 z M 5.7675781 5.3164062 L 8.4609375 7.0273438 C 8.2955918 7.528936 8.203125 8.0631236 8.203125 8.6191406 C 8.203125 11.432571 10.495194 13.724609 13.308594 13.724609 C 14.660321 13.724609 15.888203 13.191764 16.802734 12.330078 L 18.650391 13.503906 C 17.050295 14.376205 15.226866 14.859375 13.308594 14.859375 C 9.0091037 14.859375 5.1745644 12.442171 3.3027344 8.6191406 C 3.9247894 7.348644 4.7669924 6.2391358 5.7675781 5.3164062 z M 13.308594 5.7832031 C 14.874194 5.7832031 16.144531 7.0536106 16.144531 8.6191406 C 16.144531 8.7469117 16.13163 8.8727822 16.115234 8.9960938 L 11.773438 6.2363281 C 12.216095 5.9505299 12.742875 5.7832031 13.308594 5.7832031 z M 10.488281 8.3164062 L 14.777344 11.041016 C 14.348278 11.302095 13.847251 11.455078 13.308594 11.455078 C 11.743094 11.455078 10.472656 10.184671 10.472656 8.6191406 C 10.472656 8.5165457 10.477642 8.4161931 10.488281 8.3164062 z "
          />
        </svg>

        <div
          v-show="showNetworkSeedCopiedTooltip"
          style="background: black; color: white; position: absolute; bottom: 35px; right: -10px; border-radius: 5px; padding: 2px 5px;"
        >{{ $t('main.copied') }}!</div>
        <img
          src="/img/copy_icon.svg"
          :title="$t('main.copyNetworkSeed')"
          style="height: 23px; margin-left: 5px; cursor: pointer; margin-bottom: -2px; opacity: 0.9;"
          @click="copyNetworkSeed"
        />
      </span>
    </div>
    <div>
      <div style="margin-bottom: 10px" title="Historical Gossip Throughput">
        {{ $t('main.activePeerSynchronization') }}:
      </div>
      <div class="column">
        <!-- active incoming gossip rounds -->
        <div class="row" style="align-items: flex-start;">
          <div
            style="
              margin-left: 35px;
              font-size: 0.95em;
              text-align: right;
            "
          >
            {{ $t('main.incoming') }}:
          </div>
          <div class="row" style="width: 100%; margin: 0 20px;">
            <div class="column" v-if="networkInfo?.fetch_pool_info.op_bytes_to_fetch" style="align-items: flex-end; margin-top: 9px;">
              <span class="loader"></span>
              <span style="display: flex; flex: 1;"></span>
              <span style="font-size: 0.95em;">{{ networkInfo?.fetch_pool_info ? `${prettyBytes(networkInfo?.fetch_pool_info.op_bytes_to_fetch)} in queue` : "" }}</span>
            </div>
            <div
              v-else
              style="text-align: center; opacity: 0.7; font-size: 0.9em"
            >
              <span style="margin-left: 140px;" :title="$t('appStore.activeDataExchanges')"
                >{{ $t('appStore.noOngoingPeerSynchronization') }}</span
              >
            </div>
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
  NetworkInfoRequest,
} from "@holochain/client";
import prettyBytes from "pretty-bytes";

import HCProgressBar from "./HCProgressBar.vue";
import { HolochainId } from "../../types";
import { getCellId, getCellName, getCellNetworkSeed } from "../../utils";
import { writeText } from "@tauri-apps/api/clipboard";

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
    networkInfo: NetworkInfo | undefined;
    appWebsocket: AppWebsocket | undefined;
    networkSeedVisible: boolean;
    showNetworkSeedCopiedTooltip: boolean;
    showDnaHashCopiedTooltip: boolean;
  } {
    return {
      pollInterval: null,
      networkInfo: undefined,
      appWebsocket: undefined,
      networkSeedVisible: false,
      showNetworkSeedCopiedTooltip: false,
      showDnaHashCopiedTooltip: false,
    };
  },
  async created() {
    const port = this.$store.getters["appInterfacePort"](this.holochainId);
    this.appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
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
    getCellNetworkSeed,
    writeText,
    async connectAppWebsocket() {
      const port = this.$store.getters["appInterfacePort"](this.holochainId);
      this. appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      // console.log("Connected to AppWebsocket.");
    },
    async getNetworkInfo() {

      // console.log("========================================");
      // console.log("@getNetworkInfo: getting network info...")

      if (!this.appWebsocket) {
        await this.connectAppWebsocket();
      }

      // console.log("@getNetworkInfo: connected to app websocket: ", this.appWebsocket);

      let networkInfos: NetworkInfo[] = [];

      try {
        networkInfos = await this.appWebsocket!.networkInfo({
          agent_pub_key: getCellId(this.cellInfo)![1],
          dnas: [getCellId(this.cellInfo)![0]],
          last_time_queried: undefined,
        } as NetworkInfoRequest);

        this.networkInfo = networkInfos[0]; // only one network info expected per cell
      } catch(e) {
        console.log(`Failed to get network info: ${JSON.stringify(e)}`);
        console.error(`Failed to get network info: ${JSON.stringify(e)}`);
      }

    },
    dnaHashForCell(cell: CellInfo) {
      return encodeHashToBase64(new Uint8Array(getCellId(cell)![0]))
    },
    copyNetworkSeed() {
      const networkSeed = getCellNetworkSeed(this.cellInfo);
      if (networkSeed) {
        writeText(networkSeed);
        this.showNetworkSeedCopiedTooltip = true;
        setTimeout(() => this.showNetworkSeedCopiedTooltip = false, 1500);
      } else {
        writeText("")
      }
    },
    copyDnaHash() {
      const dnaHash = this.dnaHashForCell(this.cellInfo);
      writeText(dnaHash);
      this.showDnaHashCopiedTooltip = true;
      setTimeout(() => this.showDnaHashCopiedTooltip = false, 1500);
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

.loader {
  width: 475px;
  height: 6px;
  display: inline-block;
  position: relative;
  background: rgba(0, 0, 0, 0.15);
  overflow: hidden;
}
.loader::after {
  content: '';
  width: 192px;
  height: 6px;
  background: #482edf;
  position: absolute;
  top: 0;
  left: 0;
  box-sizing: border-box;
  animation: animloader 2s linear infinite;
}
@keyframes animloader {
  0% {
    left: -192px;
  }
  100% {
    left: 100%;
    transform: translateX(0%);
  }
}
</style>
