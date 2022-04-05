<template>
  <div class="row center" style="width: 512px" v-if="isLoadingFile">
    <mwc-circular-progress
      indeterminate
      style="margin-top: 80px; margin-bottom: 60px"
    ></mwc-circular-progress>
  </div>
  <div class="column" style="width: 512px" v-else-if="appInfo">
    <span style="margin-right: 8px"
      >Bundle: {{ pathToFilename(appBundlePath) }}</span
    >

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
      <mwc-textfield
        label="UID"
        outlined
        @input="uid = $event.target.value"
        helper="Change it to create a new network"
        class="row-item"
        style="flex: 1"
      ></mwc-textfield>

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
          @input="membraneProofs[appRole.id] = $event.target.value"
        ></mwc-textarea>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import type { TextField } from "@material/mwc-textfield";
import { InstalledAppInfo } from "@holochain/client";
import { AppSetup, HolochainVersion, WebAppInfo } from "../../types";
import { toUint8Array } from "js-base64";
import { ActionTypes } from "@/store/actions";
import { flatten } from "lodash-es";

export default defineComponent({
  name: "SetupApp",
  props: {
    appBundlePath: {
      type: String,
      required: true,
    },
  },
  data(): {
    appId: string | undefined;
    uid: string | undefined;
    membraneProofs: { [key: string]: string } | undefined;
    appInfo: WebAppInfo | undefined;
    isAppIdValid: boolean;
  } {
    return {
      appId: undefined,
      uid: undefined,
      membraneProofs: undefined,
      appInfo: undefined,
      isAppIdValid: true,
    };
  },
  computed: {
    isAppReadyToInstall() {
      if (!this.appId) return false;
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
  },
  async created() {
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

          const holochainVersions: HolochainVersion[] =
            this.$store.getters["holochainVersions"];

          const appsForVersion = flatten(
            holochainVersions.map((version) =>
              this.$store.getters["appsForVersion"](version)
            )
          );

          const alreadyExists = appsForVersion.find(
            (appInfo: InstalledAppInfo) => appInfo.installed_app_id === newValue
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
    onChange() {
      if (!this.isAppReadyToInstall) {
        this.$emit("setup-changed", undefined);
      }

      const setup: AppSetup = {
        appId: this.appId as string,
        uid: this.uid,
        membraneProofs: this.getEncodedMembraneProofs(),
      };

      this.$emit("setup-changed", setup);
    },
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
