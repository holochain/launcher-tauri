<template>
  <label class="switch">
    <input tabindex="0" type="checkbox" :checked="sliderOn" />
    <span class="slider round" @click.stop="handleSlide"></span>
  </label>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "ToggleSwitch",
  props: {
    sliderOn: {
      type: Boolean,
      required: true,
    },
  },
  emits: ["turn-on", "turn-off"],
  methods: {
    handleSlide() {
      if (this.sliderOn) {
        this.$emit("turn-off");
      } else {
        this.$emit("turn-on");
      }
    },
  },
});
</script>

<style scoped>
.switch {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
}

.switch input {
  opacity: 0;
  width: 40px;
  height: 0;
}

/* The slider */
.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

input:checked + .slider {
  background-color: #482edf;
}

/* input:focus + .slider {
    box-shadow: 0 0 1px #482EDF;
  } */

input:checked + .slider:before {
  -webkit-transform: translateX(18px);
  -ms-transform: translateX(18px);
  transform: translateX(18px);
}

/* Rounded sliders */
.slider.round {
  border-radius: 22px;
}

.slider.round:before {
  border-radius: 50%;
}
</style>
