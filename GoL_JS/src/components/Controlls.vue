<template>
  <div class="main">
    <div>play</div>

    <div>reset</div>

    <div>next step</div>

    <div>speed</div>

    <div>grid size</div>
  </div>
</template>

<script>
export default {
  data: function () {
    return {};
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
</style>