<template>
  <div class="row">
    <tile
      v-for="(tile, index) in row"
      v-bind:tile="tile"
      v-bind:state="state"
      v-bind:hover-info="hoverInfo"
      v-bind:current-tile="currentTile"
      v-bind:current-rotation="currentRotation"
      v-bind:get-tile="getTile"
      v-bind:play-tile="playTile"
      v-bind:play-meeple="playMeeple"
      v-bind:hover-tile="hoverTile"
      v-bind:clear-hover-tile="clearHoverTile"
      v-bind:row="rowIndex"
      v-bind:column="index"
      v-bind:key="index"
      />
  </div>
</template>

<script>
import Tile from './Tile.vue'

export default {
  props: ['row', 'rowIndex', 'playTile', 'playMeeple', 'getTile', 'hoverTile', 'clearHoverTile', 'currentTile', 'hoverInfo', 'state', 'currentRotation'],
  components: { Tile },
}
</script>
