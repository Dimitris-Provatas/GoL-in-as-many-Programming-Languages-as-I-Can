<template>
  <div style="width: 100%; height: 100%">
    <div class="main">
      <GoLVue
        :grid-width="gridWidth"
        :grid-height="gridHeight"
        :tickDelay="tickDelay"
        ref="board"
      />
    </div>
    <div class="controls">
      <ControllsVue ref="controls" />
    </div>
  </div>
</template>

<script>
import GoLVue from "./components/GoL.vue";
import ControllsVue from "./components/Controlls.vue";

export default {
  components: {
    GoLVue,
    ControllsVue,
  },
  data() {
    return {
      gridWidth: 60,
      gridHeight: 20,
      tickDelay: 1000,
    };
  },
};
</script>

<style>
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
