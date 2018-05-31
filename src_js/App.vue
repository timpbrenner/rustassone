<template>
  <div>
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
        v-bind:row="row"
        v-bind:row-index="index"
        v-bind:current-tile="currentTile"
        v-bind:get-tile="getTile"
        v-bind:play-tile="playTile"
        v-bind:key="index">
      </tile-row>
    </div>
    <div v-else>
      LOADING
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return  {
      state: 'draw',
      currentTile: null,
      playerId: null,
      grid: null,
      gameId: window.location.pathname.split('/').pop(),
    };
  },
  created() {
    return function () {
      $.ajax({
        url: 'http://localhost:8088/game/' + this.gameId,
        success: this.updateGrid,
        error: function() {console.log('CREATED GAME ERROR')},
      });
    };
  },
  methods: {
    join: function() {
      console.log($('#username').val());
      $.ajax({
        method: 'POST',
        url: 'http://localhost:8088/game/' + this.gameId + '/players',
        contentType: 'application/json',
        data: JSON.stringify({ username: $('#username').val() }),
        success: this.joinPlayer,
        error: function(e) { console.log('Player Join Fail'); },
      });
    },
    joinPlayer: function(data) {
      this.playerId = data.id;
    },
    updateGrid: function(data) {
      this.grid = data.grid;
      this.state = data.currentState;
      this.currentTurn = data.currentPlayerId;
    },
    start: function() {
      $.ajax({
        url: 'http://localhost:8088/game/' + this.gameId + '/start',
        success: this.updateGrid,
        error: function(e) { console.log('Start Fail'); },
      });
    },
    draw: function() {
      $.ajax({
        url: 'http://localhost:8088/game/' + this.gameId + '/tiles',
        success: this.drawTile,
        error: function(e) { console.log('Draw Fail'); },
      });
    },
    drawTile: function(tile) {
      this.currentTile = Object.assign({}, tile, { playerId: this.playerId });
      this.state = 'action';
    },
    getTile: function(row, column) {
      const rowObj = this.grid[row];
      if (!rowObj) return {};

      const tile = rowObj[column];
      if (!tile) return {};

      return tile;
    },
    rotateTile: function() {
      this.currentTile = Object.assign({}, this.currentTile, {
        cities: _.flatMap(this.currentTile.cities, function(side) { return (side + 1) % 4; }),
        roads: _.flatMap(this.currentTile.roads, function(side) { return (side + 1) % 4; }),
      })
    },
    playTile: function(row, column, rowOffset, columnOffset) {
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
        success: function(d) { console.log('played'); },
        error: function(d) { console.log('Error playing tile'); }
      });

      this.placeTile(row, column);
    },
    placeTile: function(row, column) {
      let rowOffset = 0;
      let columnOffset = 0;
      const newGrid = _.cloneDeep(this.grid);
      const rowLength = newGrid.length;
      const columnLength = newGrid[0].length;
      newGrid[row][column] = Object.assign(this.grid[row][column], this.currentTile);

      if (row === 0) {
        // add row in the beginning
        const newRow = [];
        rowOffset = this.grid[0][0].rowOffset - 1;
        columnOffset = this.grid[0][0].columnOffset;
        for (i = 0; i < columnLength; i++) {
          newRow.push({ rowOffset: rowOffset, columnOffset: columnOffset + i });
        }
        newGrid.unshift(newRow);
      } else if (row === this.grid.length - 1) {
        // add row at end
        const newRow = [];
        rowOffset = this.grid[rowLength - 1][0].rowOffset + 1;
        columnOffset = this.grid[0][0].columnOffset;
        for (i = 0; i < columnLength; i++) {
          newRow.push({ rowOffset: rowOffset, columnOffset: columnOffset + i });
        }

        newGrid.push(newRow);
      }

      if (column === 0) {
        // add column at beginning of each row
        rowOffset = this.grid[0][0].rowOffset;
        columnOffset = this.grid[0][0].columnOffset - 1;
        newGrid.forEach(function(row, i) {
          row.unshift({ columnOffset: columnOffset, rowOffset: rowOffset + i});
        })
      } else if (column === columnLength - 1) {
        // add column at end of each row
        rowOffset = this.grid[0][0].rowOffset;
        columnOffset = this.grid[0][columnLength - 1].columnOffset + 1;
        newGrid.forEach(function(row, i) {
          row.push({ columnOffset: columnOffset, rowOffset: rowOffset + i});
        })
      }

      this.currentTile = {};
      this.state = 'draw';
      this.grid = newGrid;
    }
  }
}
</script>
