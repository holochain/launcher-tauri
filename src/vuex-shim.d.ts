import { Store } from "vuex";
import { LauncherAdminState } from "./store";

declare module "@vue/runtime-core" {
  // Declare your own store states.
  import { ComponentCustomProperties } from "vue";

  interface ComponentCustomProperties {
    $store: Store<LauncherAdminState>;
  }
}
