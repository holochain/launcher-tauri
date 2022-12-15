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
import { i18n } from "./locale";

window.onerror = function (message, source, lineno, colno, error) {
  invoke("log", {
    log: `UI error: message: ${message}. source: ${source}. lineno: ${lineno}. colno: ${colno}. error: ${JSON.stringify(
      error
    )}`,
  });
};

const defaultLanguage = navigator.language;

// if (defaultLanguage.startsWith("de")) {
//   i18n.global.locale = "de";
// } else {
//   i18n.global.locale = "en";
// }

// always english for now
i18n.global.locale = "en";


const app = createApp(App);

app.use(store);
app.use(i18n);

app.mount("#app");
