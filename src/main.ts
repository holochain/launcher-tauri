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


// logic for setting locale
const customLocale = window.localStorage.getItem("customLocale");

if (customLocale) {
  if (i18n.global.availableLocales.includes(customLocale as any)) {
    i18n.global.locale = customLocale as any;
  } else {
    console.warn(`Invalid custom locale found in localStorage: ${customLocale}. Available locales: ${i18n.global.availableLocales}`);
  }
} else {
  // default to the webview's locale which should correspond to the OS locale
  const defaultLocale = navigator.language;
  if (i18n.global.availableLocales.includes(defaultLocale as any)) {
    i18n.global.locale = defaultLocale as any;
  }
}


const app = createApp(App);

app.use(store);
app.use(i18n);

app.mount("#app");
