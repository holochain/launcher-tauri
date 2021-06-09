import { createStore } from "vuex";

export interface LauncherAdminState {
  logs: Array<string>;
}

export default createStore({
  state(): LauncherAdminState {
    return {
      logs: [],
    };
  },
  mutations: {
    log(state: LauncherAdminState, log: string) {
      state.logs.push(`${new Date().toLocaleTimeString()} - ${log}`);
    },
  },
  actions: {},
  modules: {},
});
