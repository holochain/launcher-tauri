import { createStore } from "vuex";

export interface LauncherAdminState {
  logs: { [key: number]: string };
}

export default createStore({
  state(): LauncherAdminState {
    return {
      logs: {},
    };
  },
  mutations: {
    log(state: LauncherAdminState, logData: { log: string; date?: number }) {
      state.logs[logData.date || Date.now()] = logData.log;
    },
  },
  actions: {},
  modules: {},
});
