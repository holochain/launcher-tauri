<template>
  <div class="row center">
    <span style="margin-right: 16px">Install App</span>
    <span style="margin-right: 8px">hApp file: </span>
    <input
      type="file"
      name="Happ file"
      @change="happFile = $event.target.files[0]"
      accept=".happ"
      class="input-file"
      style="margin-right: 16px"
    />
    <span style="margin-right: 8px">UI file: </span>
    <input
      type="file"
      name="UI ZIP file"
      @change="uiFile = $event.target.files[0]"
      accept=".zip"
      class="input-file"
      style="margin-right: 16px"
    />

    <button :disabled="!uiFile || !happFile" @click="installApp()">
      Install
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import AdminUI from "@holochain/admin-ui";
import { invoke } from "@tauri-apps/api/tauri";
import { arrayBufferToBase64 } from "@/processors/buffer";

export default defineComponent({
  name: "InstallApp",
  data(): { happFile: File | undefined; uiFile: File | undefined } {
    return {
      happFile: undefined,
      uiFile: undefined,
    };
  },
  methods: {
    async installApp() {
      let appId = "";
      this.$store.commit("log", { log: "Installing hApp..." });

      try {
        const appBundle = await AdminUI.processors.fileToHappBundle(
          this.happFile as File
        );
        appId = appBundle.manifest.name;
        this.$store.commit("log", { log: "Converted .happ file to AppBundle" });

        const bytes = await (this.uiFile as File).arrayBuffer();
        const base64Bytes = await arrayBufferToBase64(bytes);
        const response = await invoke("install_ui", {
          base64Bytes,
          appId,
        });
        this.$store.commit("log", { log: "Installed UI" });

        await this.$store.dispatch(
          `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.installApp}`,
          appBundle
        );
        this.$store.commit("log", { log: "Installed app" });
      } catch (e) {
        this.$store.commit("log", {
          log: `Error installing hApp ${appId}: ${JSON.stringify(e)}`,
        });
      }
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
