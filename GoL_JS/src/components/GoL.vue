<template>
  <div class="board">
    <div
      :style="`width: calc(100% / ${gridWidth});`"
      v-for="row in gridWidth"
      :key="'row' + row"
    >
      <CellVue
        :ref="`cell[${row},${col}]`"
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
    tickDelay: {
      type: Number,
      required: true,
    },
  },
  data: function () {
    return {
      tickInterval: null,
      isPlaying: false,
    };
  },
  components: {
    CellVue,
  },
  mounted() {
    this.emitter.on("recalculate-grid", () => {
      this.$forceUpdate();

      this.emitter.emit("recalculate-height");
      this.$forceUpdate();
    });

    this.emitter.on("next-tick", () => {
      this.nextTick();
    });

    this.emitter.on("play", () => {
      this.tickInterval = setInterval(() => this.nextTick(), this.tickDelay);
      this.isPlaying = true;
    });

    this.emitter.on("stop", () => {
      clearInterval(this.tickInterval);
      this.tickInterval = null;
      this.isPlaying = false;
    });

    this.emitter.on("change-tick-delay", () => {
      clearInterval(this.tickInterval);
      this.tickInterval = null;

      if (this.isPlaying) {
        this.tickInterval = setInterval(() => this.nextTick(), this.tickDelay);
      }
    });
  },
  methods: {
    nextTick() {
      // for (let x = 1; x < this.gridWidth + 1; x++) {
      //   for (let y = 1; y < this.gridHeight + 1; y++) {
      //     this.$refs[`cell[${x},${y}]`].updateState();
      //   }
      // }

      // for (let x = 1; x < this.gridWidth + 1; x++) {
      //   for (let y = 1; y < this.gridHeight + 1; y++) {
      //     this.$refs[`cell[${x},${y}]`].commitState();
      //   }
      // }

      this.emitter.emit("next-tick-update");

      this.emitter.emit("next-tick-commit");
    },
  },
};
</script>

<style scoped>
.board {
  margin: auto;

  width: 90%;

  display: flex;
  flex: 1;
  flex-direction: row;

  border: 1px solid black;
}

.board > div {
  display: flex;
  flex: 1;
  flex-grow: 1;
  flex-direction: column;
}
</style>