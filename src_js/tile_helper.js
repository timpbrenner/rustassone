const TileHelper = {
  sideType(tile, side) {
    if (tile.cities.indexOf(side) >= 0) { return 1;
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
  },

  getRoadInfo(grid, row, column, side, playerId, roadData = { ends: false, roadCount: 1, players: {}, hovers: {} }) {
    const tile = TileHelper.getTile(grid, row, column);

    console.log('Get Road Info ------------');
    console.log('Row: ' + row + ' Column: ' + column + ' Side: ' + side);
    console.log(roadData);
    console.log(tile);

    // Recursive Case
    if (_.includes(hovers[tile.id], side)) {
      console.log('NO RECURSE, ALREADY EXISTS');

      return roadData;
    }

    if (!tile || !tile.id) {
      console.log('NO RECURSE, REACHED END');

      return Object.assign(roadData, { ends: true });
    }
    console.log('RECURSE');

    // ROAD COUNT
    let newroadData = Object.assign({}, roadData, {
      roadCount: roadData.roadCount + 1,
    });

    // PLAYERS
    if (playerId && !_.includes(newroadData.players, playerId)) {
      newroadData.players.push(playerId);
    }

    // HOVERS
    newroadData.hovers = Object.assign(
      newroadData.hovers,
      {
        [tile.id]: newroadData.hovers.push(side),
      }
    );

    _.each(tile.roads, (rSide) => {
      newroadData = TileHelper.getRoadInfo(grid, tile, rSide, playerId, newroadData);

      if (side == 0) {
        newroadData = TileHelper.getRoadInfo(grid, row - 1, column, 2, playerId, newroadData);
      } else if (side == 1) {
        newroadData = TileHelper.getRoadInfo(grid, row, column + 1, 3, playerId, newroadData);
      } else if (side == 2) {
        newroadData = TileHelper.getRoadInfo(grid, row + 1, column, 0, playerId, newroadData);
      } else if (side == 3) {
        newroadData = TileHelper.getRoadInfo(grid, row, column - 1, 1, playerId, newroadData);
      }
    });

    return newroadData;
  },

  getTile(grid, row, column) {
    const rowObj = this.grid[row];
    if (!rowObj) return {};

    const tile = rowObj[column];
    if (!tile) return {};

    return tile;
  },
};

module.exports = TileHelper;
