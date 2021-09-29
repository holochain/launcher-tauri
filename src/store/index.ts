import { ConnectionStatus } from "../types";
import { createStore } from "vuex";

export interface LauncherAdminState {
  connectionStatus: ConnectionStatus;
}

export default function (status: ConnectionStatus) {
  return createStore({
    state(): LauncherAdminState {
      return {
        connectionStatus: status,
      };
    },
    mutations: {},
    actions: {},
    modules: {},
  });
}
