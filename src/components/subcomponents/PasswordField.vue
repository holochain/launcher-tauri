<template>
  <div
    class="password-field row"
    style="align-items: center; flex-wrap: nowrap"
  >
    <input
      ref="passwordInput"
      :required="required"
      class="password-input"
      v-model="value"
      :type="passwordVisible ? undefined : 'password'"
      :placeholder="placeholder"
      :autofocus="initialFocus"
      :disabled="disabled"
    />
    <!-- <img src="eye_icon.svg" style="width: 23px; cursor: pointer;"/> -->

    <svg
      v-if="!passwordVisible"
      class="eye-icon"
      :class="{ eyeIconFocus: inFocus && !disabled }"
      @click="
        this.$refs.passwordInput.focus();
        passwordVisible = !passwordVisible;
      "
      aria-label="show password"
      width="23"
      height="15"
      viewBox="0 0 26 18"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M13.3079 2.38045C17.6075 2.38045 21.4419 4.79681 23.3137 8.61987C21.4419 12.4429 17.6075 14.8593 13.3079 14.8593C9.00841 14.8593 5.17401 12.4429 3.30218 8.61987C5.17401 4.79681 9.00841 2.38045 13.3079 2.38045ZM13.3079 0.111572C7.63574 0.111572 2.79168 3.63968 0.829102 8.61987C2.79168 13.6001 7.63574 17.1282 13.3079 17.1282C18.9801 17.1282 23.8242 13.6001 25.7868 8.61987C23.8242 3.63968 18.9801 0.111572 13.3079 0.111572ZM13.3079 5.78377C14.8735 5.78377 16.144 7.05434 16.144 8.61987C16.144 10.1854 14.8735 11.456 13.3079 11.456C11.7424 11.456 10.4718 10.1854 10.4718 8.61987C10.4718 7.05434 11.7424 5.78377 13.3079 5.78377ZM13.3079 3.51489C10.4945 3.51489 8.20296 5.80646 8.20296 8.61987C8.20296 11.4333 10.4945 13.7248 13.3079 13.7248C16.1213 13.7248 18.4129 11.4333 18.4129 8.61987C18.4129 5.80646 16.1213 3.51489 13.3079 3.51489Z"
      />
    </svg>

    <svg
      v-else
      class="eye-icon"
      :class="{ eyeIconFocus: inFocus && !disabled }"
      @click="
        this.$refs.passwordInput.focus();
        passwordVisible = !passwordVisible;
      "
      aria-label="hide password"
      width="23"
      height="15"
      viewBox="0 0 26 18"
      fill="none"
      version="1.1"
      xmlns="http://www.w3.org/2000/svg"
      xmlns:svg="http://www.w3.org/2000/svg"
    >
      <path
        d="M 13.308594 0.11132812 C 10.517021 0.11132812 7.9268891 0.96804285 5.78125 2.4296875 L 2.2285156 0.171875 L 0.91796875 2.234375 L 3.8066406 4.0703125 C 2.5276205 5.3576952 1.5072856 6.9006754 0.83007812 8.6191406 C 2.7926561 13.599371 7.6364338 17.128906 13.308594 17.128906 C 16.073791 17.128906 18.639748 16.287996 20.773438 14.851562 L 24.345703 17.123047 L 25.65625 15.058594 L 22.759766 13.21875 C 24.061669 11.921674 25.101299 10.359432 25.787109 8.6191406 C 23.824509 3.6389506 18.980794 0.11132813 13.308594 0.11132812 z M 13.308594 2.3808594 C 17.608194 2.3808594 21.442653 4.7960806 23.314453 8.6191406 C 22.681794 9.9113054 21.820132 11.038917 20.796875 11.970703 L 18.130859 10.277344 C 18.310763 9.7564004 18.412109 9.2001985 18.412109 8.6191406 C 18.412109 5.8057306 16.121994 3.515625 13.308594 3.515625 C 11.930689 3.515625 10.679412 4.0655827 9.7597656 4.9570312 L 7.8984375 3.7753906 C 9.5146174 2.8796069 11.363302 2.3808594 13.308594 2.3808594 z M 5.7675781 5.3164062 L 8.4609375 7.0273438 C 8.2955918 7.528936 8.203125 8.0631236 8.203125 8.6191406 C 8.203125 11.432571 10.495194 13.724609 13.308594 13.724609 C 14.660321 13.724609 15.888203 13.191764 16.802734 12.330078 L 18.650391 13.503906 C 17.050295 14.376205 15.226866 14.859375 13.308594 14.859375 C 9.0091037 14.859375 5.1745644 12.442171 3.3027344 8.6191406 C 3.9247894 7.348644 4.7669924 6.2391358 5.7675781 5.3164062 z M 13.308594 5.7832031 C 14.874194 5.7832031 16.144531 7.0536106 16.144531 8.6191406 C 16.144531 8.7469117 16.13163 8.8727822 16.115234 8.9960938 L 11.773438 6.2363281 C 12.216095 5.9505299 12.742875 5.7832031 13.308594 5.7832031 z M 10.488281 8.3164062 L 14.777344 11.041016 C 14.348278 11.302095 13.847251 11.455078 13.308594 11.455078 C 11.743094 11.455078 10.472656 10.184671 10.472656 8.6191406 C 10.472656 8.5165457 10.477642 8.4161931 10.488281 8.3164062 z "
      />
    </svg>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "PasswordField",
  props: {
    placeholder: {
      type: String,
      default: "Enter password",
    },
    required: {
      type: Boolean,
      default: true,
    },
    initialFocus: {
      type: Boolean,
      default: false,
    },
    disabled: {
      type: Boolean,
      default: false,
    },
  },
  data(): {
    passwordVisible: boolean;
    inFocus: boolean;
    value: string;
  } {
    return {
      passwordVisible: false,
      inFocus: false,
      value: "",
    };
  },
  methods: {
    focus() {
      (this.$refs.passwordInput as HTMLInputElement).focus();
      this.inFocus = true;
    },
    blur() {
      (this.$refs.passwordInput as HTMLElement).blur();
      this.inFocus = false;
    },
  },
});
</script>

<style scoped>
.password-input {
  all: unset;
  display: flex;
  flex: 1;
}

.password-field {
  height: 42px;
  width: 300px;
  box-shadow: 0 0 0 2px rgba(59, 61, 115, 0.25);
  border-radius: 10px;
  padding: 0 10px;
  --hc-primary-color: #482edf;
  --child-opacity: 0.35;
}

.password-field:focus-within {
  box-shadow: 0 0 0 2px var(--hc-primary-color);
  --child-opacity: 1;
}

.eye-icon {
  fill: #482edf;
  fill-opacity: var(--child-opacity);
  display: table;
  width: -moz-fit-content;
  padding: 0;
}
</style>
