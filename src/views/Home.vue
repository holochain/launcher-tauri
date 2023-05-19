<template>
  <div style="width: 100%;">
    <div class="row center-content top-bar" style="position: sticky; top: 0; z-index: 1">
      <img
        src="/img/Square284x284Logo.png"
        style="height: 42px; margin-left: 11px"
      />
      <span 
        :class="{ tab: true, selectedTab: view.type === 'launcher' }" 
        @click="view.type = 'launcher'"
      >
        {{$t("main.launcher")}}
      </span>
      <span 
        :class="{ tab: true, selectedTab: view.type === 'appStore' }" 
        @click="view.type = 'appStore'"
      >
        {{$t("appStore.appStore")}}
      </span>
      <span style="display: flex; flex: 1"></span>
      <HCButton
        style="
          margin-left: 8px;
          margin-right: 12px;
          height: 40px;
          border-radius: 8px;
          padding: 0 20px;
          cursor: pointer;
        "
        :title="reportIssueUrl"
        @click="reportIssue()"
      >
        <div class="row center-content">
          <span style="margin-left: 5px">{{ $t("main.reportIssue") }}</span>
        </div>
      </HCButton>
      <span 
        :class="{ tab: true, selectedTab: view.type === 'settings' }" 
        @click="view.type = 'settings'"
      >
        {{$t("main.settings")}}
      </span>
    </div>

    <div
      v-if="isLoading()"
      class="column center-content" style="flex: 1; height: calc(100vh - 64px);"
    >
      <LoadingDots style="--radius: 15px; --dim-color: #e8e8eb; --fill-color: #b5b5b5"></LoadingDots>
    </div>

    <div v-else class="row" style="flex: 1; overflow-y: auto;">
      <div v-if="view.type === 'launcher'" class="flex-scrollable-parent">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <Launcher></Launcher>
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
            <Settings :installedApps="$store.getters[`allApps`]"></Settings>
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
import { ActionTypes } from "../store/actions";
import HCButton from "../components/subcomponents/HCButton.vue";
import LoadingDots from "../components/subcomponents/LoadingDots.vue";
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
    HCButton,
    Launcher,
    LoadingDots,
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
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);
  },
  methods: {
    isLoading() {
      return this.$store.state.launcherStateInfo === "loading";
    },
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
    cursor: pointer;
    font-size: 1.5em; 
    margin-left: 13px;
    padding: 0 10px;
    height: 64px;
    line-height: 64px;
    vertical-align: middle;;
  }
  .selectedTab {
    background-color: blue;
    color: white;
  }
  </style>