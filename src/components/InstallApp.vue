<template>
  <div class="row">
    <span>Install App</span>
    <input
      type="file"
      name="Happ file"
      @change="happFile = $event.target.files[0]"
      accept=".happ"
      class="input-file"
    />
    <input
      type="file"
      name="UI ZIP file"
      @change="uiFile = $event.target.files[0]"
      accept=".zip"
      class="input-file"
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
      const appBundle = await AdminUI.processors.fileToHappBundle(
        this.happFile as File
      );
      const appId = appBundle.manifest.name;

      const bytes = await (this.uiFile as File).arrayBuffer();
      const base64Bytes = await arrayBufferToBase64(bytes);
      const response = await invoke("install_ui", {
        base64Bytes,
        appId,
      });

      this.$store.dispatch(
        `${AdminUI.ADMIN_UI_MODULE}/${AdminUI.ActionTypes.installApp}`,
        appBundle
      );
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
</style>
