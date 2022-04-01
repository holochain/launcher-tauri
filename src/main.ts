import "@webcomponents/scoped-custom-element-registry";
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

window.onerror = function (message, source, lineno, colno, error) {
  invoke("log", {
    log: `UI error: message: ${message}. source: ${source}. lineno: ${lineno}. colno: ${colno}. error: ${JSON.stringify(
      error
    )}`,
  });
};

const app = createApp(App);

app.use(store);
app.mount("#app");
