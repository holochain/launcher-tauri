<template>
  <mwc-dialog
    heading="Enter Password"
    scrimClickAction=""
    escapeKeyAction=""
    open
  >
    <div class="column">
      <span
        >This is the password with which you initialized the keystore.
      </span>
      <mwc-textfield
        ref="password"
        type="password"
        style="margin-top: 16px"
        label="Password"
        outlined
        dialogInitialFocus
      ></mwc-textfield>
    </div>

    <mwc-button
      :label="entering ? 'Entering...' : 'Enter Password'"
      :disabled="entering"
      slot="primaryAction"
      @click="enterPassword()"
    ></mwc-button>
  </mwc-dialog>
  <mwc-snackbar
    leading
    labelText="Incorrect Password"
    ref="snackbar"
  ></mwc-snackbar>
</template>

<script lang="ts">
import { ActionTypes } from "../../store/actions";
import { TextField } from "@material/mwc-textfield";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";

export default defineComponent({
  name: "EnterPassword",
  data(): { entering: boolean } {
    return {
      entering: false,
    };
  },
  methods: {
    async enterPassword() {
      this.entering = true;
      const password = (this.$refs["password"] as TextField).value;
      try {
        await invoke("unlock_and_launch", { password });
        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      } catch (e) {
        console.error(e);
        (this.$refs as any).snackbar.show();
      }
      this.entering = false;
    },
  },
});
</script>
