<template>
  <div class="main">
    <div class="play" @click="togglePlayState">
      <Fa :icon="['fas', 'play']" v-if="!isPlaying" />
      <Fa :icon="['fas', 'pause']" v-else />
    </div>

    <div><button v-on:click="resetBoard">Empty Board</button></div>

    <div>
      <button v-on:click="fireNextTick">Next Tick</button>
    </div>

    <div>
      <h3>Tick Delay</h3>
      <input
        id="tick-delay-slider"
        type="range"
        v-model="tickDelay"
        :min="250"
        :max="5000"
        :disabled="isPlaying"
        style="display: inline-block"
        @change="submitNewTickRate"
      />
      <input
        id="tick-delay-number"
        type="number"
        v-model="tickDelay"
        :min="250"
        :max="5000"
        :disabled="isPlaying"
        style="display: inline-block"
        @change="submitNewTickRate"
      />
    </div>

    <div class="size-inputs">
      <h3>Grid Size</h3>

      <div style="display: inline-block">
        <label for="width">Width</label>
        <input type="number" id="width" v-model="width" />

        <br />

        <label for="height">Height</label>
        <input type="number" id="height" v-model="height" />
      </div>

      <button
        style="display: inline-block; vertical-align: center"
        @click="submitNewSizes"
      >
        Change
      </button>
    </div>
  </div>
</template>

<script>
import Slider from "@vueform/slider";

export default {
  data: function () {
    return {
      isPlaying: false,
      width: this.$parent.gridWidth,
      height: this.$parent.gridHeight,
      tickDelay: this.$parent.tickDelay,
    };
  },
  components: {
    Slider,
  },
  methods: {
    togglePlayState() {
      if (this.isPlaying) {
        this.emitter.emit("stop");
      } else {
        this.emitter.emit("play");
      }

      this.isPlaying = !this.isPlaying;
    },
    fireNextTick() {
      this.emitter.emit("next-tick");
    },
    submitNewSizes() {
      this.$parent.gridWidth = this.width;
      this.$parent.gridHeight = this.height;

      this.emitter.emit("recalculate-grid");
    },
    submitNewTickRate() {
      this.$parent.tickDelay = parseInt(this.tickDelay);

      this.emitter.emit("change-tick-delay");
    },
    resetBoard() {
      this.emitter.emit("reset");
    },
  },
};
</script>

<style scoped>
.main {
  margin: auto;

  display: flex;
  flex: 1;
  flex-direction: row;

  align-items: center;
  align-content: center;
  vertical-align: middle;
  justify-content: center;
}

.main > div {
  margin-left: 2rem;
  margin-right: 2rem;
}

.play {
  font-size: 2rem;
  font-weight: bold;

  cursor: pointer;
}
</style>