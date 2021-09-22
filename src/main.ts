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
import { ADMIN_PORT } from "./constants";

async function setup() {
  const app = createApp(App);
  debugger;
  try {
    const adminWebsocket = await AdminWebsocket.connect(
      `ws://localhost:${ADMIN_PORT}`
    );

    const appInterfaces = await adminWebsocket.listAppInterfaces();

    const port = appInterfaces[0];

    const appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`);

    const store = createStore(true);
    app.use(store as any);
    app
      .use(HcAdminPlugin as any, { store, appWebsocket, adminWebsocket })
      .mount("#app");

    await invoke("log", {
      log: `Connected to Holochain, Admin port = ${ADMIN_PORT}, App port = ${port}`,
    });
  } catch (e) {
    const error = `Error connecting to Holochain: ${e}`;

    await invoke("log", { log: error });
    app.use(createStore(false) as any);
    app.mount("#app");
  }
}

setup();
