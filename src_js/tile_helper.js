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

  getRoadInfo(grid, row, column, side, playerId, roadData = { ends: false, players: {}, hovers: {} }) {
    const tile = TileHelper.getTile(grid, row, column);

    console.log('Get Road Info -- Row: ' + row + ' Column: ' + column + ' Side: ' + side);
    console.log(roadData);

    // Recursive Case
    if (_.includes(roadData.hovers[tile.id], side)) {
      return roadData;
    }

    if (!tile || !tile.id) {
      return Object.assign(roadData, { ends: true });
    }

    let newroadData = Object.assign({}, roadData);

    // PLAYERS
    if (playerId && !_.includes(newroadData.players, playerId)) {
      newroadData.players.push(playerId);
    }

    // HOVERS
    if (!newroadData.hovers[tile.id]) {
      newroadData.hovers[tile.id] = [side];
    } else {
      newroadData.hovers[tile.id].push(side)
    }

    _.each(tile.roads, (rSide) => {
      if (!_.includes(roadData.hovers[tile.id], rSide)) {
        newroadData = TileHelper.getRoadInfo(grid, row, column, rSide, playerId, newroadData);
        return;
      }

      if (rSide == 0) {
        newroadData = TileHelper.getRoadInfo(grid, row - 1, column, 2, playerId, newroadData);
      } else if (rSide == 1) {
        newroadData = TileHelper.getRoadInfo(grid, row, column + 1, 3, playerId, newroadData);
      } else if (rSide == 2) {
        newroadData = TileHelper.getRoadInfo(grid, row + 1, column, 0, playerId, newroadData);
      } else if (rSide == 3) {
        newroadData = TileHelper.getRoadInfo(grid, row, column - 1, 1, playerId, newroadData);
      }
    });

    return newroadData;
  },

  getTile(grid, row, column) {
    const rowObj = grid[row];
    if (!rowObj) return {};

    const tile = rowObj[column];
    if (!tile) return {};

    return tile;
  },
};

module.exports = TileHelper;
