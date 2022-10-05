<template>
  <canvas ref="identicon" />
</template>

<script>
import { defineComponent } from "vue";
import renderIcon from "@holo-host/identicon";

export default defineComponent({
  name: "HoloIdenticon",
  props: {
    hash: {
      type: Uint8Array,
      required: true,
    },
    size: {
      type: Number,
      default: 32,
    },
  },
  methods: {
    renderIcon,
  },
  mounted() {
    const opts = {
      hash: new Uint8Array(JSON.parse(JSON.stringify(this.hash))),
      size: this.size,
    };
    this.renderIcon(opts, this.$refs.identicon);
  },
});
</script>
