<template>
  <div class="row center">
    <span style="margin-right: 24px">Install App</span>

    <span style="margin-right: 8px">hApp file: </span>
    <button
      v-if="!happBundlePath"
      style="margin-right: 8px"
      @click="selectHappFile()"
    >
      Select hApp bundle
    </button>
    <div v-else class="row center">
      <span style="margin-right: 8px">{{ pathToFilename(happBundlePath) }}</span
      ><button @click="happBundlePath = undefined" style="margin-right: 8px">
        Remove
      </button>
    </div>

    <span style="margin-right: 8px; margin-left: 8px">UI file: </span>
    <button
      v-if="!uiBundlePath"
      style="margin-right: 8px"
      @click="selectUIFile()"
    >
      Select UI bundle
    </button>
    <div v-else class="row center">
      <span style="margin-right: 8px">{{ pathToFilename(uiBundlePath) }}</span
      ><button @click="uiBundlePath = undefined" style="margin-right: 8px">
        Remove
      </button>
    </div>

    <button
      :disabled="!happBundlePath || !uiBundlePath"
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
    happBundlePath: string | undefined;
    uiBundlePath: string | undefined;
  } {
    return {
      happBundlePath: undefined,
      uiBundlePath: undefined,
    };
  },
  methods: {
    async selectHappFile() {
      this.happBundlePath = (await open({
        filters: [{ name: "happ", extensions: ["happ"] }],
      })) as string;
    },
    async selectUIFile() {
      this.uiBundlePath = (await open({
        filters: [{ name: "ui", extensions: ["zip"] }],
      })) as string;
    },
    pathToFilename(path: string) {
      const components = path.split("/");
      return components[components.length - 1];
    },
    pathToAppId(path: string) {
      const filename = this.pathToFilename(path);
      return filename.split(".")[0];
    },
    async installApp() {
      const path = this.happBundlePath as string;
      let appId = this.pathToAppId(path);

      this.$store.commit("log", { log: "Installing hApp..." });

      try {
        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.installApp}`,
          {
            appBundlePath: this.happBundlePath,
            appId,
          }
        );
        this.$store.commit("log", { log: "Installed app" });

        const response = await invoke("install_ui", {
          uiBundlePath: this.uiBundlePath,
          appId,
        });
        this.$store.commit("log", {
          log: `Installed UI ${JSON.stringify(response)}`,
        });
      } catch (e) {
        this.$store.commit("log", {
          log: `Error installing hApp ${appId}: ${JSON.stringify(e)}`,
        });
      }

      this.happBundlePath = undefined;
      this.uiBundlePath = undefined;
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
