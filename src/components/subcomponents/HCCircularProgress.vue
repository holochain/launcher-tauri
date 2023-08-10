<template>
  <div class="lds-ring">
    <div class="slow"></div>
    <div class="slow"></div>
    <div class="slow"></div>
    <div class="fast"></div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCCircularProgress",
  props: {
    disabled: {
      type: Boolean,
      default: false,
    },
    outlined: {
      type: Boolean,
      default: false,
    },
  },
  data(): {
    passwordVisible: boolean;
    focus: boolean;
  } {
    return {
      passwordVisible: false,
      focus: false,
    };
  },
});
</script>

<style scoped>
.lds-ring {
  display: inline-block;
  position: relative;
  width: 50px;
  height: 50px;
  --loading-color: #492edf;
}
.lds-ring div {
  box-sizing: border-box;
  display: block;
  position: absolute;
  width: 42px;
  height: 42px;
  margin: 4px;
  border: 4px solid;
  border-radius: 50%;
  border-color: var(--loading-color) transparent transparent transparent;
  animation: lds-ring-slow 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
}
.lds-ring div:nth-child(1) {
  animation-delay: -0.8s;
  border-color: var(--loading-color) var(--loading-color) transparent transparent;
  animation: lds-ring-fast 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
}
.lds-ring div:nth-child(2) {
  animation-delay: -0.3s;
  border-color: var(--loading-color) transparent transparent transparent;
}
.lds-ring div:nth-child(3) {
  animation-delay: -0.15s;
  border-color: var(--loading-color) transparent transparent transparent;
}
@keyframes lds-ring {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
@keyframes lds-ring-fast {
  0% {
    transform: rotate(0deg);
  }
  50% {
    transform: rotate(250deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
@keyframes lds-ring-slow {
  0% {
    transform: rotate(0deg);
  }
  50% {
    transform: rotate(110deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

</style>
