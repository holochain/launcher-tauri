import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import { AdminWebsocket, AppWebsocket } from "@holochain/conductor-api";
import HcAdminPlugin from "@holochain/admin-ui";

async function setup() {
  const adminWebsocket = await AdminWebsocket.connect("ws://localhost:8889");
  await adminWebsocket.attachAppInterface({ port: 8888 });
  const appWebsocket = await AppWebsocket.connect("ws://localhost:8888");

  createApp(App)
    .use(store)
    .use(router)
    .use(HcAdminPlugin, { store, appWebsocket, adminWebsocket })
    .mount("#app");
}

setup();
