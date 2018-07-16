<template>
  <div class="current-tile" @click="rotate">
    <div v-bind:class="innerComponentClasses(0)" />
    <div v-bind:class="innerComponentClasses(1)" />
    <div v-bind:class="innerComponentClasses(2)" />
    <div v-bind:class="innerComponentClasses(3)" />
  </div>
</template>

<script>
import TileHelper from '../tile_helper.js'

export default {
  props: ['currentTile', 'rotateTile', 'currentRotation'],
  methods: {
    rotate() {
      this.rotateTile();
    },
    innerComponentClasses(side) {
      const rotatedSide = (side + this.currentRotation) % 4;
      const sideName = ['top', 'right', 'bottom', 'left'][rotatedSide];
      const sideType = TileHelper.sideType(this.currentTile, side);

      if (sideType === 2) {
        return 'road-' + sideName;
      } else if (sideType === 1) {
        return 'city-' + sideName;
      }
    }
  },
}
</script>
