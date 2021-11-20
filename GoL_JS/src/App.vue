<template>
  <div class="main">
    <GoLVue :grid-width="20" :grid-height="10" />
  </div>
  <div class="controls">
    <ControllsVue />
  </div>
</template>

<script setup>
import GoLVue from "./components/GoL.vue";
import ControllsVue from "./components/Controlls.vue";
</script>

<style>
#app {
  width: 100vw;
  height: 100vh;

  -webkit-touch-callout: none; /* iOS Safari */
  -webkit-user-select: none; /* Safari */
  -khtml-user-select: none; /* Konqueror HTML */
  -moz-user-select: none; /* Old versions of Firefox */
  -ms-user-select: none; /* Internet Explorer/Edge */
  user-select: none; /* Non-prefixed version, currently supported by Chrome, Edge, Opera and Firefox */
}

.main {
  width: 100%;
  height: 85%;

  background-color: wheat;
}

.controls {
  width: 100%;
  height: 15%;

  border-top: 2px solid black;
  background-color: wheat;
}
</style>
