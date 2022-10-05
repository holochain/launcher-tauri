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
      <span style="margin-top: 16px"
        ><b>WARNING!</b> If you lose this password, you will also lose access to
        any data from your Holochain applications, as there is no password
        recovery mechanism.
      </span>

      <mwc-textfield
        outlined
        type="password"
        ref="password"
        autoValidate
        helper=" "
        style="margin-top: 24px"
        label="Password"
        dialogInitialFocus
      ></mwc-textfield>

      <mwc-textfield
        outlined
        autoValidate
        ref="repeatPassword"
        helper=" "
        style="margin-top: 16px"
        type="password"
        label="Repeat Password"
      ></mwc-textfield>
    </div>

    <mwc-button
      :disabled="initializing || !isPasswordValid"
      :label="initializing ? 'Initializing...' : 'Initialize'"
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
  data() {
    return {
      isPasswordValid: false,
      initializing: false,
    };
  },
  created() {
    this.$nextTick(() => {
      const repeatPassword = this.$refs.repeatPassword as TextField;
      repeatPassword.validityTransform = (newValue: string, nativeValidity) => {
        const password = (this.$refs.password as TextField).value;
        if (newValue !== password) {
          repeatPassword.setCustomValidity("Passwords don't match");

          this.isPasswordValid = false;
          return {
            valid: false,
          };
        } else {
          this.isPasswordValid = password.length > 0;
          return {
            valid: true,
          };
        }
      };
    });
  },
  methods: {
    async initialize() {
      if (!this.initializing && this.isPasswordValid) {
        // condition required to omit ENTER key triggering initialization
        this.initializing = true;
        const password = (this.$refs["password"] as TextField).value;

        await invoke("initialize_keystore", { password });
        await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.initializing = false;
      }
    },
  },
});
</script>
