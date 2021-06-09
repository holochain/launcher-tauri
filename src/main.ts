import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import { AdminWebsocket, AppWebsocket } from "@holochain/conductor-api";
import HcAdminPlugin from "@holochain/admin-ui";
import { ADMIN_PORT, DEFAULT_APP_PORT } from "./constants";

async function setup() {
  const adminWebsocket = await AdminWebsocket.connect(
    `ws://localhost:${ADMIN_PORT}`
  );

  const appInterfaces = await adminWebsocket.listAppInterfaces();

  const port = appInterfaces[0];

  const appWebsocket = await AppWebsocket.connect(`ws://localhost:${port}`);

  createApp(App)
    .use(store)
    .use(router)
    .use(HcAdminPlugin, { store, appWebsocket, adminWebsocket })
    .mount("#app");

  store.commit(
    "log",
    `Connected to Holochain, Admin port = ${ADMIN_PORT}, App port = ${port}`
  );
}

setup();
