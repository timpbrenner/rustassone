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
  },

  getHoverInfo(grid, row, column, side, playerId, sideType, hoverData = { ends: false, players: {}, roadHovers: {}, cityHovers: {} }) {
    const tile = TileHelper.getTile(grid, row, column);
    const hoverKey = sideType === 2 ? 'roadHovers' : 'cityHovers';

    // Recursive Case
    if (_.includes(hoverData[hoverKey][tile.id], side)) {
      return hoverData;
    }

    if (!tile || !tile.id) {
      return Object.assign(hoverData, { ends: true });
    }

    let newHoverData = Object.assign({}, hoverData);

    // PLAYERS
    if (playerId && !_.includes(newHoverData.players, playerId)) {
      newHoverData.players.push(playerId);
    }

    // HOVERS
    if (!newHoverData[hoverKey][tile.id]) {
      newHoverData[hoverKey][tile.id] = [side];
    } else {
      newHoverData[hoverKey][tile.id].push(side)
    }

    let sides = [];
    if (sideType === 2) {
      sides = tile.roads;
    } else if (sideType === 1) {
      sides = tile.cities;
    }

    _.each(sides, (rSide) => {
      if (!_.includes(hoverData[hoverKey][tile.id], rSide)) {
        newHoverData = TileHelper.getHoverInfo(grid, row, column, rSide, playerId, sideType, newHoverData);
        return;
      }

      const rowOffset = [-1, 0, 1, 0];
      const columnOffset = [0, 1, 0, -1];
      const mirrorSide = [2, 3, 0, 1];

      newroadData = TileHelper.getHoverInfo(
        grid,
        row + rowOffset[rSide],
        column + columnOffset[rSide],
        mirrorSide[rSide],
        playerId,
        sideType,
        newHoverData
      );
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
