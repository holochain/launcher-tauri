<template>
  <div class="container" style="position: relative">
    <!-- <img src="/img/HolochainError.png"> -->
    <div class="exclamation-mark row center-content">
      &nbsp;&nbsp;&nbsp; ! &nbsp;&nbsp;&nbsp;
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "HCErrorIcon",
});
</script>

<style scoped>
.container {
  background: url(/img/HolochainError.png) no-repeat center top;
  background-size: cover;
  /* border-radius: 50%; */
}
img {
  object-fit: contain;
}

.exclamation-mark {
  height: 100%;
  font-weight: 600;
  color: rgb(206, 0, 0);
  text-align: center;
  padding: 20% 0;
}
</style>
