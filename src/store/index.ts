import { createStore } from "vuex";
import { invoke } from "@tauri-apps/api/tauri";
export interface LauncherAdminState {
  localAppsPorts: {
    loading: boolean;
    portMapping: { [key: string]: number };
  };
}

export default createStore({
  state(): LauncherAdminState {
    return {
      localAppsPorts: {
        loading: false,
        portMapping: {},
      },
    };
  },
  mutations: {
    loadLocalAppsPorts(state: LauncherAdminState) {
      state.localAppsPorts.loading = true;
    },
    setLocalAppsPorts(state: LauncherAdminState, portMapping) {
      state.localAppsPorts.loading = false;
      state.localAppsPorts.portMapping = portMapping;
    },
  },
  actions: {
    async fetchLocalAppsPorts({ commit }) {
      console.log("asdf");
      commit("loadLocalAppsPorts");

      const portMapping = await invoke("get_port_mapping");
      commit("setLocalAppsPorts", portMapping);
    },
  },
  modules: {},
});
