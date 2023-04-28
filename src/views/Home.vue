<template>
  <div style="width: 100%;">
    <div class="row center-content top-bar" style="position: sticky; top: 0; z-index: 1">
      <img
        src="/img/Square284x284Logo.png"
        style="height: 42px; margin-left: 11px"
      />
      <span 
        :class="{ tab: true, selectedTab: view.type === 'launcher' }" 
        style="font-size: 1.5em; margin-left: 13px"
        @click="view.type = 'launcher'"
      >
        {{$t("main.launcher")}}
      </span>
      <span 
        :class="{ tab: true, selectedTab: view.type === 'appStore' }" 
        style="font-size: 1.5em; margin-left: 13px"
        @click="view.type = 'appStore'"
      >
        {{$t("appStore.appStore")}}
      </span>
      <span 
        :class="{ tab: true, selectedTab: view.type === 'settings' }" 
        style="font-size: 1.5em; margin-left: 13px"
        @click="view.type = 'settings'"
      >
        {{$t("main.settings")}}
      </span>
      <span style="display: flex; flex: 1"></span>
      <!-- <HCButton
        outlined
        @click="installDevHub()"
        style="height: 36px; border-radius: 8px; padding: 0 20px"
        title="Install DevHub"
        :disabled="installingDevHub"
        >{{ installingDevHub ? 'installing...' : 'Install DevHub' }}
      </HCButton> -->
      <HCButton
        style="
          margin-left: 8px;
          margin-right: 12px;
          height: 40px;
          border-radius: 8px;
          padding: 0 20px;
        "
        :title="reportIssueUrl"
        @click="reportIssue()"
      >
        <div class="row center-content">
          <span style="margin-left: 5px">{{ $t("main.reportIssue") }}</span>
        </div>
      </HCButton>
    </div>

    <div class="row" style="flex: 1; overflow-y: auto;">
      <div v-if="view.type === 'launcher'" class="flex-scrollable-parent">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <Launcher
              style="display: flex; margin: 24px; margin-bottom: 50px;"
            ></Launcher>
          </div>
        </div>
      </div>

      <div v-else-if="view.type === 'appStore'" class="flex-scrollable-parent">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <AppStore></AppStore>
          </div>
        </div>
      </div>

      <div v-else style="flex: 1; display: flex">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <Settings
              style="display: flex; margin: 24px; margin-bottom: 50px;"
            ></Settings>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import AppStore from "./AppStore.vue";
import Launcher from "./Launcher.vue";
import Settings from "./Settings.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";
import "@material/mwc-fab";

type View =
  | {
      type: "launcher";
    }  
  | {
      type: "appStore";
    }
  | {
      type: "settings";
    };

export default defineComponent({
  name: "Home",
  components: {
    AppStore,
    Launcher,
    Settings
  },
  data(): {
    reportIssueUrl: string;
    snackbarText: string | undefined;
    view: View;
  } {
    return {
      reportIssueUrl: "https://github.com/holochain/launcher/issues/new",
      snackbarText: undefined,
      view: {
        type: "launcher",
      },
    };
  },
  methods: {
    async reportIssue() {
      await invoke("open_url_cmd", {
        url: this.reportIssueUrl
      });
    },
  },
});
</script>

<style scoped>
  .top-bar {
    align-items: center;
    height: 64px;
    width: 100%;
    /* background: #e8e8eb; */
    background: white;
    box-shadow: 0 0px 5px #9b9b9b;
  }

  .tab {
    display: inline-block;
  }
  .selectedTab {
    background-color: blue;
    color: white;
  }
  </style>