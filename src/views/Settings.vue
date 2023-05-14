<template>
  <div style="display: flex; margin: 24px; margin-bottom: 50px; flex-direction: column; align-items: center;">
    <div class='column' style="flex: 1 1 0%; margin-bottom: 80px; padding: 0px 30px; width: 70%; min-width: 900px;">
  
      <!-- Holochain version info -->
      <div
        class="row section-title"
      >
        <span
          style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.6)"
          :title="$t('main.holochainVersionsHelper')"
          >{{ $t("main.holochainVersions") }}</span
        >
        <span style="flex: 1"></span>
        <span
          @click="refreshStorageInfo"
          style="margin-right: 5px; margin-bottom: -8px; cursor: pointer"
        >
          <img
            src="/img/refresh.png"
            style="height: 12px; margin-right: 3px; opacity: 0.7"
          />
          {{ $t("main.refresh") }}
        </span>
      </div>
      
      <div
        class="column"
        style="margin-bottom: 50px; width: 100%;"
      >
        <div
          v-if="noHolochainVersions"
          style="margin-top: 30px; color: rgba(0, 0, 0, 0.6); text-align: center"
        >
          {{ $t("main.noHolochainVersions") }}
        </div>
        <div v-else>
          <div
            v-for="hcVersion in holochainVersions"
            :key="hcVersion"
            style="
              display: flex;
              flex: 1;
              flex-direction: column;
              width: 100%;
            "
          >
            <div class="row section-container hc-version" style="margin: 5px 0">
              <img
                src="/img/Square284x284Logo.png"
                style="height: 42px; margin-left: 11px; margin-right: 11px"
              />
              <div>
                <h2>{{ hcVersion }}</h2>
              </div>
              <span style="display: flex; flex: 1"></span>
              <span
                v-if="storageInfos && !refreshing"
                style="font-weight: 600; margin-right: 15px"
                >{{ totalStorageString(hcVersion) }}</span
              >
              <StackedChart
                v-if="storageInfos && !refreshing"
                :fractions="storageFractions(hcVersion)"
                :labels="storageLabels(hcVersion)"
                style="width: 200px; height: 34px; margin-right: 12px"
              ></StackedChart>
              <span style="width: 120px; text-align: center">{{
                storageInfos[hcVersion]
                  ? prettyBytes(storageInfos[hcVersion].conductor)
                  : "?"
              }}</span>
              <span style="width: 120px; text-align: center">{{
                storageInfos[hcVersion]
                  ? prettyBytes(storageInfos[hcVersion].uis)
                  : "?"
              }}</span>
            </div>
          </div>
        </div>
      </div>

      <div class="column section-container">
        <h2>Developer Mode</h2>
        <span>Activates DevHub + empowers you to create apps</span>
        <!-- Disable/enable switch -->
        <sl-tooltip
          class="tooltip"
          hoist
          placement="top"
          :content="
            devHubAppInfo && isAppRunning(devHubAppInfo?.webAppInfo.installed_app_info)
              ? 'Disable Dev Mode'
              : 'Enable Dev Mode'
          "
        >
          <ToggleSwitch
            style="margin-right: 29px"
            :sliderOn="!!devHubAppInfo && isAppRunning(devHubAppInfo?.webAppInfo.installed_app_info)"
            @click="toggleDevMode()"
            @keydown.enter="toggleDevMode()"
          />
        </sl-tooltip>

        <span style="margin-top: 8px"
          ><span
            style="cursor: pointer; text-decoration: underline;"
            :title="howToPublishUrl"
            @click="howToPublish()"
            @keydown.enter="howToPublish()"
            tabindex="0"
            >{{ $t("settings.readThis") }}</span
          >
          {{ $t("settings.readThisToPublish") }}</span
        >
        <HCButton
          outlined
          @click="howToPublish()"
          style="height: 36px; border-radius: 8px; padding: 0 20px"
          :title="howToPublishUrl"
          >{{ $t("settings.howToPublishAnApp") }}
        </HCButton>
      </div>

      <!-- <div
        v-if="isLoading()"
        class="column center-content" style="flex: 1; height: calc(100vh - 64px);"
      >
        <LoadingDots style="--radius: 15px; --dim-color: #e8e8eb; --fill-color: #b5b5b5"></LoadingDots>
      </div> -->

      <div
        class="column"
        style="flex: 1; margin-top: 20px"
      >
        <!-- Web Apps -->
        <div
          class="row"
          style="
            width: 100%;
            justify-content: flex-end;
            align-items: center;
            margin-top: 20px;
            margin-bottom: -5px;
          "
        >
          <HCSelectCard
            style="
              width: 200px;
              margin-right: 5px;
              box-shadow: 0 0px 3px -1px #9b9b9b;
              --hc-label-background: #e8e8eb;
            "
            :placeholder="$t('main.holochainVersions')"
            :items="holochainVersionOptions"
            @item-selected="selectedHolochainVersion = $event"
          ></HCSelectCard>
          <img
            src="/img/Square284x284Logo.png"
            style="
              height: 30px;
              filter: grayscale(50%);
              margin-right: 20px;
              margin-left: -2px;
            "
          />

          <HCSelectCard
            style="
              width: 200px;
              margin-right: 5px;
              box-shadow: 0 0px 3px -1px #9b9b9b;
              --hc-label-background: #e8e8eb;
            "
            :placeholder="$t('main.sortBy')"
            :items="sortOptions"
            @item-selected="sortOption = $event"
          ></HCSelectCard>
          <mwc-icon style="color: #482edf; text-shadow: 0 0px 5px #9b9b9b"
            >sort</mwc-icon
          >
        </div>

        <div
          class="row section-title"
          :class="{ borderBottomed: showWebApps }"
          style="margin-top: -18px"
        >
          <span
            style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.6)"
            :title="$t('main.webAppsHelper')"
            >{{ $t("main.webApps") }}</span
          >
          <span
            @click="showWebApps = !showWebApps"
            class="show-hide"
            style="opacity: 0.7; cursor: pointer; margin-left: 10px"
          >
            {{ showWebApps ? "[-]" : "[show]" }}
          </span>
        </div>
        <div v-if="showWebApps" class='section-container' style="margin-bottom: 50px; width: 100%">
          <div
            v-if="noWebApps"
            style="margin-top: 30px; color: rgba(0, 0, 0, 0.6); text-align: center"
          >
            {{ $t("main.noWebApps") }}
            {{
              selectedHolochainVersion === "All Versions"
                ? "."
                : " in this Holochain Version."
            }}
          </div>

          <div
            v-else
            v-for="app in sortedApps"
            :key="app.webAppInfo.installed_app_info.installed_app_id"
            style="
              display: flex;
              flex-direction: column;
              width: 100%;
              align-items: center;
            "
          >
            <AppSettingsCard
              v-if="app.webAppInfo.web_uis.default.type !== 'Headless'"
              :app="app"
              @openApp="$emit('openApp', $event)"
              @uninstallApp="$emit('uninstall-app', $event)"
              @disableApp="$emit('disable-app', $event)"
              @enableApp="$emit('enable-app', $event)"
              @startApp="$emit('startApp', $event)"
              @updateGui="openUpdateGuiDialog($event)"
            />
          </div>
        </div>

        <!-- Headless Apps -->
        <div
          class="row section-title"
          :class="{ borderBottomed: showHeadlessApps }"
        >
          <span
            style="margin-left: 10px; font-size: 23px; color: rgba(0, 0, 0, 0.6)"
            :title="$t('main.headlessAppsHelper')"
            >{{ $t("main.headlessApps") }}</span
          >
          <span
            @click="showHeadlessApps = !showHeadlessApps"
            class="show-hide"
            style="opacity: 0.7; cursor: pointer; margin-left: 10px"
          >
            {{ showHeadlessApps ? "[-]" : "[show]" }}
          </span>
        </div>
        <div v-if="showHeadlessApps" style="margin-bottom: 50px; width: 100%">
          <div
            v-if="noHeadlessApps"
            style="margin-top: 30px; color: rgba(0, 0, 0, 0.6); text-align: center"
          >
            {{ $t("main.noHeadlessApps") }}
            {{
              selectedHolochainVersion === "All Versions"
                ? "."
                : " in this Holochain Version."
            }}
          </div>
          <div
            v-for="app in sortedApps"
            :key="app.webAppInfo.installed_app_info.installed_app_id"
            style="
              display: flex;
              flex-direction: column;
              width: 100%;
              align-items: center;
            "
          >
            <AppSettingsCard
              v-if="app.webAppInfo.web_uis.default.type === 'Headless'"
              :app="app"
              @openApp="$emit('openApp', $event)"
              @uninstallApp="$emit('uninstall-app', $event)"
              @disableApp="$emit('disable-app', $event)"
              @enableApp="$emit('enable-app', $event)"
            />
          </div>
        </div>  
      </div>
    </div>
  </div>

  <!-- Dialogs -->
  <HCDialog ref="devHubDevsOnlyWarning">
    <div
      class="column"
      style="padding: 30px; align-items: center; max-width: 500px"
    >
      <div style="font-weight: 600; font-size: 27px; margin-bottom: 25px">
        DevHub
      </div>
      <div>
        DevHub is the place where <span style="font-weight: bold; white-space: nowrap;">app developers</span> can upload their apps such that they appear in the App Library.<br><br>
        If you instead want to install other apps, click on the <span style="font-weight: bold; white-space: nowrap;">"Install New App"</span> button in the bottom right corner of the
        main window. It will lead you to the <span style="font-weight: bold; white-space: nowrap;">App Library</span>.
      </div>

      <div class="row" style="margin-top: 30px; margin-bottom: 10px; margin-left: 50px; width: 100%;">
        <ToggleSwitch
          :sliderOn="ignoreDevHubWaring"
          @click="() => ignoreDevHubWaring = !ignoreDevHubWaring"
          @keydown.enter="() => ignoreDevHubWaring = !ignoreDevHubWaring"
        />
        <span style="margin-left: 10px;">Don't show this message again.</span>
      </div>

      <div class="row" style="margin-top: 20px;">
        <HCButton style="height: 30px; margin: 4px 6px;" outlined @click="closeDevHubNote">Cancel</HCButton>
        <HCButton style="margin: 4px 6px;" @click="handleOpenDevHub">Open DevHub</HCButton>
      </div>
    </div>
  </HCDialog>

  <HCSnackbar leading :labelText="snackbarText" ref="snackbar"></HCSnackbar>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { ActionTypes } from "../store/actions";
import { HolochainAppInfo, HolochainAppInfoExtended, HolochainId, InstalledWebAppInfo, StorageInfo } from "../types";
import "@material/mwc-icon";
import { invoke } from "@tauri-apps/api/tauri";
import HCButton from "../components/subcomponents/HCButton.vue";
import HCSnackbar from "../components/subcomponents/HCSnackbar.vue";
import HCDialog from "../components/subcomponents/HCDialog.vue";
import ToggleSwitch from "../components/subcomponents/ToggleSwitch.vue";
import LoadingDots from "../components/subcomponents/LoadingDots.vue";
import { i18n } from "../locale";
import { uniq } from "lodash-es";

import "@material/mwc-button";
import "@material/mwc-icon-button";
import "@material/mwc-icon";

import { isAppDisabled, isAppPaused, isAppRunning } from "../utils";
import AppSettingsCard from "../components/AppSettingsCard.vue";
import HCSelectCard from "../components/subcomponents/HCSelectCard.vue";
import StackedChart from "../components/subcomponents/StackedChart.vue";
import HCGenericDialog from "../components/subcomponents/HCGenericDialog.vue";
import HCLoading from "../components/subcomponents/HCLoading.vue";
import prettyBytes from "pretty-bytes";
import { getHappReleasesByEntryHashes, fetchGui, appstoreCells, fetchGuiReleaseEntry } from "../appstore/appstore-interface";
import { AppInfo, AppWebsocket, decodeHashFromBase64, encodeHashToBase64, EntryHash } from "@holochain/client";
import { GUIReleaseEntry, HappReleaseEntry } from "../appstore/types";
import { APPSTORE_APP_ID, DEVHUB_APP_ID } from "../constants";

export default defineComponent({
  name: "Settings",
  components: { 
    HCButton, 
    HCSnackbar, 
    HCDialog, 
    ToggleSwitch, 
    LoadingDots,
    AppSettingsCard,
    HCSelectCard,
    StackedChart,
    HCGenericDialog,
    HCLoading
  },
  props: {
    installedApps: {
      type: Object as PropType<Array<HolochainAppInfo>>,
      required: true,
    },
  },
  data(): {
    appWebsocket: AppWebsocket | undefined;
    appstoreAppInfo: AppInfo | undefined;
    devHubAppInfo: HolochainAppInfo | undefined;
    devModeEnabled: boolean;
    errorText: string;
    extendedAppInfos: Array<HolochainAppInfoExtended> | undefined;
    howToPublishUrl: string;
    ignoreDevHubWaring: boolean;
    installingDevHub: boolean;
    loadingText: string;
    refreshing: boolean;
    refreshTimeout: number | null;
    reportIssueUrl: string;
    selectedApp: HolochainAppInfoExtended | undefined;
    selectedGuiUpdate: GUIReleaseEntry | undefined;
    selectedGuiUpdateHash: EntryHash | undefined;
    selectedHolochainVersion: string;
    showDevHubDevsOnlyWarning: boolean;
    showHeadlessApps: boolean;
    showWebApps: boolean;
    snackbarText: string | undefined;
    sortOptions: [string, string][];
    sortOption: string | undefined;
    storageInfos: Record<string, StorageInfo>;
  } {
    return {
      devHubAppInfo: undefined,
      devModeEnabled: false,
      howToPublishUrl:
        "https://github.com/holochain/launcher#publishing-and-updating-an-app-in-the-devhub",
      snackbarText: undefined,
      reportIssueUrl: "https://github.com/holochain/launcher/issues/new",
      showDevHubDevsOnlyWarning: false,
      ignoreDevHubWaring: false,
      installingDevHub: false,
      appWebsocket: undefined,
      appstoreAppInfo: undefined,
      sortOptions: [
        [i18n.global.t('main.name'), "name"],
        [i18n.global.t('main.nameDescending'), "name descending"],
        // ["Holochain Version", "Holochain Version"],
      ],
      sortOption: undefined,
      selectedHolochainVersion: "All Versions",
      showHeadlessApps: true,
      showWebApps: true,
      storageInfos: {},
      refreshing: false,
      refreshTimeout: null,
      extendedAppInfos: undefined,
      selectedApp: undefined,
      selectedGuiUpdate: undefined,
      selectedGuiUpdateHash: undefined,
      loadingText: "",
      errorText: "Unknown error occured",
    };
  },
  emits: ["openApp", "uninstall-app", "enable-app", "disable-app", "startApp", "open-app-store"],
  async mounted() {
    console.log("mounted settings")
    await Promise.all(
      this.installedApps.map(async (app) => {
        // Check if DevHub is installed and if so store info about it locally
        if (app.webAppInfo.installed_app_info.installed_app_id === DEVHUB_APP_ID) {
          this.devHubAppInfo = app
        }

        return this.storageInfos[app.holochainVersion] = await invoke(
          "get_storage_info",
          { holochainId: app.holochainId }
        );
      })
    );

    const holochainId = this.$store.getters["holochainIdForDevhub"];
    // connect to AppWebsocket
    const port = this.$store.getters["appInterfacePort"](holochainId);
    // TODO: check why post is not available
    console.log("porttt", port)
    const appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`, 40000);
    this.appWebsocket = appWebsocket;
    // TODO add correct installed app id here.
    const appstoreAppInfo = await appWebsocket.appInfo({
      installed_app_id: APPSTORE_APP_ID,
    });
    this.appstoreAppInfo = appstoreAppInfo;

    await this.checkForUiUpdates();
  },
  computed: {
    // installedApps: {
    //   get(): Array<HolochainAppInfo> {
    //     console.log("getting installed apps")
    //     return this.$store.getters["allApps"];
    //   },
    //   set() {
    //     // Can't set 
    //   }
    // },
    sortedApps() {
      // if extended happ releases are not yet fetched from the DevHub to include potential
      // GUI updates, just return installedApps with guiUpdateAvailable undefined
      console.log("sorted apps, installed = ", this.installedApps, "extended = ", this.extendedAppInfos)
      let sortedAppList: Array<HolochainAppInfoExtended> = this.extendedAppInfos ? this.extendedAppInfos : this.installedApps.map((app) => {
        return {
          webAppInfo: app.webAppInfo,
          holochainId: app.holochainId,
          holochainVersion: app.holochainVersion,
          guiUpdateAvailable: undefined,
        }
      });

      if (this.selectedHolochainVersion !== "All Versions") {
        sortedAppList = sortedAppList.filter(
          (app) => app.holochainVersion === this.selectedHolochainVersion
        );
      }

      if (this.sortOption === "name") {
        sortedAppList = sortedAppList.sort((appA, appB) =>
          appA.webAppInfo.installed_app_info.installed_app_id.localeCompare(
            appB.webAppInfo.installed_app_info.installed_app_id
          )
        );
      } else if (this.sortOption === "name descending") {
        sortedAppList = sortedAppList.sort((appA, appB) =>
          appB.webAppInfo.installed_app_info.installed_app_id.localeCompare(
            appA.webAppInfo.installed_app_info.installed_app_id
          )
        );
      } else {
        // default is alphabetical by app id
        sortedAppList = sortedAppList.sort((appA, appB) =>
          appA.webAppInfo.installed_app_info.installed_app_id.localeCompare(
            appB.webAppInfo.installed_app_info.installed_app_id
          )
        );
      }

      return sortedAppList;
    },
    noHeadlessApps(): boolean {
      return !this.sortedApps.some(
        (app) => app.webAppInfo.web_uis.default.type === "Headless"
      );
    },
    noWebApps(): boolean {
      return this.sortedApps.every(
        (app) => app.webAppInfo.web_uis.default.type === "Headless"
      );
    },
    noHolochainVersions(): boolean {
      return this.noWebApps && this.noHeadlessApps;
    },
    holochainVersions(): string[] {
      const allApps = this.installedApps;
      return uniq(allApps.map((app) => app.holochainVersion));
    },
    holochainVersionOptions(): [string, string][] {
      let allApps = this.installedApps;
      let hcVersions: [string, string][] = [[i18n.global.t('main.allVersions'), "All Versions"]];
      uniq(allApps.map((app) => app.holochainVersion)).forEach((hcVer) => {
        hcVersions.push([hcVer, hcVer]);
      });
      return hcVersions;
    },
  },
  methods: {
    isAppDisabled,
    isAppPaused,
    isAppRunning,
    prettyBytes,
    isLoading() {
      return this.$store.state.launcherStateInfo === "loading";
    },
    closeDevHubNote() {
      if (this.ignoreDevHubWaring) {
        window.localStorage.setItem("ignoreDevHubDevsOnlyWarning", "true");
      }
      (this.$refs["devHubDevsOnlyWarning"] as typeof HCDialog).close();
    },
    async handleOpenDevHub() {
      if (this.ignoreDevHubWaring) {
        window.localStorage.setItem("ignoreDevHubDevsOnlyWarning", "true");
      }
      const appId = this.devHubAppInfo!.webAppInfo.installed_app_info.installed_app_id;
      (this.$refs["devHubDevsOnlyWarning"] as typeof HCDialog).close();
      try {
        await invoke("open_app_ui", { appId, holochainId: this.devHubAppInfo!.holochainId });
        this.showMessage(`App ${appId} opened`);
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async howToPublish() {
      await invoke("open_url_cmd", {
        url: this.howToPublishUrl,
      });
    },
    async openApp(app: HolochainAppInfo) {
      // if the DevHub is requested to be opened, show a warning dialog that
      // this is intended for developers

      if ((app.webAppInfo.installed_app_info.installed_app_id == `DevHub-${app.holochainId.content}`)
       && (!window.localStorage.ignoreDevHubDevsOnlyWarning)) {
        (this.$refs["devHubDevsOnlyWarning"] as typeof HCDialog).open();
        return;
      }

      const appId = app.webAppInfo.installed_app_info.installed_app_id;
      try {
        await invoke("open_app_ui", { appId, holochainId: app.holochainId });
        this.showMessage(`App ${appId} opened`);
      } catch (e) {
        const error = `Error opening app ${appId}: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async disableApp(app: HolochainAppInfo) {
      const appId = app.webAppInfo.installed_app_info.installed_app_id;
      try {
        await invoke("disable_app", { appId, holochainId: app.holochainId });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.showMessage(`Disabled ${appId}`);
      } catch (e) {
        const error = `Disable app ${appId} failed: ${JSON.stringify(e)}`;

        // if disabling "purportedly" fails due to being offline, ignore the error.
        if (error.includes("failed to lookup address information: Temporary failure in name resolution")) {
          this.showMessage(`Disabled ${appId}`);
        } else {
          this.showMessage(error);
        }
        await invoke("log", {
          log: error,
        });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      }
    },
    async enableApp(app: HolochainAppInfo) {
      const appId = app.webAppInfo.installed_app_info.installed_app_id;

      try {
        await invoke("enable_app", { appId, holochainId: app.holochainId });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.showMessage(`Enabled ${appId}`);
      } catch (e) {
        const error = `Enable app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async startApp(app: HolochainAppInfo) {
      // console.log("@InstalledApps: RECEIVED REQUEST TO START APP.");
      const appId = app.webAppInfo.installed_app_info.installed_app_id;
      // console.log("@InstalledApps: @startApp: appId: ", appId);

      // StartApp is not available anymore in conductor API since 0.1.0-beta-rc.4: https://github.com/holochain/holochain/blob/develop/crates/holochain_conductor_api/CHANGELOG.md#010-beta-rc4
      // instead disable app followed by enable app:
      try {
        // console.log("@InstalledApps: @startApp: disabling app.");

        await invoke("disable_app", { appId, holochainId: app.holochainId });
        // console.log("@InstalledApps: @startApp: app disabled, enabling app.");

        await invoke("enable_app", { appId, holochainId: app.holochainId });
        // console.log("@InstalledApps: @startApp: app enabled.");

        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Started ${appId}`);
      } catch (e) {
        const error = `Start app ${appId} failed: ${JSON.stringify(e)}`;
        console.error(error);
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      }
    },
    async uninstallApp(app: HolochainAppInfo) {
      const appId = app.webAppInfo.installed_app_info.installed_app_id;

      try {
        await invoke("uninstall_app", { appId, holochainId: app.holochainId });

        await this.$store.dispatch(ActionTypes.fetchStateInfo);

        this.showMessage(`Uninstalled ${appId}`);
      } catch (e) {
        const error = `Uninstall app ${appId} failed: ${JSON.stringify(e)}`;
        this.showMessage(error);
        await invoke("log", {
          log: error,
        });
      }
    },
    async toggleDevMode() {
      // TODO: track devModeEnabled in Tauri so it can be used all over the app?
      if (this.devHubAppInfo && (isAppRunning(this.devHubAppInfo.webAppInfo.installed_app_info) || isAppPaused(this.devHubAppInfo.webAppInfo.installed_app_info))) {
        await this.disableApp(this.devHubAppInfo);
      } else {
        if (!this.devHubAppInfo) {
          this.installingDevHub = true;
          try {
            await invoke("install_devhub", {});
            this.installingDevHub = false;
            window.location.reload();
          } catch (e) {
            alert(`Failed to install DevHub: ${JSON.stringify(e)}`);
            console.error(`Failed to install DevHub: ${JSON.stringify(e)}`);
            this.installingDevHub = false;
          }
        } else {
          this.enableApp(this.devHubAppInfo);
        }
      }
    },
    async reportIssue() {
      await invoke("open_url_cmd", {
        url: this.reportIssueUrl,
      });
    },
    isAppHeadless(app: HolochainAppInfo) {
      return app.webAppInfo.web_uis.default.type === "Headless";
    },
    async checkForUiUpdates() {
      console.log("Checking for UI updates...");
      // check for GUI updates
      const allApps: Array<HolochainAppInfo> = this.$store.getters["allApps"];
      const allHappReleaseHashes = allApps.map((app) => app.webAppInfo.happ_release_hash ? decodeHashFromBase64(app.webAppInfo.happ_release_hash) : undefined);
      // console.log("@InstalledAppsList: allHappReleaseHashes from store's allApps: ", allHappReleaseHashes);
      const happReleases: Array<HappReleaseEntry | undefined> = await getHappReleasesByEntryHashes((this.appWebsocket! as AppWebsocket), this.appstoreAppInfo!, allHappReleaseHashes);

      console.log("@InstalledAppsList: happReleases: ", happReleases);

      // compare with existing

      const extendedAppInfos: Array<HolochainAppInfoExtended> = allApps.map((appInfo: HolochainAppInfo, idx) => {

        if (happReleases[idx]) {
          console.log("official_gui: ", happReleases[idx]!.official_gui ? encodeHashToBase64(happReleases[idx]!.official_gui!) : undefined)
        }

        const isGuiUpdateAvailable = (appInfo.webAppInfo.web_uis.default.type === "WebApp" && happReleases[idx]?.official_gui)
          ? appInfo.webAppInfo.web_uis.default.gui_release_hash != encodeHashToBase64(happReleases[idx]?.official_gui!)
          : false

        return {
          webAppInfo: appInfo.webAppInfo,
          holochainId: appInfo.holochainId,
          holochainVersion: appInfo.holochainVersion,
          guiUpdateAvailable: isGuiUpdateAvailable ? happReleases[idx]?.official_gui : undefined,
        }
      });

      console.log("@InstalledAppsLlist: extendedAppInfos: ", extendedAppInfos);

      this.extendedAppInfos = extendedAppInfos;
    },
    async openUpdateGuiDialog(app: HolochainAppInfoExtended) {
      this.selectedApp = app;

      // console.log("Gui release hash @openUpdateGuiDialog: ", app.guiUpdateAvailable);
      (this.$refs.updateGuiDialog as typeof HCGenericDialog).open();

      if (this.appWebsocket && this.appstoreAppInfo) {
          const cells = appstoreCells(this.appstoreAppInfo);
        //   const guiReleaseResponse = await this.appWebsocket?.callZome({
        //   cap_secret: null,
        //   cell_id: getCellId(cells.happs.find((c) => "provisioned" in c )!)!,
        //   fn_name: "get_gui_release",
        //   zome_name: "happ_library",
        //   payload: {
        //     id: app.guiUpdateAvailable,
        //   },
        //   provenance: getCellId(cells.happs.find((c) => "provisioned" in c )!)![1],
        // });

        const guiReleaseResponse = await fetchGuiReleaseEntry(this.appWebsocket as AppWebsocket, this.appstoreAppInfo, app.guiUpdateAvailable!);

        this.selectedGuiUpdate = guiReleaseResponse.content;
        console.log("Got GUI Release: ", guiReleaseResponse.content);
      } else {
        alert!("Error: AppWebsocket or Appstore AppInfo undefined.")
        this.selectedGuiUpdate = undefined;
      }
    },
    storageFractions(holochainVersion: string) {
      const storageInfo: StorageInfo = this.storageInfos[holochainVersion];
      if (storageInfo) {
        const totalStorage = this.totalStorage(holochainVersion);
        const fractions = Object.values(storageInfo).map(
          (value: number) => (value / totalStorage!) * 100
        );
        return fractions;
      } else {
        return undefined;
      }
    },
    totalStorage(holochainVersion: string): number | undefined {
      const storageInfo = this.storageInfos[holochainVersion];
      if (storageInfo) {
        return Object.values(storageInfo).reduce(
          (acc, currValue) => acc + currValue
        );
      } else {
        return undefined;
      }
    },
    storageLabels(holochainVersion: string) {
      const storageInfo = this.storageInfos[holochainVersion];
      if (storageInfo) {
        return Object.entries(storageInfo).map(
          ([key, value]) => `${key} (${prettyBytes(value)})`
        );
      } else {
        return undefined;
      }
    },
    async refreshStorageInfo() {
      this.refreshing = true;
      this.refreshTimeout = window.setTimeout(
        () => (this.refreshing = false),
        200
      );
      await Promise.all(
        this.installedApps.map(async (app) => {
          this.storageInfos[app.holochainVersion] = await invoke(
            "get_storage_info",
            { holochainId: app.holochainId }
          );
        })
      );
    },
    totalStorageString(hcVersion: string) {
      const totalStorageBytes = this.totalStorage(hcVersion);
      if (totalStorageBytes) {
        return prettyBytes(totalStorageBytes);
      } else {
        return "?";
      }
    },
    async updateGui() {
      this.loadingText = "Connecting with DevHub";
      (this.$refs.downloading as typeof HCLoading).open();

      this.loadingText = "fetching UI from peer host...";

      let bytes = undefined;

      try {
        bytes = await fetchGui(
          this.appWebsocket! as AppWebsocket,
          this.appstoreAppInfo!,
          this.selectedGuiUpdate!.web_asset_id,
        );
      } catch (e) {
        console.error("Error fetching the UI: ", e);
        this.errorText = `Error fetching the UI: ${e}`;
        (this.$refs.snackbar as typeof HCSnackbar).show();
        (this.$refs.downloading as typeof HCLoading).close();
        return;
      }

      this.loadingText = "Installing...";

      if (bytes) {
        try {
          await invoke("update_default_ui", {
            holochainId: this.selectedApp!.holochainId,
            appId: this.selectedApp!.webAppInfo.installed_app_info.installed_app_id,
            uiZipBytes: bytes,
            guiReleaseHash: encodeHashToBase64(this.selectedApp!.guiUpdateAvailable!),
          });
          this.loadingText = "";
          (this.$refs.downloading as typeof HCLoading).close();
          (this.$refs.updateGuiDialog as typeof HCGenericDialog).close();
          this.selectedGuiUpdate = undefined;
          this.selectedGuiUpdateHash = undefined;

          // to remove the update button:
          await this.$store.dispatch(ActionTypes.fetchStateInfo);
          window.setTimeout(() => this.checkForUiUpdates(), 500);
        } catch (e) {
          console.error("Error updating the UI: ", e);
          this.errorText = `Error updating the UI: ${e}`;

          (this.$refs as any).snackbar.show();
          (this.$refs.downloading as typeof HCLoading).close();
          this.loadingText = "";
        }
      } else {
        console.error("Error updating the UI: Undefined bytes");
        this.errorText = `Error updating the UI: Undefined bytes`;

        (this.$refs as any).snackbar.show();
        (this.$refs.downloading as typeof HCLoading).close();
        this.loadingText = "";
      }
    },
    showMessage(message: string) {
      this.snackbarText = message;
      (this.$refs as any).snackbar.show();
    },
  },
});
</script>
<!-- We don't have scoped styles with classes because it becomes harder to export a reusable library -->

<style scoped>
h2 {
  font-weight: 600; 
  font-size: 1.2em;
}

.section-title {
  margin: 10px;
  padding-bottom: 3px;
  align-items: center;
}

.section-container {
  border-radius: 15px;
  background-color: white;
  padding: 15px;
  box-shadow: 0 0px 5px #9b9b9b;
}

.hc-version {
  align-items: center;
  flex: 1;
  margin-top: 8px;
  padding: 8px 0;
}


</style>
