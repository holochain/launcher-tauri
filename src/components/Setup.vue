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

      <mwc-textfield
        outlined
        type="password"
        ref="password"
        autoValidate
        style="margin-top: 16px"
        label="Password"
      ></mwc-textfield>

      <mwc-textfield
        outlined
        autoValidate
        ref="repeatPassword"
        style="margin-top: 16px"
        type="password"
        label="Repeat Password"
      ></mwc-textfield>
    </div>

    <mwc-button
      label="Initialize Keystore"
      slot="primaryAction"
      @click="initialize()"
      :disabled="!isPasswordValid"
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
    };
  },
  created() {
    this.$nextTick(() => {
      const repeatPassword = this.$refs.repeatPassword as TextField;
      repeatPassword.validityTransform = (newValue: string, nativeValidity) => {
        if (newValue !== (this.$refs.password as TextField).value) {
          repeatPassword.setCustomValidity("Passwords don't match");

          this.isPasswordValid = false;
          return {
            valid: false,
          };
        } else {
          this.isPasswordValid = true;
          return {
            valid: true,
          };
        }
      };
    });
  },
  methods: {
    async initialize() {
      const password = (this.$refs["password"] as TextField).value;

      await invoke("initialize_keystore", { password });
      await this.$store.dispatch(ActionTypes.fetchStateInfo);
    },
  },
});
</script>
