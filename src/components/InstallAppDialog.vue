<template>
  <HCDialog ref="dialog" @closing="$emit('closing-dialog')" :prohibit-escape="showNetworkSeedInfo">
    <div
      v-if="isLoadingFile"
      class="column"
      style="
        align-items: center;
        justify-content: center;
        width: 312px;
        overflow: hidden;
        position: relative;
      "
    >
      <!-- <img class="halo" src="/img/Holochain_Halo_no_gradient.svg" /> -->
      <mwc-circular-progress
        indeterminate
        style="margin-top: 40px; margin-bottom: 40px"
      ></mwc-circular-progress>
      <div style="margin-bottom: 10px">Processing file...</div>
    </div>

    <div
      v-else-if="installing"
      class="column"
      style="
        align-items: center;
        justify-content: center;
        width: 312px;
        overflow: hidden;
        position: relative;
      "
    >
      <!-- <img class="halo" src="/img/Holochain_Halo_no_gradient.svg" /> -->
      <mwc-circular-progress
        indeterminate
        style="margin-top: 40px; margin-bottom: 40px"
      ></mwc-circular-progress>
      <div style="margin-bottom: 10px">Installing...</div>
    </div>

    <div
      v-else-if="appInfo"
      class="column"
      style="align-items: center; margin: 10px 15px; padding: 0 10px"
    >
      <div style="font-weight: 600; font-size: 25px; margin: 20px 0 10px 0">
        Install App
      </div>
      <div style="margin-bottom: 30px; color: #482edf; font-size: 20px">
        {{ appInfo?.app_name || "" }}
      </div>
      <HCTextField
        @input="checkAppIdValidity"
        placeholder="App Id"
        style="margin: 5px; margin-bottom: 15px; width: 360px"
        label="App Id*"
        helper="Choose your own name for this app"
        :invalid="appIdInvalid"
        ref="app-id-field"
      />
      <HCSelect
        v-if="holochainSelection"
        style="margin: 5px; margin-bottom: 15px; width: 360px"
        label="Holochain Version*"
        :items="supportedHolochains"
        @item-selected="handleHolochainIdSelected($event)"
      >
      </HCSelect>
      <div class="row" style="align-items: flex-start;">
        <HCTextField
            placeholder="Network Seed (Optional)"
            label="Network Seed (Optional)"
            title="If in doubt, leave this blank"
            style="margin: 5px; margin-bottom: 25px; width: 325px;"
            helper="Peers with the same network seed are part of the same network"
            ref="network-seed-field"
        />
        <img
          @mouseover="openNetworkSeedInfo"
          @mouseleave="closeNetworkSeedInfo"
          @keydown.enter="openNetworkSeedInfo"
          @keydown.esc="closeNetworkSeedInfo"
          tabindex="0"
          src="/img/info_icon.svg"
          style="width: 27px; cursor: pointer; margin-top: 18px; margin-left: 3px; margin-right: 5px; opacity: 0.9;"
        >
      </div>
      <div class="column" style="width: 100%">
        <div class="row" style="margin: 20px 0 15px 20px;"
        >
          <div
            @click="showAdvanced = !showAdvanced"
            @keydown.enter="showAdvanced = !showAdvanced"
            class="row advanced-button"
            style="align-items: center;"
            tabindex="0"
          >
            <div
              style="
                text-align: center;
                font-size: 24px;
                width: 20px;
                cursor: pointer;
              "
            >
              {{ showAdvanced ? "-" : "+" }}
            </div>
            <div style="font-size: 20px; margin-left: 10px">Advanced</div>
          </div>
          <span style="display: flex; flex: 1;"></span>
        </div>
      </div>

      <div v-show="showAdvanced" class="column" style="align-items: center">

        <HCSelect
          style="margin: 5px; margin-bottom: 15px; width: 360px; display: none;"
          label="Public Key"
          :items="allPubKeys"
          :invalid="holochainId ? undefined : 'Select Holochain version first'"
          @item-selected="reuseAgentPubKey = $event"
          helper="Optionally choose an already existing public key"
        >
        </HCSelect>

        <div class="row" style="align-items: flex-start; width: 100%">
          <span
            style="
              margin-left: 5px;
              margin-top: 10px;
              font-weight: 600;
              font-size: 18px;
            "
            >Dna Roles</span
          >
        </div>

        <div
          v-for="(appRole, index) of appInfo.roles_to_create"
          :key="appRole.name"
          class="column"
          style="flex: 1; margin-top: 8px"
        >
          <span>#{{ index + 1 }} {{ appRole.name }}</span>

          <HCTextArea
            placeholder="Membrane Proof"
            style="margin-top: 10px; display: flex; flex: 1"
            :cols="32"
            label="Membrane Proof"
            helper="Check with the author whether this is required."
            @input="membraneProofs[appRole.name] = $event.target.value"
          />
        </div>
      </div>

      <div class="row" style="margin-top: 30px; margin-bottom: 20px">
        <HCButton
          style="width: 80px; height: 30px; margin: 4px 6px"
          outlined
          @click="close"
          >Cancel</HCButton
        >
        <HCButton
          style="width: 80px; margin: 4px 6px"
          :disabled="!isAppReadyToInstall || installing || !!appIdInvalid"
          @click="installApp()"
          >Install</HCButton
        >
      </div>
    </div>
  </HCDialog>

  <div class="info-popup row" v-if="showNetworkSeedInfo">
    <img src="/img/info_icon.svg" style="opacity: 0.9; width: 40px;">
    <div style="margin: 0 15px;">
      If you enter a network seed, you create an app instance with its
      own network and data store. If in doubt, leave this field blank to join
      the app's public network. Make sure that others who want to collaborate
      with you enter the same network seed!
    </div>
  </div>

  <HCSnackbar
    :timeoutMs="10000"
    :labelText="snackbarText"
    ref="snackbar"
  ></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ActionTypes } from "../store/actions";
import { flatten, uniq } from "lodash-es";
import { toUint8Array } from "js-base64";
import { encodeHashToBase64 } from "@holochain/client";

import HCButton from "./subcomponents/HCButton.vue";
import HCDialog from "./subcomponents/HCDialog.vue";
import HCTextField from "./subcomponents/HCTextField.vue";
import HCTextArea from "./subcomponents/HCTextArea.vue";
import HCSelect from "./subcomponents/HCSelect.vue";
import HCSnackbar from "./subcomponents/HCSnackbar.vue";
import {
  HolochainId,
  HolochainVersion,
  InstalledWebAppInfo,
  WebAppInfo,
} from "../types";
import { AppWithReleases } from "../devhub/get-happs";
import { AppRoleManifest } from "@holochain/client";

export default defineComponent({
  name: "InstallAppDialog",
  emits: ["error", "closing-dialog", "app-installed"],
  components: {
    HCDialog,
    HCTextField,
    HCTextArea,
    HCButton,
    HCSelect,
    HCSnackbar,
  },
  props: {
    appBundlePath: {
      type: String,
      required: true,
    },
    holochainSelection: {
      type: Boolean,
    },
    happReleaseHash: {
      type: String,
    },
    guiReleaseHash: {
      type: String,
    },
  },
  data(): {
    showAdvanced: boolean;
    installing: boolean;
    appId: string | undefined;
    membraneProofs: { [key: string]: string };
    appInfo: WebAppInfo | undefined;
    isAppIdValid: boolean;
    reuseAgentPubKey: string | undefined;
    holochainId: HolochainId | undefined;
    supportedHolochains: Array<[string, string]>; // holochain version string as key
    snackbarText: string | undefined;
    appIdInvalid: string | undefined;
    error: boolean;
    showNetworkSeedInfo: boolean;
  } {
    return {
      showAdvanced: false,
      installing: false,
      appId: undefined,
      membraneProofs: {},
      appInfo: undefined,
      isAppIdValid: true,
      reuseAgentPubKey: undefined,
      holochainId: undefined,
      snackbarText: undefined,
      supportedHolochains: [],
      appIdInvalid: undefined,
      error: false,
      showNetworkSeedInfo: false,
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
      if (this.appBundlePath && !this.appInfo &&!this.error) return true;
      return false;
    },
    allPubKeys() {
      if (!this.holochainId) return [];

      const pubkeys: Uint8Array[] = this.$store.getters[
        "allPublicKeysForHolochainId"
      ](this.holochainId);

      const allPubKeys: [string, string | undefined][] = [
        ["Generate New Public Key (default)", undefined],
      ];
      uniq(pubkeys.map(encodeHashToBase64)).forEach((pubKey) => {
        allPubKeys.push([pubKey, pubKey]);
      });

      return allPubKeys;
    },
  },
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);

    const { holochain_versions }: { holochain_versions: HolochainVersion[] } =
      await invoke("get_supported_versions", {});
    const orderedVersions = holochain_versions.sort((a, b) =>
      b > a ? 1 : a === b ? 0 : -1
    );

    let supportedHolochains: Array<[string, string]> = [];
    orderedVersions.forEach((v) => supportedHolochains.push([v, v]));

    if (
      this.$store.getters["runningHolochainIds"].find(
        (id: HolochainId) => id.type === "CustomBinary"
      )
    ) {
      supportedHolochains.push(["Custom Binary", "CustomBinary"]);
    }

    this.supportedHolochains = supportedHolochains;

    if (!this.holochainSelection) {
      try {
        this.holochainId = this.$store.getters["holochainIdForDevhub"];
      } catch (e) {
        console.log("Failed to get holochain version: ", e);
      }
    }

    try {
      this.appInfo = (await invoke("get_app_info", {
        appBundlePath: this.appBundlePath,
      })) as WebAppInfo;
      this.appId = this.appInfo.app_name;

      this.$nextTick(() => {
        const appIdField = this.$refs["app-id-field"] as typeof HCTextField;
        appIdField.value = this.appId as string;
        this.checkAppIdValidity();
      });
    } catch (e) {
      this.$emit("error", `Error: ${JSON.stringify(e)}`)
      console.log("Error getting app info: ", e);
      this.close();
    }
  },
  methods: {
    openNetworkSeedInfo() {
      this.showNetworkSeedInfo = true;
    },
    closeNetworkSeedInfo() {
      // Called with a lag to make sure the escape key is still blocked on the main dialog
      window.setTimeout(() => this.showNetworkSeedInfo = false, 50);
    },
    open() {
      (this.$refs.dialog as typeof HCDialog).open();
    },
    close() {
      (this.$refs.dialog as typeof HCDialog).close();
    },
    checkAppIdValidity() {
      const newValue = (this.$refs["app-id-field"] as typeof HCTextField).value;

      let regExp = new RegExp(/^[a-zA-Z 0-9-:_.]*$/);

      if (newValue === "") {
        this.appIdInvalid = "App Id must not be empty.";
        return;
      } else if (!regExp.test(newValue)) {
        // this restriction is added here because labels of tauri windows require it and we base window labels on app id's
        // additionally '/' is forbidden because it is interpreted as an OS path separator when storing the web assets
        this.appIdInvalid =
          "App Id may only contain alphanumeric characters as well as '-', ':', '.' and '_'";
        return;
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

      if (alreadyExists) {
        this.appIdInvalid = "App Id already exists.";
        return;
      }

      this.appIdInvalid = undefined;
    },
    handleHolochainIdSelected(version: string) {
      if (version === "CustomBinary") {
        this.holochainId = {
          type: "CustomBinary",
          content: undefined,
        };
      } else {
        this.holochainId = {
          type: "HolochainVersion",
          content: version,
        };
      }
    },
    encodeHashToBase64,
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

      const appId = (this.$refs["app-id-field"] as typeof HCTextField).value;
      const networkSeed = (
        this.$refs["network-seed-field"] as typeof HCTextField
      ).value;

      try {
        this.installing = true;

        // console.log("@InstallAppDialog: guiReleaseHash: ", this.guiReleaseHash);
        // console.log("@InstallAppDialog: happReleaseHash: ", this.happReleaseHash);

        await invoke("install_app", {
          appId,
          appBundlePath: this.appBundlePath,
          membraneProofs: this.getEncodedMembraneProofs(),
          networkSeed,
          reuseAgentPubKey: this.reuseAgentPubKey,
          holochainId: this.holochainId,
          happReleaseHash: this.happReleaseHash,
          guiReleaseHash: this.guiReleaseHash,
        });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Installed ${this.appId}`);

        this.$emit("app-installed", this.appId);

        this.installing = false;

      } catch (e) {
        console.log("Error installing the app: ", e);
        this.showMessage(`Error installing app: ${JSON.stringify(e)}`);
        this.installing = false;
        this.$nextTick(() => {
          (this.$refs["app-id-field"] as typeof HCTextField).value = this.appId;
          this.checkAppIdValidity();
        });
      }
    },
    showMessage(message: string) {
      this.snackbarText = message;
      (this.$refs as any).snackbar.show();
    },
  },
});
</script>

<style scoped>
.halo {
  height: 130%;
  top: -3%;
  position: absolute;
  opacity: 50%;
}

.advanced-button:hover {
  opacity: 0.8;
  cursor: pointer;
}

.info-popup {
  align-items: flex-start;
  border-radius: 12px;
  background-color: white;
  position: fixed;
  bottom: 50px;
  right: 50px;
  padding: 17px;
  max-width: 600px;
  margin: 0; /*reset some browser centering*/
  border: 4px solid var(--hc-primary-color);
  box-shadow: 0 0px 5px #9b9b9b;
  overflow-y: auto;
  z-index: 10000;
}
</style>
