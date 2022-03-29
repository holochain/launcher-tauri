<template>
  <mwc-dialog
    heading="Setup Password"
    scrimClickAction=""
    escapeKeyAction=""
    open
  >
    <div class="column">
      <span
        >This is the password with which the keystore where your private keys
        live will be initialized.
      </span>
      <mwc-textfield outlined ref="password" label="Password"></mwc-textfield>
      <mwc-textfield outlined label="Repeat Password"></mwc-textfield>
    </div>

    <mwc-button
      label="Initialize Keystore"
      slot="primaryAction"
      @click="initialize()"
    ></mwc-button>
  </mwc-dialog>
</template>

<script lang="ts">
import { ActionTypes } from "@/store/actions";
import { TextField } from "@material/mwc-textfield";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";

export default defineComponent({
  name: "Setup",
  methods: {
    async initialize() {
      const password = (this.$refs["password"] as TextField).value;

      await invoke("initialize_keystore", { password });
      await this.$store.dispatch(ActionTypes.fetchStateInfo);
    },
  },
});
</script>
