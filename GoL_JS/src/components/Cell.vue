<template>
  <div
    :id="`cell[${xPos},${yPos}]`"
    :ref="`cell[${xPos},${yPos}]`"
    class="cell"
    :style="`width: 100%; height: ${height}; background-color: ${
      isAlive ? '#0955c7' : '#FFF'
    };`"
    @click="isAlive = !isAlive"
  ></div>
</template>

<script>
export default {
  props: {
    xPos: {
      type: Number,
      required: true,
    },
    yPos: {
      type: Number,
      required: true,
    },
  },
  data: function () {
    return {
      isAlive: false,
      willBeAlive: false,
      height: "1px",

      isLeftMost: this.xPos == 1,
      isRightMost: this.xPos == this.$parent.gridWidth,

      isUpperMost: this.yPos == 1,
      isLowerMost: this.yPos == this.$parent.gridHeight,
    };
  },
  created() {
    window.addEventListener("resize", this.calculateHeight);
  },
  destroyed() {
    window.removeEventListener("resize", this.calculateHeight);
  },
  mounted() {
    this.calculateHeight();

    this.emitter.on("recalculate-height", () => {
      this.calculateHeight();

      this.$forceUpdate();
    });

    this.emitter.on("next-tick-update", () => {
      this.updateState();
    });

    this.emitter.on("next-tick-commit", () => {
      this.commitState();
    });

    this.emitter.on("reset", () => {
      this.resetState();
    });
  },
  methods: {
    calculateHeight() {
      this.height =
        this.$refs[`cell[${this.xPos},${this.yPos}]`].clientWidth + "px";
    },
    updateState() {
      let countAliveNeighbors = 0;

      // console.log("-".repeat(100) + `\nI am shell ${this.xPos}-${this.yPos}\n`);

      /**
       * UL UC UR
       *  L  X  R
       * DL DC DR
       *
       * UL: cell[${this.xPos - 1},${this.yPos - 1}]
       * UC: cell[${this.xPos},${this.yPos - 1}]
       * UR: cell[${this.xPos + 1},${this.yPos - 1}]
       * L : cell[${this.xPos - 1},${this.yPos}]
       * C : cell[${this.xPos},${this.yPos}]
       * R : cell[${this.xPos + 1},${this.yPos}]
       * DL: cell[${this.xPos - 1},${this.yPos + 1}]
       * DC: cell[${this.xPos},${this.yPos + 1}]
       * DR: cell[${this.xPos + 1},${this.yPos + 1}]
       */

      // UL
      // console.log(`I look at cell ${this.xPos - 1}-${this.yPos - 1}`);
      if (!this.isUpperMost && !this.isLeftMost) {
        if (
          this.$parent.$refs[`cell[${this.xPos - 1},${this.yPos - 1}]`].isAlive
        ) {
          countAliveNeighbors++;
        }
      }

      // UC
      // console.log(`I looked at cell ${this.xPos}-${this.yPos - 1} because`);
      if (!this.isUpperMost) {
        if (this.$parent.$refs[`cell[${this.xPos},${this.yPos - 1}]`].isAlive) {
          countAliveNeighbors++;
        }
      }

      // UR
      // console.log(`I looked at cell ${this.xPos + 1}-${this.yPos - 1} because`);
      if (!this.isUpperMost && !this.isRightMost) {
        if (
          this.$parent.$refs[`cell[${this.xPos + 1},${this.yPos - 1}]`].isAlive
        ) {
          countAliveNeighbors++;
        }
      }

      // L
      // console.log(`I looked at cell ${this.xPos - 1}-${this.yPos} because`);
      if (!this.isLeftMost) {
        if (this.$parent.$refs[`cell[${this.xPos - 1},${this.yPos}]`].isAlive) {
          countAliveNeighbors++;
        }
      }

      // R
      // console.log(`I looked at cell ${this.xPos + 1}-${this.yPos} because`);
      if (!this.isRightMost) {
        if (this.$parent.$refs[`cell[${this.xPos + 1},${this.yPos}]`].isAlive) {
          countAliveNeighbors++;
        }
      }

      // DL
      // console.log(`I looked at cell ${this.xPos - 1}-${this.yPos + 1} because`);
      if (!this.isLowerMost && !this.isLeftMost) {
        if (
          this.$parent.$refs[`cell[${this.xPos - 1},${this.yPos + 1}]`].isAlive
        ) {
          countAliveNeighbors++;
        }
      }

      // DC
      // console.log(`I looked at cell ${this.xPos}-${this.yPos + 1} because`);
      if (!this.isLowerMost) {
        if (this.$parent.$refs[`cell[${this.xPos},${this.yPos + 1}]`].isAlive) {
          countAliveNeighbors++;
        }
      }

      // DR
      // console.log(`I looked at cell ${this.xPos + 1}-${this.yPos + 1} because`);
      if (!this.isLowerMost && !this.isRightMost) {
        if (
          this.$parent.$refs[`cell[${this.xPos + 1},${this.yPos + 1}]`].isAlive
        ) {
          countAliveNeighbors++;
        }
      }

      // console.log(`I found ${countAliveNeighbors} alive cells around me!`);

      if (this.isAlive) {
        if (countAliveNeighbors == 2 || countAliveNeighbors == 3) {
          this.willBeAlive = true;
        } else {
          this.willBeAlive = false;
        }
      } else {
        if (countAliveNeighbors == 3) {
          this.willBeAlive = true;
        } else {
          this.willBeAlive = false;
        }
      }

      // console.log(
      //   `I will be ${this.willBeAlive ? "alive" : "dead"} next tick!\n` +
      //     "-".repeat(100)
      // );
    },
    commitState() {
      this.isAlive = this.willBeAlive;
    },
    resetState() {
      this.isAlive = false;
    },
  },
};
</script>

<style scoped>
.cell {
  display: inline-block;

  border: 1px solid black;
  border-radius: 1rem;
}
</style>