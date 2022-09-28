<template>
  <div class="background">
    <div class="row">
      <div class="column center-content left-half">
        <div
          style="
            color: #ffffff;
            text-align: center;
            font-size: 40px;
            line-height: 48px;
            margin: 20px;
          "
        >
          Discover, install and easily manage your Holochain apps
        </div>
        <img class="halo" src="/img/Holochain_Halo.svg" />
      </div>
      <div class="column center-content right-half">
        <img
          src="/img/lock_icon.svg"
          style="height: 35px; margin-bottom: 10px"
        />
        <div style="font-size: 27px; font-weight: 600; margin-bottom: 25px">
          Create password
        </div>

        <form>
          <div class="column" style="align-items: center">
            <PasswordField
              required
              initialFocus
              ref="password"
              placeholder="Enter password"
              style="margin-bottom: 12px"
            />
            <PasswordField
              required
              ref="repeatPassword"
              placeholder="Confirm password"
              style="margin-bottom: 3px"
              @input="checkPasswordValidity"
            />

            <div
              style="
                margin-bottom: 50px;
                color: #ff3131;
                text-align: left;
                padding: 0 10px;
                font-size: 0.9em;
                height: 22px;
              "
            >
              {{ this.passwordsDontMatch ? "Password's don't match." : "" }}
            </div>

            <HcButton
              :disabled="initializing || !isPasswordValid"
              @click="initialize()"
              style="width: 128px"
              >{{ this.initializing ? "initializing..." : "Continue" }}
            </HcButton>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { ActionTypes } from "@/store/actions";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";
import PasswordField from "../subcomponents/PasswordField.vue";
import HcButton from "../subcomponents/HCButton.vue";

export default defineComponent({
  name: "Setup",
  components: { PasswordField, HcButton },
  data() {
    return {
      isPasswordValid: false,
      passwordsDontMatch: false,
      initializing: false,
    };
  },
  created() {
    this.$nextTick(() => {
      const repeatPassword = this.$refs.repeatPassword as typeof PasswordField;
      console.log("repeatPassword value: ", repeatPassword.value);
      // repeatPassword.validityTransform = (newValue: string, nativeValidity) => {
      //   const password = (this.$refs.password as TextField).value;
      //   if (newValue !== password) {
      //     repeatPassword.setCustomValidity("Passwords don't match");

      //     this.isPasswordValid = false;
      //     return {
      //       valid: false,
      //     };
      //   } else {
      //     this.isPasswordValid = password.length > 0;
      //     return {
      //       valid: true,
      //     };
      //   }
      // };
    });
  },
  methods: {
    async initialize() {
      if (!this.initializing && this.isPasswordValid) {
        // condition required to omit ENTER key triggering initialization
        this.initializing = true;
        const password = (this.$refs["password"] as typeof PasswordField).value;

        await invoke("initialize_keystore", { password });
        await this.$store.dispatch(ActionTypes.fetchStateInfo);
        this.initializing = false;
      }
    },
    checkPasswordValidity() {
      console.log("Checking password validity.");
      console.log(
        "password value: ",
        (this.$refs.password as typeof PasswordField).value
      );
      console.log(
        "repeatPassword value: ",
        (this.$refs.repeatPassword as typeof PasswordField).value
      );
      const passwordValue = (this.$refs.password as typeof PasswordField).value;
      const repeatPasswordValue = (
        this.$refs.repeatPassword as typeof PasswordField
      ).value;
      this.isPasswordValid =
        passwordValue.length > 0 && passwordValue === repeatPasswordValue;
      this.passwordsDontMatch =
        repeatPasswordValue.length > 0 && passwordValue !== repeatPasswordValue;
    },
  },
});
</script>

<style scoped>
.background {
  display: flex;
  flex: 1;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: rgb(21, 16, 65);
}

.left-half {
  background-color: #482edf;
  height: 695px;
  width: 500px;
  border-radius: 15px 0 0 15px;
  position: relative;
  overflow: hidden;
}

.right-half {
  background-color: #ffffff;
  height: 695px;
  width: 500px;
  border-radius: 0 15px 15px 0;
}

.halo {
  height: 700px;
  position: absolute;
  left: -10.11%;
  right: 51.81%;
  top: 0;
  bottom: 18.88%;
}
</style>
