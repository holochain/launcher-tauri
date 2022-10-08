<template>
  <HCDialog ref="dialog" @closing="$emit('closing-dialog')">
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
      <img class="halo" src="/img/Holochain_Halo_no_gradient.svg" />
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
      <img class="halo" src="/img/Holochain_Halo_no_gradient.svg" />
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
        style="margin: 5px; width: 360px"
        label="Holochain Version*"
        :items="supportedHolochains"
        @item-selected="handleHolochainIdSelected($event)"
      >
      </HCSelect>
      <div class="column" style="width: 100%">
        <div class="row" style="margin: 20px 0 15px 20px; align-items: center">
          <div
            @click="showAdvanced = !showAdvanced"
            @keydown.enter="showAdvanced = !showAdvanced"
            style="
              text-align: center;
              font-size: 24px;
              width: 20px;
              cursor: pointer;
            "
            tabindex="0"
          >
            {{ showAdvanced ? "-" : "+" }}
          </div>
          <div style="font-size: 20px; margin-left: 10px">Advanced</div>
        </div>
      </div>

      <div v-show="showAdvanced" class="column" style="align-items: center">
        <HCTextField
          placeholder="Network Seed"
          label="Network Seed"
          style="margin: 5px; margin-bottom: 15px; width: 360px"
          helper="Change it to create a new network"
          ref="network-seed-field"
        />

        <HCSelect
          style="margin: 5px; margin-bottom: 15px; width: 360px"
          label="Public Key"
          :items="allPubKeys"
          :invalid="holochainId ? undefined : 'Select Holochain version first'"
          @item-selected="reuseAgentPubKey = $event"
          helper="Optionally chose an already existing public key"
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
          :key="appRole.id"
          class="column"
          style="flex: 1; margin-top: 8px"
        >
          <span>#{{ index + 1 }} {{ appRole.id }}</span>

          <HCTextArea
            placeholder="Membrane Proof"
            style="margin-top: 10px; display: flex; flex: 1"
            :cols="32"
            label="Membrane Proof"
            helper="Check with the author if this is required."
            @input="
              membraneProofs[appRole.id] = $event.target.value;
              onChange();
            "
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
          :disabled="!isAppReadyToInstall || installing || appIdInvalid"
          @click="installApp()"
          >Install</HCButton
        >
      </div>
    </div>
  </HCDialog>

  <mwc-snackbar leading :labelText="snackbarText" ref="snackbar"></mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ActionTypes } from "@/store/actions";
import { serializeHash } from "@holochain-open-dev/utils";
import { flatten, uniq } from "lodash-es";
import { toUint8Array } from "js-base64";

import HCButton from "./subcomponents/HCButton.vue";
import HCDialog from "./subcomponents/HCDialog.vue";
import HCTextField from "./subcomponents/HCTextField.vue";
import HCTextArea from "./subcomponents/HCTextArea.vue";
import HCSelect from "./subcomponents/HCSelect.vue";
import {
  HolochainId,
  HolochainVersion,
  InstalledWebAppInfo,
  WebAppInfo,
} from "../types";

export default defineComponent({
  name: "InstallAppDialog",
  components: { HCDialog, HCTextField, HCTextArea, HCButton, HCSelect },
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
    showAdvanced: boolean;
    installing: boolean;
    appId: string | undefined;
    membraneProofs: { [key: string]: string } | undefined;
    appInfo: WebAppInfo | undefined;
    isAppIdValid: boolean;
    reuseAgentPubKey: string | undefined;
    holochainId: HolochainId | undefined;
    supportedHolochains: Array<[string, string]>; // holochain version string as key
    snackbarText: string | undefined;
    appIdInvalid: string | undefined;
  } {
    return {
      showAdvanced: false,
      installing: false,
      appId: undefined,
      membraneProofs: undefined,
      appInfo: undefined,
      isAppIdValid: true,
      reuseAgentPubKey: undefined,
      holochainId: undefined,
      snackbarText: undefined,
      supportedHolochains: [],
      appIdInvalid: undefined,
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

      const pubkeys: Uint8Array[] = this.$store.getters[
        "allPublicKeysForHolochainId"
      ](this.holochainId);

      const allPubKeys: [string, string | undefined][] = [
        ["Generate New Public Key (default)", undefined],
      ];
      uniq(pubkeys.map(serializeHash)).forEach((pubKey) => {
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
      const appIdField = this.$refs["app-id-field"] as typeof HCTextField;
      appIdField.value = this.appId as string;
      this.checkAppIdValidity();
    });
  },
  methods: {
    open() {
      (this.$refs.dialog as typeof HCDialog).open();
    },
    close() {
      (this.$refs.dialog as typeof HCDialog).close();
    },
    checkAppIdValidity() {
      const newValue = (this.$refs["app-id-field"] as typeof HCTextField).value;

      let regExp = new RegExp(/^[a-zA-Z 0-9-/:_]*$/);

      if (newValue === "") {
        this.appIdInvalid = "App Id must not be empty.";
        return;
      } else if (!regExp.test(newValue)) {
        // this restriction is added here because labels of tauri windows require it and we base window labels on app id's
        this.appIdInvalid =
          "App Id may only contain alphanumeric characters as well as '-', '/', ':' and '_'";
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
        };
      } else {
        this.holochainId = {
          type: "HolochainVersion",
          content: version,
        };
      }
    },
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

      const appId = (this.$refs["app-id-field"] as typeof HCTextField).value;
      const networkSeed = (
        this.$refs["network-seed-field"] as typeof HCTextField
      ).value;

      try {
        this.installing = true;

        await invoke("install_app", {
          appId,
          appBundlePath: this.appBundlePath,
          membraneProofs: this.getEncodedMembraneProofs(),
          networkSeed,
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

<style scoped>
.halo {
  height: 130%;
  top: -3%;
  position: absolute;
  opacity: 50%;
}
</style>
