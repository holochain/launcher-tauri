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
      ></mwc-textfield>
    </div>

    <mwc-button
      :label="introducing ? 'Entering...' : 'Enter Password'"
      :disabled="introducing"
      slot="primaryAction"
      @click="introducePassword()"
    ></mwc-button>
  </mwc-dialog>
  <mwc-snackbar
    leading
    labelText="Incorrect Password"
    ref="snackbar"
  ></mwc-snackbar>
</template>

<script lang="ts">
import { ActionTypes } from "@/store/actions";
import { TextField } from "@material/mwc-textfield";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";

export default defineComponent({
  name: "IntroducePassword",
  data(): { introducing: boolean } {
    return {
      introducing: false,
    };
  },
  methods: {
    async introducePassword() {
      this.introducing = true;
      const password = (this.$refs["password"] as TextField).value;
      try {
        await invoke("unlock_and_launch", { password });
        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      } catch (e) {
        console.error(e);
        (this.$refs as any).snackbar.show();
      }
      this.introducing = false;
    },
  },
});
</script>
