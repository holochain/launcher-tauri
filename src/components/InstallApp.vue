<template>
  <div class="row center">
    <span style="margin-right: 24px">Install App</span>

    <span style="margin-right: 8px">hApp file: </span>
    <button
      v-if="!webAppBundlePath"
      style="margin-right: 8px"
      @click="selectWebHappFile()"
    >
      Select Web-hApp bundle
    </button>
    <div v-else class="row center">
      <span style="margin-right: 8px">{{
        pathToFilename(webAppBundlePath)
      }}</span
      ><button @click="webAppBundlePath = undefined" style="margin-right: 8px">
        Remove
      </button>
    </div>

    <button
      :disabled="!webAppBundlePath || !this.appId"
      @click="installApp()"
      style="margin-left: 24px"
    >
      Install
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import AdminUI from "@holochain/admin-ui";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

export default defineComponent({
  name: "InstallApp",
  data(): {
    webAppBundlePath: string | undefined;
    appId: string | undefined;
  } {
    return {
      appId: Date.now().toString(),
      webAppBundlePath: undefined,
    };
  },
  methods: {
    async selectWebHappFile() {
      this.webAppBundlePath = (await open({
        filters: [{ name: "webhapp", extensions: ["webhapp"] }],
      })) as string;
    },
    pathToFilename(path: string) {
      const components = path.split("/");
      return components[components.length - 1];
    },
    async installApp() {
      try {
        await invoke("install_app", {
          appId: this.appId,
          webAppBundlePath: this.webAppBundlePath,
        });

        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.fetchInstalledApps}`
        );

        this.$snackbar.add({
          type: "success",
          text: `Installed hApp ${this.appId}`,
        });
      } catch (e) {
        this.$snackbar.add({
          type: "error",
          text: JSON.stringify(e),
        });
      }

      this.webAppBundlePath = undefined;
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.app-row {
  margin-bottom: 16px;
}

.app-title {
  font-size: 24px;
}

.cell-row {
  margin-top: 8px;
}
.center {
  align-items: center;
  justify-content: center;
}
</style>
