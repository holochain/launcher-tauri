import "array-flat-polyfill";
import "blob-polyfill";
import { createApp } from "vue";
import "@material/mwc-textfield";
import "@material/mwc-textarea";
import "@material/mwc-fab";
import "@material/mwc-snackbar";
import "@material/mwc-button";
import "@material/mwc-dialog";

import { invoke } from "@tauri-apps/api/tauri";

import App from "./App.vue";
import { store } from "./store";
import { ActionTypes } from "./store/actions";

window.onerror = function (message, source, lineno, colno, error) {
  invoke("log", {
    log: `UI error: message: ${message}. source: ${source}. lineno: ${lineno}. colno: ${colno}. error: ${JSON.stringify(
      error
    )}`,
  });
};

async function setup() {
  const app = createApp(App);

  app.use(store);
  store.dispatch(ActionTypes.fetchStateInfo);
  app.mount("#app");
}

setup();
