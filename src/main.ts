import "array-flat-polyfill";
import "blob-polyfill";
import { createApp } from "vue";
import "@material/mwc-textfield";
import "@material/mwc-textarea";
import "@material/mwc-fab";
import "@material/mwc-snackbar";
//import "@material/mwc-button";
//import "@material/mwc-dialog";

import { AdminWebsocket, AppWebsocket } from "@holochain/conductor-api";
import HcAdminPlugin from "@holochain/admin-ui";
import { invoke } from "@tauri-apps/api/tauri";

import App from "./App.vue";
import createStore from "./store";
import { ConnectionStatus } from "./types";

window.onerror = function (message, source, lineno, colno, error) {
  invoke("log", {
    log: `UI error: message: ${message}. source: ${source}. lineno: ${lineno}. colno: ${colno}. error: ${JSON.stringify(
      error
    )}`,
  });
};

async function setup() {
  const app = createApp(App);

  let status = await getConnectionStatus();
  let store = createStore(status) as any;

  if (status.type === "Connected") {
    try {
      const { appWebsocket, adminWebsocket } = await connect(
        status.admin_interface_port
      );

      app.use(HcAdminPlugin as any, { store, appWebsocket, adminWebsocket });

      await invoke("log", {
        log: `Connected to Holochain, Admin port = ${status.admin_interface_port}, App URL = ${appWebsocket.client.socket.url}`,
      });
    } catch (e) {
      const error = `Error connecting to Holochain: ${e}`;
      status = {
        type: "Error",
        error,
      };
      store = createStore(status);
      await invoke("log", { log: error });
    }
  }

  app.use(store);
  app.mount("#app");
}

async function getConnectionStatus(): Promise<ConnectionStatus> {
  try {
    const state: ConnectionStatus = await invoke("get_connection_status", {});
    return state;
  } catch (e) {
    return {
      type: "Error",
      error: `Error getting the connection status: ${e}`,
    };
  }
}

async function connect(adminPort: number): Promise<{
  appWebsocket: AppWebsocket;
  adminWebsocket: AdminWebsocket;
}> {
  const adminWebsocket = await AdminWebsocket.connect(
    `ws://localhost:${adminPort}`
  );

  const appInterfaces = await adminWebsocket.listAppInterfaces();

  const port = appInterfaces[0];

  const appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`);
  return {
    appWebsocket,
    adminWebsocket,
  };
}

setup();
