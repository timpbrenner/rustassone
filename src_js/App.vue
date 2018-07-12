<template>
  <div>
    <div class="scoreboard">
      <player
        v-for="(player) in players"
        v-bind:player="player"
        v-bind:key="player.id"
      >
    </player>

    </div>
    <div v-if="playerId === null">
      <div class="join">
        <div>JOIN MATCH</div><br />
        <input id="username" type="text"><br />
        <button id="join" @click="join">Submit</button>
      </div>
    </div>
    <div v-else-if="state === 'draw'">
      <button class="draw-button" @click="draw" >
        DRAW
      </button>
    </div>
    <div v-else-if="state === 'action'">
      <current-tile
        v-bind:current-tile="currentTile"
        v-bind:rotate-tile="rotateTile">
      </current-tile>
    </div>
    <div v-else-if="state === 'not_started'">
      <button class="start-button" @click="start" >
        START
      </button>
    </div>
    <div class='game' v-if="grid">
      <tile-row
        v-for="(row, index) in grid"
        v-bind:state="state"
        v-bind:hover-info="hoverInfo"
        v-bind:row="row"
        v-bind:row-index="index"
        v-bind:current-tile="currentTile"
        v-bind:get-tile="getTile"
        v-bind:play-tile="playTile"
        v-bind:play-meeple="playMeeple"
        v-bind:hover-tile="hoverTile"
        v-bind:clear-hover-tile="clearHoverTile"
        v-bind:key="index">
      </tile-row>
    </div>
    <div v-else>
      LOADING
    </div>
  </div>
</template>

<script>
import TileRow from './components/TileRow.vue'
import CurrentTile from './components/CurrentTile.vue'
import Player from './components/Player.vue'
import TileHelper from './tile_helper.js'

export default {
  components: { TileRow, CurrentTile, Player },
  data() {
    return  {
      state: 'draw',
      currentTile: null,
      playerId: null,
      grid: null,
      players: [],
      hoverInfo: { roadHovers: {}, cityHovers: {} },
      gameId: window.location.pathname.split('/').pop(),
    };
  },
  created() {
    $.ajax({
      url: 'http://localhost:8088/game/' + this.gameId,
      success: this.updateGrid,
      error: function() {console.log('CREATED GAME ERROR')},
    });
  },
  methods: {
    join() {
      $.ajax({
        method: 'POST',
        url: 'http://localhost:8088/game/' + this.gameId + '/players',
        contentType: 'application/json',
        data: JSON.stringify({ username: $('#username').val() }),
        success: this.joinPlayer,
        error: function(e) { console.log('Player Join Fail'); },
      });
    },
    joinPlayer(data) {
      this.playerId = data.id;
    },
    updateGrid(data) {
      console.log('---------------');
      console.log(data);
      console.log('---------------');

      this.players = data.players;
      this.grid = data.grid;
      this.state = data.currentState;
      this.currentTurn = data.currentPlayerId;
    },
    start() {
      $.ajax({
        url: 'http://localhost:8088/game/' + this.gameId + '/start',
        success: this.updateGrid,
        error: function(e) { console.log('Start Fail'); },
      });
    },
    draw() {
      $.ajax({
        url: 'http://localhost:8088/game/' + this.gameId + '/tiles',
        success: this.drawTile,
        error: function(e) { console.log('Draw Fail'); },
      });
    },
    drawTile(tile) {
      this.currentTile = Object.assign({}, tile, { playerId: this.playerId });
      this.state = 'action';
    },
    getTile(row, column) {
      const rowObj = this.grid[row];
      if (!rowObj) return {};

      const tile = rowObj[column];
      if (!tile) return {};

      return tile;
    },
    rotateTile() {
      this.currentTile = Object.assign({}, this.currentTile, {
        cities: _.flatMap(this.currentTile.cities, function(side) { return (side + 1) % 4; }),
        roads: _.flatMap(this.currentTile.roads, function(side) { return (side + 1) % 4; }),
      })
    },
    playTile(row, column, rowOffset, columnOffset) {
      $.ajax({
        method: 'POST',
        url: 'http://localhost:8088/game/' + this.gameId + '/play',
        contentType: 'application/json',
        data: JSON.stringify({
          tile_id: this.currentTile.id,
          player_id: this.playerId,
          row_offset: rowOffset,
          column_offset: columnOffset,
        }),
        success: this.updateGrid,
        error: function(d) { console.log('Error playing tile'); }
      });
    },
    playMeeple(tileId, side) {
      $.ajax({
        method: 'POST',
        url: 'http://localhost:8088/game/' + this.gameId + '/meeple',
        contentType: 'application/json',
        data: JSON.stringify({
          player_id: this.playerId,
          tile_id: tileId,
          side: side,
        }),
        success: function(d) { console.log('played meeple'); },
        error: function(d) { console.log('Error playing meeple'); }
      });
    },
    hoverTile(tile, row, column, side, sideType) {
      if (!tile.playerId) {
        return;
      }

      this.hoverInfo = TileHelper.getHoverInfo(this.grid, row, column, side, this.playerId, sideType);
    },
    clearHoverTile() {
      this.hoverInfo = {
        roadHovers: {},
        cityHovers: {},
      };
    },
  }
}
</script>
