<template>
  <HCDialog ref="dialog" @closing="$emit('closing-dialog')">

    <div
      class="column"
      style="align-items: center; margin: 10px 15px; padding: 0 10px"
    >
      <div style="font-weight: 600; font-size: 25px; margin: 20px 0 10px 0">
        Select App Version
      </div>

      <div style="margin-bottom: 30px; color: #482edf; font-size: 20px">
        {{ appName }}
      </div>

      <div class="column" style="width: 100%; align-items: flex-start; margin-bottom: 40px;">

        <div class="column card">
          <div style="text-align: right; font-weight: 600">latest Release</div>

          <div class="row" style="align-items: flex-end;">
            <div class="column">
              <div><span style="font-weight: 600;">hApp version:</span> {{ releaseInfos[0].happRelease.content.version }}</div>
              <div><span style="font-weight: 600;">UI version: </span>{{ releaseInfos[0].guiRelease? releaseInfos[0].guiRelease.content.version : "no official UI" }}</div>
              <!-- GUI version as well here? Needs to be fetched independently from a DevHub host -->
            </div>
            <span style="display: flex; flex: 1;"></span>
            <HCButton @click="() => {
              $emit('release-selected', releaseInfos[0]);
              close();
            }">Install</HCButton>
          </div>
        </div>
        <div>
        </div>


        <div class="row" style="margin: 20px 0 10px 10px;"
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
            <div style="font-size: 18px; margin-left: 10px">show older releases</div>
          </div>
          <span style="display: flex; flex: 1;"></span>
        </div>

        <div v-show="showAdvanced" class="column" style="align-items: center; width: 100%;">
          <div v-if="releaseInfos.length < 2" style="text-align: center;">
            No older releases available for this app.
          </div>
          <div v-else>
            <div
              v-for="(releaseInfo) of releaseInfos.slice(1)"
              :key="releaseInfo.happRelease.content.version"
              class="row card"
              style="align-items: flex-end; padding-top: 15px;"
            >
              <div class="column">
                <div><span style="font-weight: 600;">hApp version:</span> {{ releaseInfo.happRelease.content.version }}</div>
                <div><span style="font-weight: 600;">UI version: </span>{{ releaseInfo.guiRelease? releaseInfo.guiRelease.content.version : "no official UI" }}</div>
                <!-- GUI version as well here? Needs to be fetched independently from a DevHub host -->
              </div>
              <span style="display: flex; flex: 1;"></span>
              <HCButton @click="() => {
                $emit('release-selected', releaseInfos[0]);
                close();
              }">Install</HCButton>
            </div>
          </div>

        </div>


      </div>


      <HCButton
        style="width: 80px; height: 30px; margin: 4px 6px; margin-bottom: 15px;"
        outlined
        @click="cancel"
      >Cancel
      </HCButton>

    </div>
  </HCDialog>

  <HCSnackbar
    :timeoutMs="10000"
    :labelText="snackbarText"
    ref="snackbar"
  ></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

import HCButton from "./subcomponents/HCButton.vue";
import HCDialog from "./subcomponents/HCDialog.vue";
import HCSnackbar from "./subcomponents/HCSnackbar.vue";

import { ReleaseInfo } from "../types";

export default defineComponent({
  name: "SelectReleaseDialog",
  emits: ["error", "closing-dialog", "release-selected", "cancel"],
  components: {
    HCDialog,
    HCButton,
    HCSnackbar,
  },
  props: {
    releaseInfos: {
      type: Object as PropType<Array<ReleaseInfo>>, // assumed to be sorted by "published_at" in descending order (latest release first)
      required: true,
    },
    appName: {
      type: String,
      required: true,
    }
  },
  data(): {
    showAdvanced: boolean;
    snackbarText: string | undefined;
    error: boolean;
  } {
    return {
      showAdvanced: false,
      snackbarText: undefined,
      error: false,
    };
  },
  methods: {
    open() {
      (this.$refs.dialog as typeof HCDialog).open();
    },
    close() {
      (this.$refs.dialog as typeof HCDialog).close();
      this.$emit('closing-dialog');
    },
    cancel() {
      (this.$refs.dialog as typeof HCDialog).close();
      this.$emit('cancel');
    }
  },
});
</script>

<style scoped>

.advanced-button:hover {
  opacity: 0.8;
  cursor: pointer;
}


.card {
  min-width: 500px;
  background: #f6f6fa;
  border-radius: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
  /* border: 2px solid #e1e1e1; */
  padding: 5px 15px 15px 15px;
  margin: 12px 0;
}
</style>
