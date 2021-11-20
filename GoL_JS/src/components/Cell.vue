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
  mounted() {
    console.log(this.$parent.gridWidth);
    this.height =
      this.$refs[`cell[${this.xPos},${this.yPos}]`].clientWidth + "px";
  },
  methods: {
    updateState() {
      let countAliveNeighbours = 0;

      /**
       * UL UC UR
       *  L  X  R
       * DL DC DR
       *
       * UL: cell[${this.xPos - 1},${this.yPos - 1}]
       * UC: cell[${this.xPos - 1},${this.yPos}]
       * UR: cell[${this.xPos - 1},${this.yPos + 1}]
       * L : cell[${this.xPos},${this.yPos - 1}]
       * C : cell[${this.xPos},${this.yPos}]
       * R : cell[${this.xPos},${this.yPos + 1}]
       * DL: cell[${this.xPos + 1},${this.yPos - 1}]
       * DC: cell[${this.xPos + 1},${this.yPos}]
       * DR: cell[${this.xPos + 1},${this.yPos + 1}]
       */

      // UL
      if (
        this.$refs[`cell[${this.xPos - 1},${this.yPos - 1}]`].isAlive &&
        !this.isUpperMost &&
        !this.isLeftMost
      ) {
        countAliveNeighbours++;
      }

      // UC
      if (
        this.$refs[`cell[${this.xPos - 1},${this.yPos}]`].isAlive &&
        !this.isUpperMost
      ) {
        countAliveNeighbours++;
      }

      // UR
      if (
        this.$refs[`cell[${this.xPos - 1},${this.yPos + 1}]`].isAlive &&
        !this.isUpperMost &&
        !this.isRightMost
      ) {
        countAliveNeighbours++;
      }

      // L
      if (
        this.$refs[`cell[${this.xPos},${this.yPos - 1}]`].isAlive &&
        !this.isLeftMost
      ) {
        countAliveNeighbours++;
      }

      // R
      if (
        this.$refs[`cell[${this.xPos},${this.yPos + 1}]`].isAlive &&
        !this.isRightMost
      ) {
        countAliveNeighbours++;
      }

      // DL
      if (
        this.$refs[`cell[${this.xPos + 1},${this.yPos - 1}]`].isAlive &&
        !this.isLowerMost &&
        !this.isLeftMost
      ) {
        countAliveNeighbours++;
      }

      // DC
      if (
        this.$refs[`cell[${this.xPos + 1},${this.yPos}]`].isAlive &&
        !this.isLowerMost
      ) {
        countAliveNeighbours++;
      }

      // DR
      if (
        this.$refs[`cell[${this.xPos + 1},${this.yPos + 1}]`].isAlive &&
        !this.isLowerMost &&
        !this.isRightMost
      ) {
        countAliveNeighbours++;
      }

      if (this.isAlive) {
        if (countAliveNeighbours == 2 || countAliveNeighbours == 3) {
          this.willBeAlive = true;
        } else {
          this.willBeAlive = false;
        }
      } else {
        if (countAliveNeighbours == 3) {
          this.willBeAlive = true;
        } else {
          this.willBeAlive = false;
        }
      }
    },
    commitState() {
      this.isAlive = this.willBeAlive;
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