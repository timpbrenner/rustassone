use schema::tiles;

#[derive(Queryable, QueryableByName)]
#[table_name = "tiles"]
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

#[derive(Queryable)]
pub struct Game {
  pub id: i32,
  pub current_player_id: Option<i32>,
  pub current_state: Option<String>,
}

#[derive(Queryable)]
pub struct Player {
  pub id: i32,
  pub username: Option<String>,
}

#[derive(Queryable)]
pub struct GamePlayer {
  pub id: i32,
  pub game_id: i32,
  pub player_id: i32,
}

#[derive(Queryable)]
pub struct GameTile {
  pub id: i32,
  pub game_id: i32,
  pub tile_id: i32,
  pub player_id: Option<i32>,
  pub meeple_location: Option<i32>,
  pub row_offset: i32,
  pub column_offset: i32,
}

#[derive(Queryable)]
pub struct GamePiece {
  pub id: i32,
  pub game_id: i32,
  pub tile_id: i32,
  pub player_id: i32,
  pub side: i32,
}
