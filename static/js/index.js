const TileHelper = {
  sideType(tile, side) {
    if (tile.cities.indexOf(side) >= 0) {
      return 1;
    } else if (tile.roads.indexOf(side) >= 0) {
      return 2;
    }
    return 0;
  },

  sideClasses(tile) {
    if (!tile.playerId) return[];

    return [
      "top-" + TileHelper.sideClass(tile, 0),
      "right-" + TileHelper.sideClass(tile, 1),
      "bottom-" + TileHelper.sideClass(tile, 2),
      "left-" + TileHelper.sideClass(tile, 3),
    ];
  },

  sideClass(tile, side) {
    return ["farm", "city", "road"][TileHelper.sideType(tile, side)];
  },

  matchesSurrounding(tile, getTile, row, column) {
    const above = getTile(row - 1, column);
    const right = getTile(row, column + 1);
    const below = getTile(row + 1, column);
    const left = getTile(row, column - 1);

    return (!above.playerId || TileHelper.sideType(tile, 0) === TileHelper.sideType(above, 2)) &&
     (!right.playerId || TileHelper.sideType(tile, 1) === TileHelper.sideType(right, 3)) &&
     (!below.playerId || TileHelper.sideType(tile, 2) === TileHelper.sideType(below, 0)) &&
     (!left.playerId || TileHelper.sideType(tile, 3) === TileHelper.sideType(left, 1));
  }
};

Vue.component('current-tile', {
  props: ['currentTile', 'rotateTile'],
  template: '<div v-bind:class="classes"\
    @click="rotate">\
  </div>',
  methods: {
    rotate() {
      this.rotateTile();
    }
  },
  computed: {
    classes() {
      return ['current-tile'].concat(TileHelper.sideClasses(this.currentTile)).join(' ');
    }
  }
})

Vue.component('tile', {
  props: ['tile', 'playTile', 'getTile', 'row', 'column', 'currentTile'],
  template: '\
  <div v-bind:class="classes"\
    @mouseover="checkHover"\
    @mouseout="hover = false"\
    @click="play"\
  />\
  ',
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
})

Vue.component('tile-row', {
  props: ['row', 'rowIndex', 'playTile', 'getTile', 'currentTile'],
  template: '<div class="row">\
    <tile\
       v-for="(tile, index) in row"\
       v-bind:tile="tile"\
       v-bind:current-tile="currentTile"\
       v-bind:get-tile="getTile"\
       v-bind:play-tile="playTile"\
       v-bind:row="rowIndex"\
       v-bind:column="index"\
       v-bind:key="index"\
     ></tile>\
  </div>'
})

var app = new Vue({
  el: '#app',
  data: {
    state: 'draw',
    currentTile: null,
    playerId: null,
    grid: null,
    gameId: window.location.pathname.split('/').pop(),
  },
  created: function () {
    $.ajax({
      url: 'http://localhost:8000/game/' + this.gameId + '/current',
      success: this.updateGrid,
    });
  },
  methods: {
    join: function() {
      $.ajax({
        url: 'http://localhost:8000/game/' + this.gameId + '/join',
        data: { username: $('#username').val() },
        success: this.joinPlayer,
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
        url: 'http://localhost:8000/game/' + this.gameId + '/start',
        success: this.updateGrid,
      });
    },
    draw: function() {
      $.ajax({
        url: 'http://localhost:8000/game/' + this.gameId + '/draw',
        success: this.drawTile,
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
        url: 'http://localhost:8000/game/' + this.gameId + '/play',
        data: {
          tile_id: this.currentTile.id,
          player_id: this.playerId,
          row_offset: rowOffset,
          column_offset: columnOffset,
        },
        success: function(d) { }
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
})
