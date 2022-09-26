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
          Enter password
        </div>

        <form>
          <div class="column" style="align-items: center">
            <PasswordField
              required
              ref="password"
              placeholder="Enter password"
              style="margin-bottom: 30px"
            />

            <HcButton
              :disabled="entering"
              @click="enterPassword()"
              style="width: 128px"
              >{{ this.entering ? "Loading..." : "Continue" }}
            </HcButton>
          </div>
        </form>
      </div>
    </div>
  </div>

  <mwc-snackbar
    leading
    labelText="Incorrect Password"
    ref="snackbar"
  ></mwc-snackbar>
</template>

<script lang="ts">
import { ActionTypes } from "@/store/actions";
import { invoke } from "@tauri-apps/api/tauri";
import { defineComponent } from "vue";
import PasswordField from "../subcomponents/PasswordField.vue";
import HcButton from "../subcomponents/HcButton.vue";

export default defineComponent({
  name: "EnterPassword",
  components: { PasswordField, HcButton },
  data(): { entering: boolean } {
    return {
      entering: false,
    };
  },
  methods: {
    async enterPassword() {
      this.entering = true;
      const password = (this.$refs["password"] as typeof PasswordField).value;
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
