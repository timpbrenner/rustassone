#[derive(Queryable)]
pub struct Tile {
    pub id: i32,
}

#[derive(Queryable)]
pub struct TileCity {
    pub id: i32,
    pub tile_id: i32,
    pub city_side: i32,
    pub tile_group: i32,
}

#[derive(Queryable)]
pub struct TileRoad {
    pub id: i32,
    pub tile_id: i32,
    pub road_side: i32,
    pub tile_group: i32,
}
