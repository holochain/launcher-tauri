<template>
  <div v-if="entering" class="entering-background">
    <div class="animated" style="font-size: 40px">{{ bootUpSlogan }}</div>
  </div>
  <div v-else class="background">
    <div
      class="row"
      style="box-shadow: 0 0 35px rgb(21, 16, 65); border-radius: 15px"
    >
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
          style="height: 35px; margin-bottom: 10px; opacity: 0.95"
        />
        <div style="font-size: 27px; font-weight: 600; margin-bottom: 25px">
          Enter password
        </div>

        <form>
          <div class="column" style="align-items: center">
            <PasswordField
              required
              initialFocus
              :disabled="pwInputDisabled"
              ref="password"
              placeholder="Enter password"
              style="margin-bottom: 5px"
              @input="invalidPassword = false"
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
              {{ invalidPassword ? "Invalid Password." : "" }}
            </div>

            <HCButton
              :disabled="entering"
              @click="enterPassword()"
              style="width: 128px"
              >{{ entering ? "Starting..." : "Continue" }}
            </HCButton>
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
import HCButton from "../subcomponents/HCButton.vue";
import { bootUpSlogans } from "../../bootUpSlogans";

export default defineComponent({
  name: "EnterPassword",
  components: { PasswordField, HCButton },
  data(): {
    entering: boolean;
    pwInputDisabled: boolean;
    invalidPassword: boolean;
    bootUpSlogan: string | undefined;
  } {
    return {
      entering: false,
      pwInputDisabled: false,
      invalidPassword: false,
      bootUpSlogan: undefined,
    };
  },
  mounted() {
    this.bootUpSlogan =
      bootUpSlogans[Math.floor(Math.random() * bootUpSlogans.length)];
  },
  methods: {
    async enterPassword() {
      this.entering = true;
      this.pwInputDisabled = true;
      const passwordField = this.$refs["password"] as typeof PasswordField;
      passwordField.blur();
      const password = passwordField.value;
      try {
        await invoke("unlock_and_launch", { password });
        await this.$store.dispatch(ActionTypes.fetchStateInfo);
      } catch (e: any) {
        console.error(e);
        if (e === "Error launching the keystore: IncorrectPassword") {
          this.invalidPassword = true;
        }
        this.pwInputDisabled = false;
        this.$nextTick(() => passwordField.setFocus());
      }
      this.entering = false;
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
  background-color: #e8e8eb;
  /* background-color: #331ead; */
}

.left-half {
  /* background-color: #482edf; */
  /* background-color: rgb(44, 32, 148); */
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

.entering-background {
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
  background-image: url(/img/Holochain_Halo_complete.svg);
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
