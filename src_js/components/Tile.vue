<template>
  <div
    @mouseover="tileMouseOver"
    @mouseout="tileMouseOut"
    v-bind:class="classes"
    @click="play"
  >
    <div v-bind:class="meeplePlacement()" />
    <div
      @mouseover="mouseOver(0)"
      @mouseout="mouseOut"
      @click="placeMeeple(0)"
      v-bind:class="innerComponentClasses(0)"
    />
    <div
      @mouseover="mouseOver(1)"
      @mouseout="mouseOut"
      @click="placeMeeple(1)"
      v-bind:class="innerComponentClasses(1)"
    />
    <div
      @mouseover="mouseOver(2)"
      @mouseout="mouseOut"
      @click="placeMeeple(2)"
      v-bind:class="innerComponentClasses(2)"
    />
    <div
      @mouseover="mouseOver(3)"
      @mouseout="mouseOut"
      @click="placeMeeple(3)"
      v-bind:class="innerComponentClasses(3)"
    />
  </div>
</template>

<script>
import TileHelper from '../tile_helper.js'
import _ from 'lodash'

export default {
  props: ['tile', 'playTile', 'playMeeple', 'getTile', 'hoverTile', 'clearHoverTile', 'row', 'column', 'currentTile', 'hoverInfo'],
  methods: {
    anySurround() {
      return this.getTile(this.row - 1, this.column).playerId ||
        this.getTile(this.row, this.column - 1).playerId ||
        this.getTile(this.row + 1, this.column).playerId ||
        this.getTile(this.row, this.column + 1).playerId;
    },
    play() {
      if (this.tile.id) { return; }
      if (!this.anySurround()) { return; }
      if (!TileHelper.matchesSurrounding(this.currentTile, this.getTile, this.row, this.column)) { return; }

      this.playTile(this.row, this.column, this.tile.rowOffset, this.tile.columnOffset);
    },
    placeMeeple(side, e) {
      this.playMeeple(this.tile.id, side);
    },
    tileMouseOver(side) {
      this.hover = true;
    },
    tileMouseOut(side) {
      this.hover = false;
    },
    mouseOver(side) {
      const sideType = TileHelper.sideType(this.tile, side);

      this.hoverTile(this.tile, this.row, this.column, side, sideType);
      this.hover = true;
    },
    mouseOut() {
      this.clearHoverTile();
    },
    meeplePlacement() {
      if (this.tile.meeple && this.tile.meeple.playerId) {
        const sideName = ['top', 'right', 'bottom', 'left'][this.tile.meeple.side];
        return 'meeple meeple-' + sideName;
      }
    },
    innerComponentClasses(side) {
      const sideName = ['top', 'right', 'bottom', 'left'][side];
      const sideType = TileHelper.sideType(this.tile, side);

      if (sideType === 2) {
        if (_.includes(this.hoverInfo.roadHovers[this.tile.id], side)) {
          return 'road-' + sideName + ' road-' + sideName + '-hover';
        }

        return 'road-' + sideName;
      } else if (sideType === 1) {
        if (_.includes(this.hoverInfo.cityHovers[this.tile.id], side)) {
          return 'city-' + sideName + ' city-' + sideName + '-hover';
        }

        return 'city-' + sideName;
      }
    }
  },
  data: function() {
    return { hover: false };
  },
  computed: {
    classes: function() {
      if (this.tile.playerId) {
        return 'tile played';
      }

      const klasses = ["tile"];
      const matches = this.currentTile && this.currentTile.playerId && TileHelper.matchesSurrounding(this.currentTile, this.getTile, this.row, this.column);
      const surround = this.anySurround();

      if (matches && surround && this.hover) {
        klasses.push("playable-hover");

      } else if (surround && this.hover) {
        klasses.push("hover");

      } else if (this.anySurround()) {
        klasses.push("active");
      }

      return klasses.join(' ');
    },
  }
}
</script>
