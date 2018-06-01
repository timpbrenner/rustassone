<template>
  <div v-bind:class="classes"
    @mouseover="checkHover"
    @mouseout="hover = false"
    @click="play"
  />
</template>

<script>
import TileHelper from '../tile_helper.js'

export default {
  props: ['tile', 'playTile', 'getTile', 'row', 'column', 'currentTile'],
  methods: {
    checkHover: function() {
      this.hover = true;
    },
    anySurround() {
      return this.getTile(this.row - 1, this.column).playerId ||
        this.getTile(this.row, this.column - 1).playerId ||
        this.getTile(this.row + 1, this.column).playerId ||
        this.getTile(this.row, this.column + 1).playerId;
    },
    play() {
      if (!this.anySurround()) { return; }
      if (!TileHelper.matchesSurrounding(this.currentTile, this.getTile, this.row, this.column)) { return; }

      this.playTile(this.row, this.column, this.tile.rowOffset, this.tile.columnOffset)
    }
  },
  data: function() {
    return { hover: false };
  },
  computed: {
    classes: function() {
      let klasses = ["tile"];

      if (this.tile.playerId) {
        klasses.push("played");
        return klasses.concat(TileHelper.sideClasses(this.tile)).join(' ');
      }

      const matches = this.currentTile && this.currentTile.playerId && TileHelper.matchesSurrounding(this.currentTile, this.getTile, this.row, this.column);
      const surround = this.anySurround();

      if (matches && surround && this.hover) {
        klasses.push("playable-hover");

      } else if (surround && this.hover) {
        klasses.push("hover");

      } else if (this.anySurround()) {
        klasses.push("active");
      }

      return klasses.concat(TileHelper.sideClasses(this.tile)).join(' ');
    }
  }
}
</script>
