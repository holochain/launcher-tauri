import {
  HolochainAppInfo,
  HolochainId,
  HolochainState,
  LauncherStateInfo,
} from "../types";
import { invoke } from "@tauri-apps/api/tauri";
import { createStore } from "vuex";
import { flatten, uniq } from "lodash-es";

export interface LauncherAdminState {
  launcherStateInfo: "loading" | LauncherStateInfo;
}

export const store = createStore<LauncherAdminState>({
  state() {
    return {
      launcherStateInfo: "loading",
    };
  },
  getters: {
    oldFiles(state) {
      if (state.launcherStateInfo === "loading") return false;
      return (
        state.launcherStateInfo.state.type === "Error" &&
        state.launcherStateInfo.state.content.type === "OldFilesExist"
      );
    },
    /**
     * Checks whether there is already another instance of the Holochain Launcher running
     * @param state
     * @returns boolean
     */
    isAlreadyRunning(state) {
      if (state.launcherStateInfo === "loading") return false;
      return (
        state.launcherStateInfo.state.type === "Error" &&
        state.launcherStateInfo.state.content.type ===
          "AnotherInstanceIsAlreadyRunning"
      );
    },
    /**
     * Checks for errors launching the launcher itself, the keystore
     * or any of the holochain versions
     * @param state
     * @returns string | undefined : if undefined, no error was found or the launcherStateInfo is still loading
     */
    errorLaunching(state) {
      if (state.launcherStateInfo === "loading") return undefined;

      // Errors launching the launcher itself
      if (
        state.launcherStateInfo.state.type === "Error" &&
        state.launcherStateInfo.state.content.type === "ErrorLaunching"
      )
        return state.launcherStateInfo.state.content.content;

      if (
        state.launcherStateInfo.state.type === "Error" &&
        state.launcherStateInfo.state.content.type === "FileSystemError"
      )
        return state.launcherStateInfo.state.content.content;

      // Errors launching the Keystore
      if (
        state.launcherStateInfo.state.type === "Running" &&
        state.launcherStateInfo.state.content.type === "Error" &&
        state.launcherStateInfo.state.content.content.type ===
          "LaunchKeystoreError"
      ) {
        if (
          typeof state.launcherStateInfo.state.content.content.content
            .content === "string"
        ) {
          return state.launcherStateInfo.state.content.content.content.content;
        } else {
          if (
            state.launcherStateInfo.state.content.content.content.content
              .type === "BinaryNotFound"
          ) {
            return "Error finding the tauri binary";
          } else {
            return state.launcherStateInfo.state.content.content.content.content
              .content;
          }
        }
      }

      // Errors launching any of the Holochain versions
      if (
        state.launcherStateInfo.state.type === "Running" &&
        state.launcherStateInfo.state.content.type === "Running"
      ) {
        const c = state.launcherStateInfo.state.content.content;

        const allHolochains = Object.values(c.versions);
        if (c.custom_binary) allHolochains.push(c.custom_binary);

        const error = allHolochains.find((v) => v.type === "Error");
        if (error) return error.content as string;
      }
    },
    databaseFileTypeError(state) {
      if (state.launcherStateInfo === "loading") return undefined;

      if (
        state.launcherStateInfo.state.type === "Running" &&
        state.launcherStateInfo.state.content.type === "Running"
      ) {
        const c = state.launcherStateInfo.state.content.content;

        const allHolochains = Object.values(c.versions);
        if (c.custom_binary) allHolochains.push(c.custom_binary);

        const error = allHolochains.find((v) => v.type === "Error");
        if (
          error &&
          error.content
            .toString()
            .includes(
              "SqliteFailure(Error { code: NotADatabase, extended_code: 26 }"
            )
        ) {
          return true;
        }
      }
    },
    addressAlreadyInUseError(state) {
      if (state.launcherStateInfo === "loading") return undefined;

      if (
        state.launcherStateInfo.state.type === "Running" &&
        state.launcherStateInfo.state.content.type === "Running"
      ) {
        const c = state.launcherStateInfo.state.content.content;

        const allHolochains = Object.values(c.versions);
        if (c.custom_binary) allHolochains.push(c.custom_binary);

        const error = allHolochains.find((v) => v.type === "Error");
        if (
          error &&
          error.content
            .toString()
            .includes(
              "InterfaceError(WebsocketError(Io(Os { code: 98, kind: AddrInUse"
            )
        ) {
          return true;
        }
      }
    },
    holochainIdForDevhub(state) {
      const stateInfo = state.launcherStateInfo;

      if (stateInfo === "loading") return undefined;

      return {
        type: "HolochainVersion",
        content: stateInfo.default_version,
      };
    },
    hdiOfDevhub(state) {
      const stateInfo = state.launcherStateInfo;

      if (stateInfo === "loading") return undefined;

      const defaultVersion = stateInfo.default_version;

      if (stateInfo.state.content.type === "Running") {
        const holochainVersions = stateInfo.state.content.content.versions;
        const devhubHolochainVersion = holochainVersions[defaultVersion];
        if (devhubHolochainVersion.type === "Running") {
          return {
            type: "HdiVersion",
            content: devhubHolochainVersion.content.hdi_version,
          };
        }
      }

      return {
        type: "Error",
        content: "DevHub Holochain version not running.",
      };
    },
    setupNeeded(state) {
      if (state.launcherStateInfo === "loading") return undefined;

      return (
        state.launcherStateInfo.state.type === "Running" &&
        state.launcherStateInfo.state.content.type === "Error" &&
        state.launcherStateInfo.state.content.content.type === "InitNecessary"
      );
    },
    passwordNeeded(state) {
      if (state.launcherStateInfo === "loading") return undefined;

      return (
        state.launcherStateInfo.state.type === "Running" &&
        state.launcherStateInfo.state.content.type === "Error" &&
        state.launcherStateInfo.state.content.content.type ===
          "PasswordNecessary"
      );
    },
    runningHolochainIds(state): Array<HolochainId> {
      const stateInfo = state.launcherStateInfo;

      if (
        stateInfo === "loading" ||
        stateInfo.state.type === "Error" ||
        stateInfo.state.content.type === "Error"
      )
        return [];

      const versions: Array<HolochainId> = Object.keys(
        stateInfo.state.content.content.versions
      )
        .sort((a, b) => b.localeCompare(a))
        .map((v) => ({
          type: "HolochainVersion",
          content: v,
        }));
      if (
        stateInfo.state.content.content.custom_binary &&
        stateInfo.state.content.content.custom_binary.type === "Running"
      ) {
        versions.push({
          type: "CustomBinary",
          content: undefined,
        });
      }

      return versions;
    },
    allApps(state): Array<HolochainAppInfo> {
      const stateInfo = state.launcherStateInfo;

      if (
        stateInfo === "loading" ||
        stateInfo.state.type === "Error" ||
        stateInfo.state.content.type === "Error"
      )
        return [];

      const allInstalledApps: Array<HolochainAppInfo> = [];

      const holochainsStateInfo = stateInfo.state.content.content;

      // add all apps of the custom binary if present
      if (
        holochainsStateInfo.custom_binary &&
        holochainsStateInfo.custom_binary.type === "Running"
      ) {
        holochainsStateInfo.custom_binary.content.installed_apps.forEach(
          (app) => {
            allInstalledApps.push({
              webAppInfo: app,
              holochainId: {
                type: "CustomBinary",
                content: undefined,
              },
              holochainVersion: "Custom Binary",
            });
          }
        );
      }

      // add all other apps
      Object.entries(holochainsStateInfo.versions).forEach(
        ([holochainVersion, holochainState]) => {
          if (holochainState.type === "Running") {
            holochainState.content.installed_apps.forEach((app) => {
              allInstalledApps.push({
                webAppInfo: app,
                holochainId: {
                  type: "HolochainVersion",
                  content: holochainVersion,
                },
                holochainVersion,
              });
            });
          }
        }
      );

      return allInstalledApps;
    },
    appsForHolochain: (state) => (holochainId: HolochainId) => {
      const stateInfo = state.launcherStateInfo;

      if (
        stateInfo === "loading" ||
        stateInfo.state.type === "Error" ||
        stateInfo.state.content.type === "Error"
      )
        return [];

      let holochainState: HolochainState;
      if (holochainId.type === "CustomBinary")
        holochainState = stateInfo.state.content.content
          .custom_binary as HolochainState;
      else
        holochainState =
          stateInfo.state.content.content.versions[holochainId.content];

      if (holochainState.type === "Error") return [];

      // Sort apps alphabetically

      return holochainState.content.installed_apps.sort((app1, app2) => {
        if (
          app1.installed_app_info.installed_app_id <
          app2.installed_app_info.installed_app_id
        ) {
          return -1;
        }
        if (
          app1.installed_app_info.installed_app_id >
          app2.installed_app_info.installed_app_id
        ) {
          return 1;
        }
        return 0;
      });
    },
    allPublicKeysForHolochainId: (state) => (holochainId: HolochainId) => {
      const stateInfo = state.launcherStateInfo;

      if (
        stateInfo === "loading" ||
        stateInfo.state.type === "Error" ||
        stateInfo.state.content.type === "Error"
      )
        return [];

      let holochainState: HolochainState;
      if (holochainId.type === "CustomBinary")
        holochainState = stateInfo.state.content.content
          .custom_binary as HolochainState;
      else
        holochainState =
          stateInfo.state.content.content.versions[holochainId.content];

      if (!holochainState || holochainState.type === "Error") return [];

      const allCells = flatten(
        holochainState.content.installed_apps.map(
          (app) => app.installed_app_info.cell_data
        )
      );

      return uniq(allCells.map((c) => c.cell_id[1]));
    },
    appInterfacePort: (state) => (holochainId: HolochainId) => {
      const stateInfo = state.launcherStateInfo;

      if (
        stateInfo === "loading" ||
        stateInfo.state.type === "Error" ||
        stateInfo.state.content.type === "Error"
      )
        return undefined;

      let holochainState: HolochainState;
      if (holochainId.type === "CustomBinary")
        holochainState = stateInfo.state.content.content
          .custom_binary as HolochainState;
      else
        holochainState =
          stateInfo.state.content.content.versions[holochainId.content];

      if (holochainState.type === "Error") return undefined;

      return holochainState.content.app_interface_port;
    },
  },
  mutations: {
    loadStateInfo(state) {
      state.launcherStateInfo = "loading";
    },
    setStateInfo(state, StateInfo) {
      state.launcherStateInfo = StateInfo;
    },
  },
  actions: {
    async fetchStateInfo({ commit }) {
      commit("loadStateInfo");
      const StateInfo = await invoke("get_state_info", {});

      commit("setStateInfo", StateInfo);
    },
  },
});
