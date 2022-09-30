<template>
  <mwc-dialog
    :heading="`Install App: ${appInfo?.app_name || ''}`"
    open
    @closing="$emit('closing-dialog')"
    scrimClickAction=""
    escapeKeyAction=""
  >
    <div class="row center-content" v-if="isLoadingFile">
      <mwc-circular-progress
        indeterminate
        style="margin-top: 80px; margin-bottom: 60px"
      ></mwc-circular-progress>
    </div>
    <div class="column" style="width: 512px" v-else-if="appInfo">
      <div class="column" style="flex: 1">
        <mwc-textfield
          label="App Id"
          outlined
          ref="appIdField"
          class="row-item"
          @input="appId = $event.target.value"
          autoValidate
          required
          helper="Has to be unique"
          style="flex: 1"
        ></mwc-textfield>

        <mwc-select
          v-if="!hdkVersionForApp"
          label="Install To Holochain Version:"
          outlined
          :fixedMenuPosition="true"
          required
          style="margin-top: 16px"
          @selected="holochainId = supportedHolochains[$event.detail.index]"
          @closing="$event.stopPropagation()"
        >
          <mwc-list-item
            v-for="(holochainId, index) of supportedHolochains"
            :key="index"
            :value="holochainId.content || 'Custom Holochain Binary'"
            >{{
              holochainId.content || "Custom Holochain Binary"
            }}</mwc-list-item
          >
        </mwc-select>

        <div class="row" style="margin-top: 16px; align-items: center">
          <mwc-icon-button
            :icon="showAdvanced ? 'expand_less' : 'expand_more'"
            @click="showAdvanced = !showAdvanced"
          ></mwc-icon-button>
          <span>Advanced</span>
        </div>

        <div
          class="column"
          :style="{ display: showAdvanced ? 'flex' : 'none' }"
        >
          <mwc-textfield
            label="Network Seed"
            outlined
            @input="networkSeed = $event.target.value"
            helper="Change it to create a new network"
            class="row-item"
            style="flex: 1; margin-top: 8px"
          ></mwc-textfield>

          <mwc-select
            style="margin-top: 8px"
            label="Agent Public Key"
            outlined
            :fixedMenuPosition="true"
            @selected="reuseAgentPubKey = allPubKeys[$event.detail.index - 1]"
            @closing="$event.stopPropagation()"
          >
            <mwc-list-item selected value="1"
              >Generate new Public Key</mwc-list-item
            >
            <mwc-list-item
              v-for="pubKey of allPubKeys"
              :key="pubKey"
              :value="pubKey"
              >{{ pubKey }}</mwc-list-item
            >
          </mwc-select>

          <span
            class="row-item"
            style="font-size: 18px; font-weight: 400; color: black"
            >Dna Roles</span
          >
          <div
            v-for="(appRole, index) of appInfo.roles_to_create"
            :key="appRole.id"
            class="column"
            style="flex: 1; margin-top: 8px"
          >
            <span>#{{ index + 1 }} {{ appRole.id }}</span>
            <mwc-textarea
              style="flex: 1; margin-top: 4px"
              label="Membrane Proof"
              outlined
              helper="Check with the author if this is required"
              @input="
                membraneProofs[appRole.id] = $event.target.value;
                onChange();
              "
            ></mwc-textarea>
          </div>
        </div>
      </div>
    </div>
    <mwc-button
      label="Cancel"
      slot="secondaryAction"
      dialogAction="close"
      :disabled="installing"
    >
    </mwc-button>
    <mwc-button
      v-if="appBundlePath"
      slot="primaryAction"
      :disabled="!isAppReadyToInstall || installing"
      @click="installApp()"
      :label="installing ? 'Installing...' : 'Install app'"
    >
    </mwc-button>
  </mwc-dialog>
  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import type { TextField } from "@material/mwc-textfield";
import { InstalledAppInfo, AgentPubKey } from "@holochain/client";
import {
  HolochainId,
  HolochainVersion,
  InstalledWebAppInfo,
  WebAppInfo,
} from "../types";
import { toUint8Array } from "js-base64";
import { flatten, uniq } from "lodash-es";
import { deserializeHash, serializeHash } from "@holochain-open-dev/utils";
import "@material/mwc-textarea";
import "@material/mwc-select";
import "@material/mwc-dialog";
import "@material/mwc-button";
import { ActionTypes } from "@/store/actions";

export default defineComponent({
  name: "SetupApp",
  props: {
    appBundlePath: {
      type: String,
      required: true,
    },
    hdkVersionForApp: {
      type: String,
    },
  },
  data(): {
    installing: boolean;
    showAdvanced: boolean;
    appId: string | undefined;
    networkSeed: string | undefined;
    membraneProofs: { [key: string]: string } | undefined;
    appInfo: WebAppInfo | undefined;
    isAppIdValid: boolean;
    reuseAgentPubKey: string | undefined;
    holochainId: HolochainId | undefined;
    supportedHolochains: HolochainId[];
    snackbarText: string | undefined;
  } {
    return {
      installing: false,
      appId: undefined,
      showAdvanced: false,
      networkSeed: undefined,
      membraneProofs: undefined,
      appInfo: undefined,
      isAppIdValid: true,
      reuseAgentPubKey: undefined,
      holochainId: undefined,
      snackbarText: undefined,
      supportedHolochains: [],
    };
  },
  computed: {
    isAppReadyToInstall() {
      if (!this.appId) return false;
      if (!this.holochainId) return false;
      if (!this.isAppIdValid) return false;
      return true;
    },
    isFileLoaded() {
      if (this.appInfo) return true;
      return false;
    },
    isLoadingFile() {
      if (this.appBundlePath && !this.appInfo) return true;
      return false;
    },
    allPubKeys() {
      if (!this.holochainId) return [];

      const pubkeys = this.$store.getters["allPublicKeysForHolochainId"](
        this.holochainId
      );
      return uniq(pubkeys.map(serializeHash));
    },
  },
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);

    const { holochain_versions }: { holochain_versions: HolochainVersion[] } =
      await invoke("get_supported_versions", {});
    const orderedVersions = holochain_versions.sort((a, b) =>
      b > a ? 1 : a === b ? 0 : -1
    );
    this.supportedHolochains = orderedVersions.map((v) => ({
      type: "HolochainVersion",
      content: v,
    }));

    if (
      this.$store.getters["runningHolochainIds"].find(
        (id: HolochainId) => id.type === "CustomBinary"
      )
    ) {
      this.supportedHolochains.push({ type: "CustomBinary" });
    }

    if (this.hdkVersionForApp) {
      // Get Holochain Version
      const version: HolochainVersion = await invoke("choose_version_for_hdk", {
        hdkVersion: this.hdkVersionForApp,
      });
      this.holochainId = {
        type: "HolochainVersion",
        content: version,
      };
    }

    this.membraneProofs = {};
    this.appInfo = (await invoke("get_app_info", {
      appBundlePath: this.appBundlePath,
    })) as WebAppInfo;
    this.appId = this.appInfo.app_name;

    this.$nextTick(() => {
      setTimeout(() => {
        const appIdField = this.$refs.appIdField as TextField;
        appIdField.validityTransform = (newValue: string, nativeValidity) => {
          if (newValue === "") {
            appIdField.setCustomValidity("App Id is required");
            this.isAppIdValid = false;

            return {
              valid: false,
            };
          }

          const holochainIds: HolochainId[] =
            this.$store.getters["runningHolochainIds"];

          const appsForHolochain = flatten(
            holochainIds.map((holochainId) =>
              this.$store.getters["appsForHolochain"](holochainId)
            )
          );

          const alreadyExists = appsForHolochain.find(
            (appInfo: InstalledWebAppInfo) =>
              appInfo.installed_app_info.installed_app_id === newValue
          );

          const isValid = !alreadyExists;

          // changes to make to the native validity
          if (!isValid) {
            appIdField.setCustomValidity("App Id already exists");
          }
          this.isAppIdValid = isValid;
          return {
            valid: isValid,
          };
        };
        appIdField.value = this.appId as string;
        appIdField.reportValidity();
      });
    });
  },
  methods: {
    serializeHash,
    pathToFilename(path: string) {
      const components = path.split("/");
      return components[components.length - 1];
    },
    getEncodedMembraneProofs() {
      if (!this.membraneProofs) return {};

      const encodedMembraneProofs: Record<string, Array<number>> = {};
      for (const dnaSlot of Object.keys(this.membraneProofs)) {
        encodedMembraneProofs[dnaSlot] = Array.from(
          toUint8Array(this.membraneProofs[dnaSlot])
        );
      }
      return encodedMembraneProofs;
    },
    async installApp() {
      if (!this.isAppReadyToInstall) return;

      try {
        this.installing = true;

        await invoke("install_app", {
          appId: this.appId,
          appBundlePath: this.appBundlePath,
          membraneProofs: this.membraneProofs,
          networkSeed: this.networkSeed,
          reuseAgentPubKey: this.reuseAgentPubKey,
          holochainId: this.holochainId,
        });

        this.installing = false;
        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Installed ${this.appId}`);

        this.$emit("app-installed", this.appId);
      } catch (e) {
        this.installing = false;
        this.showMessage(JSON.stringify(e));
      }
    },
    showMessage(message: string) {
      this.snackbarText = message;
      (this.$refs as any).snackbar.show();
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.app-row {
  margin-bottom: 16px;
}

.app-title {
  font-size: 24px;
}

.cell-row {
  margin-top: 8px;
}
.center {
  align-items: center;
  justify-content: center;
}

.row-item {
  margin-top: 16px;
}
</style>
