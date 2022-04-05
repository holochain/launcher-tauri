<template>
  <div v-if="loading" class="column center-content" style="flex: 1">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  <div v-else></div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import "@material/mwc-dialog";
import "@material/mwc-circular-progress";
import { HappRelease } from "@/devhub/types";
import { AppWebsocket } from "@holochain/client";
import { AppWithReleases, getAppsForHdk } from "../../devhub/get-happs";
import { HdkVersion } from "@/hdk";

export default defineComponent({
  name: "InstallableApps",
  data(): { loading: true; installableApps: Array<AppWithReleases> } {
    return {
      loading: true,
      installableApps: [],
    };
  },

  async mounted() {
    const port = this.$store.getters["appInterfacePort"];

    const appWs = await AppWebsocket.connect(`ws://localhost:${port}`);

    const devhubInfo = await appWs.appInfo({ installed_app_id: "DevHub" });
    const devhubHappCell = devhubInfo.cell_data.find(
      (c) => c.role_id === "happs"
    );

    if (!devhubHappCell) throw new Error("Happ library cell not found");

    this.installableApps = await getAppsForHdk(
      appWs,
      devhubHappCell,
      HdkVersion.v0_0_127
    );
  },
  methods: {},
});
</script>
