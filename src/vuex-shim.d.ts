import { Store } from "vuex";

declare module "@vue/runtime-core" {
  // Declare your own store states.
  import { ComponentCustomProperties } from "vue";

  interface ComponentCustomProperties {
    $store: Store<{ admin: HcAdminState }>;
  }
}
