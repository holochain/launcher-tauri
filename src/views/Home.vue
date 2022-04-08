<template>
  <div class="column" style="flex: 1">
    <div v-if="view.type === 'installedApps'" class="flex-scrollable-parent">
      <div class="flex-scrollable-container">
        <div class="flex-scrollable-y">
          <InstalledApps
            @open-app-store="view = { type: 'appStore' }"
            style="display: flex; margin: 24px; margin-bottom: 50px"
          ></InstalledApps>
        </div>
      </div>
    </div>

    <div v-else style="flex: 1; display: flex">
      <AppStore @go-back="view = { type: 'installedApps' }"></AppStore>
    </div>
  </div>
</template>

<script lang="ts">
import AppStore from "./AppStore.vue";
import InstalledApps from "./InstalledApps.vue";
import { defineComponent } from "vue";
import "@material/mwc-fab";

type View =
  | {
      type: "installedApps";
    }
  | {
      type: "appStore";
    };

export default defineComponent({
  name: "Home",
  components: {
    AppStore,
    InstalledApps,
  },
  data(): {
    snackbarText: string | undefined;
    view: View;
  } {
    return {
      snackbarText: undefined,
      view: {
        type: "installedApps",
      },
    };
  },
  methods: {},
});
</script>
