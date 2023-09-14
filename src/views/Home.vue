<template>
  <div style="width: 100%">
    <div
      class="row center-content top-bar"
      style="position: sticky; top: 0; z-index: 1"
    >
      <span
        :class="{ tab: true, selectedTab: view.type === 'launcher' }"
        @click="view.type = 'launcher'"
        @keypress.enter="view.type = 'launcher'"
        tabindex="0"
      >
        <img src="/img/launch_icon.svg" />
        <span>{{ $t("main.launcher") }}</span>
      </span>
      <span
        :class="{ tab: true, selectedTab: view.type === 'appStore' }"
        @click="view.type = 'appStore'"
        @keypress.enter="view.type = 'appStore'"
        tabindex="0"
      >
        <img src="/img/home_icon.svg" />
        <span>{{ $t("appStore.appStore") }}</span>
      </span>
      <span style="display: flex; flex: 1"></span>
      <div
        class="row center-content"
        style="padding: 0 15px; height: 100%"
        tabindex="0"
        @click="reportIssue()"
        @keypress.enter="reportIssue()"
      >
        <img
          src="/img/bug_icon.png"
          style="cursor: pointer; width: 20px; margin-top: 3px"
          :title="`Report Bug on GitHub (${reportIssueUrl})`"
        />
      </div>
      <span
        :class="{ tab: true, selectedTab: view.type === 'settings' }"
        tabindex="0"
        @click="view.type = 'settings'"
        @keypress.enter="view.type = 'settings'"
        :title="`${$t('main.settings')}${
          updatesAvailable ? $t('main.updatesAvailable') : ''
        }`"
        style="position: relative"
      >
        <img src="/img/gear_icon.svg" />
        <div
          v-if="updatesAvailable"
          style="
            background: rgb(255, 217, 0);
            border-radius: 50%;
            height: 15px;
            width: 15px;
            position: absolute;
            bottom: 32px;
            right: 10px;
          "
        ></div>
      </span>
    </div>

    <div class="row" style="flex: 1; overflow-y: auto">
      <div
        v-if="view.type === 'launcher'"
        class="flex-scrollable-parent"
        style="display: flex; flex: 1"
      >
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <Launcher
              @show-message="showMessage($event)"
              @select-view="selectView($event)"
            ></Launcher>
          </div>
        </div>
      </div>

      <div v-else-if="view.type === 'appStore'" class="flex-scrollable-parent">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <AppStore
              @show-message="showMessage($event)"
              @select-view="selectView($event)"
            ></AppStore>
          </div>
        </div>
      </div>

      <div v-else style="flex: 1; display: flex">
        <div class="flex-scrollable-container">
          <div class="flex-scrollable-y">
            <Settings
              :installedApps="$store.getters[`allApps`]"
              @show-message="showMessage($event)"
              @updated-ui="checkForUiUpdates"
            ></Settings>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="bottom-bar row">
    <img src="/img/Square284x284Logo.png" style="height: 30px; z-index: -1px" />
    <span class="beta-indicator">Beta 0.2</span>
  </div>

  <HCSnackbar leading :labelText="snackbarText" ref="snackbar"></HCSnackbar>
</template>

<script lang="ts">
import AppStore from "./AppStore.vue";
import Launcher from "./Launcher.vue";
import Settings from "./Settings.vue";
import { ActionTypes } from "../store/actions";
import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";
import "@material/mwc-fab";
import { APPSTORE_APP_ID } from "../constants";
import { HolochainAppInfo } from "../types";
import {
  AppWebsocket,
  decodeHashFromBase64,
  DnaHashB64,
  encodeHashToBase64,
} from "@holochain/client";
import { getHappReleasesByActionHashes } from "../appstore/appstore-interface";
import { HappReleaseEntry } from "../appstore/types";
import { mapActions, mapGetters } from "vuex";

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
    HCSnackbar,
    Launcher,
    Settings,
  },
  data(): {
    reportIssueUrl: string;
    snackbarText: string | undefined;
    updatesAvailable: boolean;
    view: View;
  } {
    return {
      reportIssueUrl: "https://github.com/holochain/launcher/issues/new",
      snackbarText: undefined,
      updatesAvailable: false,
      view: {
        type: "launcher",
      },
    };
  },
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);

    await this.checkForUiUpdates();
  },
  computed: {
    ...mapGetters(["appWebsocket"]),
  },
  methods: {
    isLoading() {
      return this.$store.state.launcherStateInfo === "loading";
    },
    ...mapActions(["connectToWebsocket"]),
    async checkForUiUpdates() {
      const installedApps: Array<HolochainAppInfo> =
        this.$store.getters[`allApps`];

      await this.connectToWebsocket();
      const appWebsocket = this.appWebsocket as AppWebsocket;

      if (!appWebsocket) {
        return;
      }

      const appstoreAppInfo = await appWebsocket.appInfo({
        installed_app_id: APPSTORE_APP_ID,
      });

      const updatableApps = installedApps.filter(
        (app) => app.webAppInfo.happ_release_info?.resource_locator
      );

      // sort all happ release ResourceLocators by DnaHash of the DevHub they originate from
      const updatableAppsByLocatorDna: Record<DnaHashB64, HolochainAppInfo[]> =
        {};

      updatableApps.forEach((app) => {
        const dnaHash =
          app.webAppInfo.happ_release_info!.resource_locator!.dna_hash;
        const apps = updatableAppsByLocatorDna[dnaHash];

        if (apps) {
          updatableAppsByLocatorDna[dnaHash] = [...apps, app];
        } else {
          updatableAppsByLocatorDna[dnaHash] = [app!];
        }
      });

      let updatesAvailable = false;

      // console.log("updatableAppsByLocatorDna: ", updatableAppsByLocatorDna);

      await Promise.allSettled(
        Object.values(updatableAppsByLocatorDna).map(async (apps) => {
          const actionHashes = apps.map((app) =>
            decodeHashFromBase64(
              app.webAppInfo.happ_release_info!.resource_locator!.resource_hash
            )
          );
          const devHubDnaHash = decodeHashFromBase64(
            apps[0].webAppInfo.happ_release_info!.resource_locator!.dna_hash
          );

          try {
            // console.log(
            //   "@Home.vue @created: actionHashes: ",
            //   actionHashes.map((eh) => encodeHashToBase64(eh))
            // );
            const happReleases: Array<HappReleaseEntry | undefined> =
              await getHappReleasesByActionHashes(
                appWebsocket as AppWebsocket,
                appstoreAppInfo,
                devHubDnaHash,
                actionHashes
              );

            apps.forEach((app, idx) => {
              // if (happReleases[idx]) {
              //   console.log(
              //     "official_gui: ",
              //     happReleases[idx]!.official_gui
              //       ? encodeHashToBase64(happReleases[idx]!.official_gui!)
              //       : undefined
              //   );
              // }

              // if it's installed as a webapp and the happ release has an official GUI, check whether it's a new GUI
              if (
                app.webAppInfo.web_uis.default.type === "WebApp" &&
                happReleases[idx]?.official_gui
              ) {
                const guiReleaseInfo =
                  app.webAppInfo.web_uis.default.gui_release_info;
                const guiReleaseHash =
                  app.webAppInfo.web_uis.default.gui_release_info
                    ?.resource_locator!.resource_hash;
                // console.log("guiReleaseHash: ", guiReleaseHash);
                if (guiReleaseInfo && guiReleaseHash) {
                  if (
                    guiReleaseHash !=
                    encodeHashToBase64(happReleases[idx]!.official_gui!)
                  ) {
                    updatesAvailable = true;
                  }
                }
              }
            });
          } catch (e) {
            console.error(
              `Failed to get happ releases from DevHub host of network with DNA hash ${encodeHashToBase64(
                devHubDnaHash
              )}: ${JSON.stringify(e)}`
            );
          }
        })
      );

      this.updatesAvailable = updatesAvailable;
    },
    async reportIssue() {
      await invoke("open_url_cmd", {
        url: this.reportIssueUrl,
      });
    },
    showMessage(message: string) {
      this.snackbarText = message;
      (this.$refs["snackbar"] as typeof HCSnackbar).show();
    },
    selectView(view: View) {
      this.view = view;
    },
  },
});
</script>

<style scoped>
.beta-indicator {
  margin-left: 5px;
  font-size: 20px;
  opacity: 0.3;
}
.bottom-bar {
  align-items: center;
  height: 30px;
  position: fixed;
  bottom: 5px;
  left: 5px;
  width: 100%;
  z-index: 0;
}
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
  padding: 0 15px;
  height: 64px;
  line-height: 64px;
  vertical-align: middle;
  color: #9e9ea2;
}

.tab:not(.selectedTab):hover {
  background: linear-gradient(228.21deg, #bc2fd834 0%, #2f86d840 94.99%);
}

.tab:not(.selectedTab):focus {
  background: linear-gradient(228.21deg, #bc2fd834 0%, #2f86d840 94.99%);
}

.tab img {
  height: 24px;
  /* Turn SVG gray using https://codepen.io/sosuke/pen/Pjoqqp */
  filter: invert(76%) sepia(0%) saturate(1356%) hue-rotate(209deg)
    brightness(82%) contrast(84%);
  vertical-align: middle;
}

.tab span {
  margin-left: 10px;
  vertical-align: middle;
}

.selectedTab {
  color: white;
  background: linear-gradient(228.21deg, #bb2fd8 0%, #2f87d8 94.99%);
  box-shadow: 0px 15px 25px rgba(0, 0, 0, 0.15);
}

.selectedTab img {
  /* Turn the SVG white using https://codepen.io/sosuke/pen/Pjoqqp */
  filter: invert(100%) sepia(100%) saturate(0%) hue-rotate(288deg)
    brightness(102%) contrast(102%);
}
</style>
