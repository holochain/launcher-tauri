<template>
  <HCGenericDialog
    @confirm="uninstallApp(app)"
    closeOnSideClick
    ref="uninstall-app-dialog"
    primaryButtonLabel="Uninstall"
    ><div style="text-align: center">
      Are you sure you want to uninstall this App? This will irrevocably delete
      all data stored in it.
    </div>
  </HCGenericDialog>

  <div class="container">
    <div
      style="
        position: relative;
        display: flex;
        flex-direction: row;
        align-items: center;
        width: 100%;
        height: 120px;
      "
    >
      <div style="position: relative">
        <!-- assumes same agent pub key for all cells (just taking the first one) -->
        <div v-show="showPubKeyTooltip" class="tooltip">Copied!</div>
        <HoloIdenticon
          title="Your Public Key"
          :class="{ holoIdenticon: !showMore, holoIdenticonMore: showMore }"
          style="position: absolute; top: 78px; left: 78px; cursor: pointer"
          :hash="app.webAppInfo.installed_app_info.cell_data[0].cell_id[1]"
          :size="42"
          @click="copyPubKey()"
        ></HoloIdenticon>

        <img
          v-if="appIcon"
          :class="{ appIcon: !showMore, appIconMore: showMore }"
          :src="`${appIcon}`"
        />
        <div
          v-else
          :class="{ appIcon: !showMore, appIconMore: showMore }"
          class="column center-content"
          style="background-color: #372ba5"
        >
          <div style="color: white; font-size: 45px; font-weight: 600">
            {{ app.webAppInfo.installed_app_info.installed_app_id.slice(0, 2) }}
          </div>
        </div>
      </div>

      <div
        style="
          display: flex;
          flex: 1;
          font-size: 23px;
          font-weight: 700;
          margin-left: 40px;
          margin-right: 30px;
          word-break: break-all;
        "
      >
        {{ app.webAppInfo.installed_app_info.installed_app_id }}
      </div>

      <sl-tooltip
        style="--show-delay: 500"
        hoist
        placement="top"
        :content="getAppStatus(app)"
      >
        <div
          :class="{
            running: isAppRunning(app.webAppInfo.installed_app_info),
            stopped: isAppDisabled(app.webAppInfo.installed_app_info),
            paused: isAppPaused(app.webAppInfo.installed_app_info),
          }"
          class="app-status"
          style="margin-right: 29px"
          tabindex="0"
        ></div>
      </sl-tooltip>

      <div
        v-if="
          isAppRunning(app.webAppInfo.installed_app_info) && !isAppHeadless(app)
        "
        style="display: flex"
      >
        <sl-tooltip class="tooltip" hoist placement="top" content="Open App">
          <img
            tabindex="0"
            style="margin-right: 29px; width: 24px; cursor: pointer"
            src="/img/Open_App.svg"
            @click="$emit('openApp', app)"
            v-on:keyup.enter="$emit('openApp', app)"
          />
        </sl-tooltip>
      </div>

      <sl-tooltip
        class="tooltip"
        hoist
        placement="top"
        :content="
          isAppRunning(app.webAppInfo.installed_app_info)
            ? 'Disable App'
            : 'Start App'
        "
      >
        <ToggleSwitch
          v-if="
            isAppUninstallable(
              app.webAppInfo.installed_app_info.installed_app_id
            )
          "
          style="margin-right: 29px"
          :sliderOn="isAppRunning(app.webAppInfo.installed_app_info)"
          @click="handleSlider(app)"
        />
      </sl-tooltip>

      <sl-tooltip class="tooltip" hoist placement="top" content="App Details">
        <HCMoreToggle
          @toggle="showMore = !showMore"
          style="margin-right: 33px"
          tabindex="0"
        />
      </sl-tooltip>
    </div>

    <div
      v-if="showMore"
      class="column"
      style="align-items: left; width: 100%; margin-bottom: 20px"
    >
      <div class="row" style="margin-top: 45px; margin-left: 140px">
        <span style="margin-right: 10px; font-weight: bold; font-size: 1em"
          >Holochain Version:</span
        >
        <span style="opacity: 0.7; font-family: monospace: font-size: 1em;">{{
          app.holochainId.type === "CustomBinary"
            ? "Custom Binary"
            : app.holochainId.content
        }}</span>
      </div>

      <!-- Add Gossip state here if holochain version is not 0.0.169 or lower -->
      <!-- calls a tauri command to get the gossip state -->

      <table style="text-align: left; margin-top: 20px; margin-left: 140px">
        <tr>
          <th></th>
          <th>Dna Hash / Gossip Info</th>
        </tr>

        <tr>
          <th>Main Cells</th>
          <th></th>
        </tr>

        <tr
          style=""
          v-for="cellData in mainCells"
          :key="
            JSON.stringify([...cellData.cell_id[0], ...cellData.cell_id[1]])
          "
        >
          <td>
            <span
              >{{ cellData.role_id.slice(0, 20)
              }}{{ cellData.role_id.length > 20 ? "..." : "" }}</span
            >
          </td>
          <td>
            <span style="margin-right: 20px">Dna Hash:</span>
            <span
              style="opacity: 0.7; font-family: monospace; font-size: 14px"
              >{{ serializeHash(cellData.cell_id[0]) }}</span
            ><br />
            <span>Peer Synchronization:</span><br />
            <div class="row" style="align-items: center">
              <div>incoming:</div>
              <div style="width: 50%; margin: 0 20px">
                <HCProgressBar
                  :progress="
                    gossipProgressIncoming(gossipInfo[cellData.role_id])
                  "
                  style="--height: 10px"
                />
              </div>
              <div>
                {{ gossipProgressIncomingString(gossipInfo[cellData.role_id]) }}
              </div>
            </div>
            <div class="row" style="align-items: center">
              <div>outgoing:</div>
              <div style="width: 50%; margin: 0 20px">
                <HCProgressBar
                  :progress="
                    gossipProgressOutgoing(gossipInfo[cellData.role_id])
                  "
                  style="--height: 10px"
                />
              </div>
              <div>
                {{ gossipProgressOutgoingString(gossipInfo[cellData.role_id]) }}
              </div>
            </div>
          </td>
        </tr>

        <br />
        <tr v-if="clonedCells.length > 0">
          <th>Cloned Cells</th>
          <th></th>
        </tr>

        <tr
          style=""
          v-for="cellData in clonedCells"
          :key="
            JSON.stringify([...cellData.cell_id[0], ...cellData.cell_id[1]])
          "
        >
          <td>
            <span
              >{{ cellData.role_id.slice(0, 20)
              }}{{ cellData.role_id.length > 20 ? "..." : "" }}</span
            >
          </td>
          <td>
            <span
              style="opacity: 0.7; font-family: monospace; font-size: 14px"
              >{{ serializeHash(cellData.cell_id[0]) }}</span
            >
          </td>
        </tr>
      </table>

      <span
        v-if="getReason(app.webAppInfo.installed_app_info)"
        style="margin-top: 16px; margin-left: 140px"
      >
        {{ getReason(app.webAppInfo.installed_app_info) }}
      </span>

      <div
        style="
          display: flex;
          flex-direction: row;
          justify-content: flex-end;
          margin-top: 40px;
          margin-right: 20px;
        "
      >
        <HCButton
          class="btn"
          style="--hc-primary-color: #d80d0d"
          @click="requestUninstall"
          v-if="
            isAppUninstallable(
              app.webAppInfo.installed_app_info.installed_app_id
            )
          "
          outlined
          >Uninstall
        </HCButton>

        <HCButton
          style="--hc-primary-color: #dd821a"
          v-if="
            !isAppDisabled(app.webAppInfo.installed_app_info) &&
            isAppUninstallable(
              app.webAppInfo.installed_app_info.installed_app_id
            )
          "
          outlined
          @click="disableApp(app)"
          >Disable
        </HCButton>
        <HCButton
          style="--hc-primary-color: #008704"
          v-if="isAppDisabled(app.webAppInfo.installed_app_info)"
          @click="enableApp(app)"
          outlined
          >Enable
        </HCButton>
        <HCButton
          style="--hc-primary-color: #008704"
          v-if="isAppPaused(app.webAppInfo.installed_app_info)"
          @click="startApp(app)"
          outlined
          >Start
        </HCButton>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { HolochainAppInfo } from "../types";
import { serializeHash } from "@holochain-open-dev/utils";
import { isAppRunning, isAppDisabled, isAppPaused, getReason } from "../utils";
import { writeText } from "@tauri-apps/api/clipboard";
import { AppWebsocket, DnaHash, DnaGossipInfo } from "@holochain/client";
import prettyBytes from "pretty-bytes";

import "@shoelace-style/shoelace/dist/components/tooltip/tooltip.js";
import "@shoelace-style/shoelace/dist/themes/light.css";
// import "@holochain-open-dev/utils/dist/holo-identicon";
import HoloIdenticon from "../components/subcomponents/HoloIdenticon.vue";

import ToggleSwitch from "./subcomponents/ToggleSwitch.vue";
import HCButton from "./subcomponents/HCButton.vue";
import HCMoreToggle from "./subcomponents/HCMoreToggle.vue";
import HCGenericDialog from "./subcomponents/HCGenericDialog.vue";
import HCProgressBar from "./subcomponents/HCProgressBar.vue";

export default defineComponent({
  name: "InstalledAppCard",
  components: {
    ToggleSwitch,
    HCButton,
    HCMoreToggle,
    HCGenericDialog,
    HoloIdenticon,
    HCProgressBar,
  },
  props: {
    appIcon: {
      type: String,
    },
    app: {
      type: Object as PropType<HolochainAppInfo>,
      required: true,
    },
  },
  data(): {
    showMore: boolean;
    showUninstallDialog: boolean;
    showPubKeyTooltip: boolean;
    gossipInfo: Record<string, DnaGossipInfo>;
  } {
    return {
      showMore: false,
      showUninstallDialog: false,
      showPubKeyTooltip: false,
      gossipInfo: {},
    };
  },
  emits: ["openApp", "enableApp", "disableApp", "startApp", "uninstallApp"],
  computed: {
    mainCells() {
      const allCells = this.app.webAppInfo.installed_app_info.cell_data;
      return allCells.filter((cell) => !cell.role_id.includes("."));
    },
    clonedCells() {
      const allCells = this.app.webAppInfo.installed_app_info.cell_data;
      return allCells.filter((cell) => cell.role_id.includes("."));
    },
  },
  async mounted() {
    await this.getGossipInfo(this.app);
  },
  methods: {
    serializeHash,
    getReason,
    isAppRunning,
    isAppDisabled,
    isAppPaused,
    writeText,
    isAppHeadless(app: HolochainAppInfo) {
      return app.webAppInfo.web_ui_info.type === "Headless";
    },
    requestUninstall() {
      (this.$refs["uninstall-app-dialog"] as typeof HCGenericDialog).open();
      this.showUninstallDialog = true;
    },
    async enableApp(app: HolochainAppInfo) {
      this.$emit("enableApp", app);
    },
    async disableApp(app: HolochainAppInfo) {
      this.$emit("disableApp", app);
    },
    async startApp(app: HolochainAppInfo) {
      this.$emit("startApp", app);
    },
    async uninstallApp(app: HolochainAppInfo) {
      this.showUninstallDialog = false;
      this.$emit("uninstallApp", app);
    },
    getAppStatus(app: HolochainAppInfo) {
      if (isAppRunning(app.webAppInfo.installed_app_info)) {
        return "Running";
      }
      if (isAppDisabled(app.webAppInfo.installed_app_info)) {
        return "Disabled";
      }
      if (isAppPaused(app.webAppInfo.installed_app_info)) {
        return "Paused";
      }
      return "Unknown State";
    },
    isAppUninstallable(installedAppId: string) {
      const _hdiOfDevhub = this.$store.getters["hdiOfDevhub"];
      const holochainId = this.$store.getters["holochainIdForDevhub"];

      return installedAppId !== `DevHub-${holochainId.content}`;
    },
    async handleSlider(app: HolochainAppInfo) {
      if (isAppRunning(app.webAppInfo.installed_app_info)) {
        await this.disableApp(app);
      } else if (isAppDisabled(app.webAppInfo.installed_app_info)) {
        await this.enableApp(app);
      } else if (isAppPaused(app.webAppInfo.installed_app_info)) {
        await this.startApp(app);
      } else {
        throw new Error("Unknown App state.");
      }
    },
    copyPubKey() {
      const pubKey =
        this.app.webAppInfo.installed_app_info.cell_data[0].cell_id[1];
      this.writeText(serializeHash(pubKey));
      this.showPubKeyTooltip = true;
      setTimeout(() => {
        this.showPubKeyTooltip = false;
      }, 1200);
    },
    async getGossipInfo(app: HolochainAppInfo) {
      const port = this.$store.getters["appInterfacePort"](app.holochainId);
      const appWs = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
      let dnas: DnaHash[] = app.webAppInfo.installed_app_info.cell_data.map(
        (cell) => cell.cell_id[0]
      );
      let roleIds: string[] = app.webAppInfo.installed_app_info.cell_data.map(
        (cell) => cell.role_id
      );
      const gossipInfo: DnaGossipInfo[] = await appWs.gossipInfo({
        dnas: app.webAppInfo.installed_app_info.cell_data.map(
          (cell) => cell.cell_id[0]
        ),
      });

      let output: Record<string, DnaGossipInfo> = {};
      roleIds.forEach((id, idx) => (output[id] = gossipInfo[idx]));
      this.gossipInfo = output;
    },
    gossipProgressIncoming(info: DnaGossipInfo) {
      const incoming_bytes_expected =
        info.total_historical_gossip_throughput.expected_op_bytes.incoming;
      const incoming_bytes_actual =
        info.total_historical_gossip_throughput.op_bytes.incoming;
      return 100 * (incoming_bytes_actual / incoming_bytes_expected);
    },
    gossipProgressOutgoing(info: DnaGossipInfo) {
      const outgoing_bytes_expected =
        info.total_historical_gossip_throughput.expected_op_bytes.outgoing;
      const outgoing_bytes_actual =
        info.total_historical_gossip_throughput.op_bytes.outgoing;
      return 100 * (outgoing_bytes_actual / outgoing_bytes_expected);
    },
    gossipProgressIncomingString(info: DnaGossipInfo) {
      const incoming_bytes_expected =
        info.total_historical_gossip_throughput.expected_op_bytes.incoming;
      const incoming_bytes_actual =
        info.total_historical_gossip_throughput.op_bytes.incoming;
      return `${prettyBytes(incoming_bytes_actual)} / ${prettyBytes(
        incoming_bytes_expected
      )}`;
    },
    gossipProgressOutgoingString(info: DnaGossipInfo) {
      const outgoing_bytes_expected =
        info.total_historical_gossip_throughput.expected_op_bytes.outgoing;
      const outgoing_bytes_actual =
        info.total_historical_gossip_throughput.op_bytes.outgoing;
      return `${prettyBytes(outgoing_bytes_actual)} / ${prettyBytes(
        outgoing_bytes_expected
      )}`;
    },
  },
});
</script>

<style scoped>
.container {
  position: relative;
  display: flex;
  flex: 1;
  flex-direction: column;
  align-items: center;
  background: #ffffff;
  border-radius: 22px;
  width: 100%;
  max-width: 1100px;
  min-width: 900px;
  margin: 8px;
  /* box-shadow: 0 0 2px rgb(131, 128, 176); */
  box-shadow: 0 0px 5px #9b9b9b;
}

.btn {
  width: 80px;
  margin: 5px;
}

.tooltip {
  --show-delay: 1000;
}

.tooltip::part(base) {
  font-family: "Poppins";
}

.appIcon {
  display: flex;
  width: 120px;
  height: 120px;
  padding: 0;
  border-radius: 22px 0 0 22px;
  object-fit: cover;
}

.appIconMore {
  display: flex;
  width: 120px;
  height: 120px;
  padding: 0;
  border-radius: 22px 0 22px 0;
  object-fit: cover;
}

.holoIdenticon {
  border-radius: 12px 0 0 0;
}

.holoIdenticonMore {
  border-radius: 12px 0 22px 0;
}

.app-status {
  height: 10px;
  width: 10px;
  border-radius: 50%;
}

.running {
  background-color: rgb(0, 185, 0);
}

.stopped {
  background-color: rgb(220, 0, 0);
}

.paused {
  background-color: rgb(175, 175, 175);
}

.tooltip {
  position: absolute;
  /* color: #482edf; */
  color: white;
  bottom: 56px;
  left: 62px;
  background: #5537fc;
  border-radius: 5px;
  /* border: 2px solid #482edf; */
  padding: 1px 7px;
}
</style>
