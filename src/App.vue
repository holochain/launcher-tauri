<template>
  <!--   <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link>
  </div>
 -->
  <Home style="display: flex; flex: 1"></Home>
</template>
<script lang="ts">
import Home from "./views/Home.vue";
import { defineComponent } from "vue";

export default defineComponent({
  name: "App",
  components: {
    Home,
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
  --mdc-theme-primary: #4720e3;
  --mdc-theme-secondary: #0dddd3;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
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
</style>
