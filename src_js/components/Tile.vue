<template>
  <div v-bind:class="classes" @click="play">
    <div
      @mouseover="mouseOver(0)"
      @mouseout="mouseOut"
      v-bind:class="innerComponentClasses(0)"
    />
    <div
      @mouseover="mouseOver(1)"
      @mouseout="mouseOut"
      v-bind:class="innerComponentClasses(1)"
    />
    <div
      @mouseover="mouseOver(2)"
      @mouseout="mouseOut"
      v-bind:class="innerComponentClasses(2)"
    />
    <div
      @mouseover="mouseOver(3)"
      @mouseout="mouseOut"
      v-bind:class="innerComponentClasses(3)"
    />
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
    mouseOver(side) {
      this.hoverTile(this.tile, this.row, this.column, side);
      this.hover = true;
    },
    mouseOut() {
      this.clearHoverTile();
      this.hover = false;
    },
    innerComponentClasses(side) {
      const sideName = ['top', 'right', 'bottom', 'left'][side];

      if (_.includes(this.tile.roads, side)) {
        if (_.includes(this.roadHovers[this.tile.id], side)) {
          return 'road-' + sideName + ' road-' + sideName + '-hover';
        }

        return 'road-' + sideName;
      } else if (_.includes(this.tile.cities, side)) {
        // if (_.includes(this.cityHovers[this.tile.id], side)) {
        //   return 'city-' + sideName + ' city-' + sideName + '-hover';
        // }

        return 'city-' + sideName;
      }
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
    },
  }
}
</script>
