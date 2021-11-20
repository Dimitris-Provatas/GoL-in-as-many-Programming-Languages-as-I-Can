<template>
  <div class="board">
    <div
      :style="`width: calc(100% / ${gridWidth});`"
      v-for="row in gridWidth"
      :key="'row' + row"
    >
      <CellVue
        :ref="`cell[${xPos},${yPos}]`"
        v-for="col in gridHeight"
        :key="'col' + col"
        :xPos="row"
        :yPos="col"
      />
    </div>
  </div>
</template>

<script>
import CellVue from "./Cell.vue";

export default {
  props: {
    gridWidth: {
      type: Number,
      required: true,
    },
    gridHeight: {
      type: Number,
      required: true,
    },
  },
  data: function () {
    return {};
  },
  components: {
    CellVue,
  },
  computed() {},
  methods: {
    nextTick() {
      for (let x = 1; x < this.gridWidth + 1; x++) {
        for (let y = 1; y < this.gridHeight + 1; y++) {
          this.$refs[`cell[${x},${y}]`].updateState();
        }
      }

      for (let x = 1; x < this.gridWidth + 1; x++) {
        for (let y = 1; y < this.gridHeight + 1; y++) {
          this.$refs[`cell[${x},${y}]`].commitState();
        }
      }
    },
  },
};
</script>

<style scoped>
.board {
  margin: auto;

  width: 75%;
  display: flex;
  flex: 1;

  flex-direction: row;
}

.board > div {
  display: flex;
  flex: 1;
  flex-grow: 1;
  flex-direction: column;
}
</style>