<template>
  <div class="background">
    <!-- <img src="/img/Holochain_Halo_complete.svg" style="height: 110vh;"> -->
    <div v-if="step == 0" class="column" style="align-items: center">
      <div style="color: white; font-size: 56px; margin-bottom: 30px">
        Welcome to Holochain
      </div>
      <div style="color: white; margin-bottom: 80px; font-size: 22px">
        [imagine a catchy slogan here]
      </div>
      <HCButton
        @click="step = 1"
        style="--hc-primary-color: white; color: #331ead"
        >Get started</HCButton
      >
    </div>

    <div
      v-if="step == 1"
      class="column"
      style="
        box-shadow: 0 0 30px rgb(21, 16, 65);
        border-radius: 15px;
        background: white;
        align-items: center;
        max-width: 700px;
        padding: 55px 50px 50px 50px;
      "
    >
      <div class="row" style="margin-bottom: 40px">
        <img
          src="/img/lock_icon.svg"
          style="height: 35px; opacity: 0.95; margin-right: 20px"
        />
        <div style="font-size: 27px; font-weight: 600; margin-bottom: 25px">
          Set up password
        </div>
      </div>

      <div style="font-size: 1.1em; margin-bottom: 15px; width: 100%">
        To get started, you need to set up a password for the Holochain
        Launcher.
      </div>
      <div style="font-size: 1.1em; text-align: left; margin-bottom: 40px">
        This password is used to secure your cryptographic private keys
        associated with your identities in Holochain apps.
      </div>
      <div class="column" style="margin-bottom: 20px">
        <div style="font-weight: bold; margin-bottom: 2px">
          Choose your password:
        </div>
        <PasswordField
          required
          initialFocus
          ref="password"
          placeholder="Choose password"
          @input="checkPasswordValidity"
          style="margin-bottom: 12px"
        />
      </div>
      <div
        style="
          background-color: #f8cecc;
          border-radius: 10px;
          padding: 15px;
          margin-bottom: 30px;
        "
      >
        ⚠️ This password <strong>cannot be reset or changed</strong> without
        losing access to your Holochain apps on this device.<br />
        Make sure to write it on paper right now and create at least one backup
        of it in a safe place.
      </div>
      <div class="row" style="align-items: center; margin-bottom: 50px">
        <ToggleSwitch
          tabindex="0"
          :sliderOn="backupConfirmed"
          @click="backupConfirmed = !backupConfirmed"
          @keydown.enter="backupConfirmed = !backupConfirmed"
        ></ToggleSwitch>
        <div style="margin-left: 10px">
          I confirm that I have created a safe backup of my password.
        </div>
      </div>
      <HCButton
        @click="choosePassword"
        :disabled="!backupConfirmed || !isPasswordValid"
        style="width: 70px"
      >
        Next
      </HCButton>
    </div>

    <div
      v-if="step == 2"
      class="column"
      style="
        box-shadow: 0 0 30px rgb(21, 16, 65);
        border-radius: 15px;
        background: white;
        align-items: center;
        max-width: 700px;
        padding: 50px;
      "
    >
      <img
        src="/img/lock_icon.svg"
        style="height: 35px; margin-bottom: 40px; opacity: 0.95"
      />
      <div class="column" style="margin-bottom: 3px">
        <div style="font-weight: bold; margin-bottom: 2px">
          Repeat your password:
        </div>
        <PasswordField
          required
          initialFocus
          ref="repeatPassword"
          placeholder="Repeat password"
          @input="checkRepeatPasswordValidity"
          @keydown.enter="initialize()"
        />
      </div>
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
        {{
          passwordsDontMatch && repeatedPassword
            ? "Password's don't match."
            : ""
        }}
      </div>
      <div class="row">
        <HCButton
          outlined
          style="width: 70px; height: 30px; margin-right: 10px"
          @click="
            step = 1;
            firstPassword = undefined;
            backupConfirmed = false;
            repeatedPassword = undefined;
          "
        >
          Back
        </HCButton>

        <HCButton
          @click="initialize()"
          :disabled="!backupConfirmed || passwordsDontMatch"
          style="width: 70px"
        >
          Let's Go!
        </HCButton>
      </div>
    </div>

    <div
      v-if="step == 3"
      class="column"
      style="max-width: 660px; align-items: center"
    >
      <div style="font-size: 40px; margin-bottom: 45px; color: #e2e1f5">
        Setting up
      </div>
      <LoadingDots style="--radius: 15px"></LoadingDots>
    </div>

    <div
      v-if="loadingState"
      style="
        position: fixed;
        bottom: 5px;
        left: 10px;
        color: #e2e1f5;
        font-size: 0.9em;
      "
    >
      {{ loadingState }}...
    </div>
  </div>
</template>

<script lang="ts">
import { ActionTypes } from "../../store/actions";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";
import PasswordField from "../subcomponents/PasswordField.vue";
import HCButton from "../subcomponents/HCButton.vue";
import ToggleSwitch from "../subcomponents/ToggleSwitch.vue";
import LoadingDots from "../subcomponents/LoadingDots.vue";
import { listen } from "@tauri-apps/api/event";

export default defineComponent({
  name: "Setup",
  components: { PasswordField, HCButton, ToggleSwitch, LoadingDots },
  data(): {
    isPasswordValid: boolean;
    passwordsDontMatch: boolean;
    firstPassword: string | undefined;
    initializing: boolean;
    pwInputDisabled: boolean;
    backupConfirmed: boolean;
    step: number;
    repeatedPassword: string | undefined;
    loadingState: string | undefined;
  } {
    return {
      isPasswordValid: false,
      passwordsDontMatch: true,
      firstPassword: undefined,
      initializing: false,
      pwInputDisabled: false,
      backupConfirmed: false,
      step: 0,
      repeatedPassword: undefined,
      loadingState: undefined,
    };
  },
  async mounted() {
    this.loadingState = undefined;
    await listen("progress-update", (event) => {
      this.loadingState = event.payload as string;
    });
  },
  created() {
    this.$nextTick(() => {
      const repeatPassword = this.$refs.repeatPassword as typeof PasswordField;
    });
  },
  methods: {
    async initialize() {
      this.step = 3;
      if (!this.initializing && !this.passwordsDontMatch) {
        const password = this.firstPassword;

        await invoke("initialize_keystore", { password });
        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      }
    },
    checkPasswordValidity() {
      const passwordValue = (this.$refs.password as typeof PasswordField).value;
      this.firstPassword = passwordValue;
      this.isPasswordValid = passwordValue.length > 0;
    },
    checkRepeatPasswordValidity() {
      const repeatPasswordValue = (
        this.$refs.repeatPassword as typeof PasswordField
      ).value;
      this.repeatedPassword = repeatPasswordValue;
      const itsAMatch = this.firstPassword === repeatPasswordValue;
      this.passwordsDontMatch = !itsAMatch;
      return itsAMatch;
    },
    choosePassword() {
      this.step = 2;
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
  /* background-color: rgb(21, 16, 65); */
  /* background-color: #e3e4eb; */
  background-color: #331ead;
  background-size: cover;
  background-position: center center;
  background-image: url(/img/Holochain_Halo.svg);
}

.left-half {
  background-color: #331ead;
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

input[type="checkbox"] {
  transform: scale(1.8);
}

.animated {
  animation: colorchange 7s linear infinite;
}

@keyframes colorchange {
  0% {
    color: #6b66c9;
  }
  50% {
    color: #ffffff;
  }
  100% {
    color: #6b66c9;
  }
}
</style>
