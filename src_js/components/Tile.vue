<template>
  <div v-bind:class="classes"
    @mouseover="mouseOver"
    @mouseout="mouseOut"
    @click="play"
  >
    <div v-if="showRoad(0)" class="road-top" />
    <div v-if="showRoad(1)" class="road-right" />
    <div v-if="showRoad(2)" class="road-bottom" />
    <div v-if="showRoad(3)" class="road-left" />
  </div>
</template>

<script>
import TileHelper from '../tile_helper.js'
import _ from 'lodash'

export default {
  props: ['tile', 'playTile', 'getTile', 'hoverTile', 'clearHoverTile', 'row', 'column', 'currentTile', 'roadHovers'],
  methods: {
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
    },
    mouseOver() {
      this.hoverTile(this.tile);
      this.hover = true;
    },
    mouseOut() {
      this.clearHoverTile();
      this.hover = false;
    },
    showRoad(side) {
      return _.includes(this.roadHovers[this.tile.id], side)
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
