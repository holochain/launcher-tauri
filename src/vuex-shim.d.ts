import { Store } from "vuex";
import { LauncherAdminState } from "./store";
import { SnackbarService } from "vue3-snackbar";

declare module "@vue/runtime-core" {
  // Declare your own store states.
  import { ComponentCustomProperties } from "vue";

  interface ComponentCustomProperties {
    $store: Store<{ admin: HcAdminState } & LauncherAdminState>;
    $snackbar: SnackbarService;
  }
}
