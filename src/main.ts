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

  const appWebsocket = await connectAppWebsocket(adminWebsocket);

  createApp(App)
    .use(store)
    .use(router)
    .use(HcAdminPlugin, { store, appWebsocket, adminWebsocket })
    .mount("#app");
}

async function connectAppWebsocket(adminWebsocket: AdminWebsocket) {
  const appInterfaces = await adminWebsocket.listAppInterfaces();

  const port = appInterfaces[0];

  return AppWebsocket.connect(`ws://localhost:${port}`);
}

setup();
