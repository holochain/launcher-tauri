<template>
  <!--   <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link>
  </div>
 -->
  <Home v-if="isConnected()" style="display: flex; flex: 1"></Home>
  <AlreadyRunning v-else-if="isAlreadyRunning()"></AlreadyRunning>
  <FactoryReset style="display: flex; flex: 1"></FactoryReset>
  <About></About>
</template>
<script lang="ts">
import Home from "./views/Home.vue";
import FactoryReset from "./views/FactoryReset.vue";
import AlreadyRunning from "./components/AlreadyRunning.vue";
import About from "./components/About.vue";
import { defineComponent } from "vue";

export default defineComponent({
  name: "App",
  components: {
    Home,
    FactoryReset,
    About,
    AlreadyRunning,
  },
  methods: {
    isConnected() {
      return this.$store.state.connectionStatus.type === "Connected";
    },
    isAlreadyRunning() {
      return this.$store.state.connectionStatus.type === "AlreadyRunning";
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
  font-family: Mulish, sans-serif;
  background-color: #ededed;
}

:root {
  font-family: Mulish, sans-serif;
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
