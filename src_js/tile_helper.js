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
  }
};

module.exports = TileHelper;
