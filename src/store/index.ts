import { createStore } from "vuex";

export interface LauncherAdminState {
  isConnected: boolean;
}

export default function (connected: boolean) {
  return createStore({
    state(): LauncherAdminState {
      return {
        isConnected: connected,
      };
    },
    mutations: {},
    actions: {},
    modules: {},
  });
}
