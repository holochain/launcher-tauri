<template>
  <div style="flex: 1; display: flex">
    <AlreadyRunning v-if="$store.getters[`isAlreadyRunning`]"> </AlreadyRunning>
    <Error
      v-else-if="
        $store.getters[`errorLaunching`] &&
        !$store.getters[`databaseFileTypeError`] &&
        !$store.getters[`addressAlreadyInUseError`]
      "
      heading="Error Launching Holochain"
      offerRestart
    >
      <div
        style="
          margin-top: 8px;
          display: flex;
          flex: 1;
          background: rgba(255, 0, 0, 0.1);
          overflow-x: scroll;
          border-radius: 5px;
          padding: 10px;
          max-width: 600px;
        "
      >
        {{ $store.getters[`errorLaunching`] }}
      </div>
    </Error>
    <Error
      v-else-if="$store.getters[`addressAlreadyInUseError`]"
      heading="Error Launching Holochain"
      offerQuit
    >
      <div class="column" style="align-items: center">
        <div style="font-weight: bold; margin-bottom: 20px; font-size: 1.1em">
          Websocket address is already in use.
        </div>
        <div style="text-align: left; margin-bottom: 25px">
          This error usually occurs when the Launcher has earlier crashed
          unexpectedly or has been force quit manually, which leads to
          <strong>one or more orphan holochain processes</strong> still running
          in the background.<br />
        </div>
        <div
          style="
            text-align: center;
            background-color: #9fe09d;
            border-radius: 8px;
            padding: 10px 15px;
          "
        >
          To fix it, <strong>quit the Holochain Launcher</strong> and
          <strong>manually kill all running processes</strong> that start with
          <span
            style="
              font-family: monospace;
              font-size: 1.1em;
              background-color: rgba(0, 0, 0, 0.2);
              padding: 2px 6px;
              border-radius: 5px;
              white-space: nowrap;
            "
            >holochain-v</span
          > and <span
            style="
              font-family: monospace;
              font-size: 1.1em;
              background-color: rgba(0, 0, 0, 0.2);
              padding: 2px 6px;
              border-radius: 5px;
              white-space: nowrap;
            "
            >lair-keystore-v</span
          >.
        </div>
        <div style="text-align: left; margin-top: 25px">
          You can kill running processes by opening the Task-Manager (Windows,
          Ctrl+Alt+Delete) or Activity Monitor (macOS) and search for the
          corresponding processes.<br />
        </div>
      </div>
    </Error>
    <Setup v-else-if="$store.getters[`setupNeeded`]"></Setup>
    <EnterPassword v-else-if="$store.getters[`passwordNeeded`]"></EnterPassword>
    <Home
      v-else-if="
        !$store.getters['oldFiles'] && !$store.getters[`databaseFileTypeError`]
      "
      style="display: flex; flex: 1"
    ></Home>
    <About></About>
    <Config></Config>
    <HCGenericDialog
      @confirm="restartLauncher"
      ref="restartDialog"
      :primaryButtonLabel="$t('buttons.restart')"
      :closeOnSideClick="true"
    >
      <div style="margin: 0 30px; max-width: 500px; text-align: center">
        {{ $t("dialogs.restart") }}
      </div>
    </HCGenericDialog>
    <FactoryReset></FactoryReset>
  </div>
</template>
<script lang="ts">
import Home from "./views/Home.vue";
import FactoryReset from "./views/FactoryReset.vue";
import Config from "./components/settings/Config.vue";
import Error from "./components/settings/Error.vue";
import AlreadyRunning from "./components/settings/AlreadyRunning.vue";
import EnterPassword from "./components/setup/EnterPassword.vue";
import Setup from "./components/setup/Setup.vue";
import About from "./components/settings/About.vue";
import HCGenericDialog from "./components/subcomponents/HCGenericDialog.vue";

import { defineComponent } from "vue";
import { ActionTypes } from "./store/actions";
import "@material/mwc-circular-progress";
import "@fontsource/poppins";
import "@fontsource/poppins/600.css";
import "@fontsource/poppins/700.css";
import "@fontsource/poppins/800.css";
import "@fontsource/poppins/900.css";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  name: "App",
  components: {
    EnterPassword,
    Setup,
    Home,
    FactoryReset,
    About,
    Error,
    Config,
    AlreadyRunning,
    HCGenericDialog,
  },
  mounted() {
    // define window.__HC_LAUNCHER_ENV__ so that js-client routes zome-call signing to tauri
    (window as any).__HC_LAUNCHER_ENV__ = {};
    this.$nextTick(async () => {
      const restartDialog = this.$refs.restartDialog as typeof HCGenericDialog;
      await listen("request-restart", () => {
        restartDialog.open();
      });
    });
  },
  async created() {
    await this.$store.dispatch(ActionTypes.fetchStateInfo);
  },
  methods: {
    async restartLauncher() {
      await invoke("restart");
    },
  },
});
</script>
<style>
html,
body,
#app {
  margin: 0;
  display: flex;
  flex: 1;
  height: 100%;
  font-family: Poppins, sans-serif;
  /* background-color: #ededed; */
  background-color: #e8e8eb;
}

:root {
  font-family: Poppins, sans-serif;
  --mdc-theme-primary: #4720e3;
  --mdc-theme-secondary: #0dddd3;
}

#app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}

#nav {
  padding: 30px;
}

#nav a {
  font-weight: bold;
  color: #2c3e50;
}

#nav a.router-link-exact-active {
  color: #42b983;
}

.column {
  display: flex;
  flex-direction: column;
}
.row {
  display: flex;
  flex-direction: row;
}

.center-content {
  align-items: center;
  justify-content: center;
}

.title {
  font-size: 1.5em;
}

.flex-scrollable-parent {
  position: relative;
  display: flex;
  flex: 1;
}
.flex-scrollable-container {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}
.flex-scrollable-x {
  max-width: 100%;
  overflow-x: auto;
}
.flex-scrollable-y {
  max-height: 100%;
  overflow-y: auto;
}

mwc-button,
mwc-icon-button {
  --mdc-theme-primary: rgb(90, 90, 90);
}

@font-face {
  font-family: "Material Icons";
  font-style: normal;
  font-weight: 400;
  src: url(/fonts/icons/MaterialIcons-Regular.eot); /* For IE6-8 */
  src: local("Material Icons"), local("MaterialIcons-Regular"),
    url(/fonts/icons/MaterialIcons-Regular.ttf) format("truetype");
}

/* mulish-300 - latin */
@font-face {
  font-family: "Mulish";
  font-style: normal;
  font-weight: 300;
  src: url("/fonts/mulish/mulish-v5-latin-300.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-300.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-300.woff2") format("woff2"),
    /* Super Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-300.woff")
      format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-300.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-300.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-regular - latin */
@font-face {
  font-family: "Mulish";
  font-style: normal;
  font-weight: 400;
  src: url("/fonts/mulish/mulish-v5-latin-regular.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-regular.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-regular.woff2")
      format("woff2"),
    /* Super Modern Browsers */
      url("/fonts/mulish/mulish-v5-latin-regular.woff") format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-regular.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-regular.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-500 - latin */
@font-face {
  font-family: "Mulish";
  font-style: normal;
  font-weight: 500;
  src: url("/fonts/mulish/mulish-v5-latin-500.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-500.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-500.woff2") format("woff2"),
    /* Super Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-500.woff")
      format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-500.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-500.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-600 - latin */
@font-face {
  font-family: "Mulish";
  font-style: normal;
  font-weight: 600;
  src: url("/fonts/mulish/mulish-v5-latin-600.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-600.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-600.woff2") format("woff2"),
    /* Super Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-600.woff")
      format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-600.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-600.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-700 - latin */
@font-face {
  font-family: "Mulish";
  font-style: normal;
  font-weight: 700;
  src: url("/fonts/mulish/mulish-v5-latin-700.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-700.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-700.woff2") format("woff2"),
    /* Super Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-700.woff")
      format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-700.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-700.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-800 - latin */
@font-face {
  font-family: "Mulish";
  font-style: normal;
  font-weight: 800;
  src: url("/fonts/mulish/mulish-v5-latin-800.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-800.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-800.woff2") format("woff2"),
    /* Super Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-800.woff")
      format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-800.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-800.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-900 - latin */
@font-face {
  font-family: "Mulish";
  font-style: normal;
  font-weight: 900;
  src: url("/fonts/mulish/mulish-v5-latin-900.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-900.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-900.woff2") format("woff2"),
    /* Super Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-900.woff")
      format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-900.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-900.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-italic - latin */
@font-face {
  font-family: "Mulish";
  font-style: italic;
  font-weight: 400;
  src: url("/fonts/mulish/mulish-v5-latin-italic.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-italic.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-italic.woff2")
      format("woff2"),
    /* Super Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-italic.woff")
      format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-italic.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-italic.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-500italic - latin */
@font-face {
  font-family: "Mulish";
  font-style: italic;
  font-weight: 500;
  src: url("/fonts/mulish/mulish-v5-latin-500italic.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-500italic.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-500italic.woff2")
      format("woff2"),
    /* Super Modern Browsers */
      url("/fonts/mulish/mulish-v5-latin-500italic.woff") format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-500italic.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-500italic.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-600italic - latin */
@font-face {
  font-family: "Mulish";
  font-style: italic;
  font-weight: 600;
  src: url("/fonts/mulish/mulish-v5-latin-600italic.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-600italic.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-600italic.woff2")
      format("woff2"),
    /* Super Modern Browsers */
      url("/fonts/mulish/mulish-v5-latin-600italic.woff") format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-600italic.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-600italic.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-700italic - latin */
@font-face {
  font-family: "Mulish";
  font-style: italic;
  font-weight: 700;
  src: url("/fonts/mulish/mulish-v5-latin-700italic.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-700italic.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-700italic.woff2")
      format("woff2"),
    /* Super Modern Browsers */
      url("/fonts/mulish/mulish-v5-latin-700italic.woff") format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-700italic.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-700italic.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-800italic - latin */
@font-face {
  font-family: "Mulish";
  font-style: italic;
  font-weight: 800;
  src: url("/fonts/mulish/mulish-v5-latin-800italic.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-800italic.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-800italic.woff2")
      format("woff2"),
    /* Super Modern Browsers */
      url("/fonts/mulish/mulish-v5-latin-800italic.woff") format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-800italic.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-800italic.svg#Mulish") format("svg"); /* Legacy iOS */
}
/* mulish-900italic - latin */
@font-face {
  font-family: "Mulish";
  font-style: italic;
  font-weight: 900;
  src: url("/fonts/mulish/mulish-v5-latin-900italic.eot"); /* IE9 Compat Modes */
  src: local(""),
    url("/fonts/mulish/mulish-v5-latin-900italic.eot?#iefix")
      format("embedded-opentype"),
    /* IE6-IE8 */ url("/fonts/mulish/mulish-v5-latin-900italic.woff2")
      format("woff2"),
    /* Super Modern Browsers */
      url("/fonts/mulish/mulish-v5-latin-900italic.woff") format("woff"),
    /* Modern Browsers */ url("/fonts/mulish/mulish-v5-latin-900italic.ttf")
      format("truetype"),
    /* Safari, Android, iOS */
      url("/fonts/mulish/mulish-v5-latin-900italic.svg#Mulish") format("svg"); /* Legacy iOS */
}
</style>
