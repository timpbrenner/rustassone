//use diesel;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::insert_into;
use rocket_contrib::Json;

use models::*;
use lib::*;
use std;

#[derive(Serialize)]
pub struct BlahTile {
    pub id: i32,
    pub cities: Vec<i32>,
    pub roads: Vec<i32>,
}

#[derive(Serialize)]
pub struct BlahGame {
    pub id: i32,
    pub grid: Vec<Vec<BlahTile>>,
    pub current_player_id: i32,
    pub current_state: String,
}

#[derive(FromForm)]
pub struct TilePlay {
    pub tile_id: Option<i32>,
    pub player_id: Option<i32>,
    pub row_offset: Option<i32>,
    pub column_offset: Option<i32>
}

pub fn get_game(_current_game_id: String) -> Json<BlahGame> {
    let tile = BlahTile { id: 1, cities: Vec::new(), roads: Vec::new() };
    let grid = vec![vec![tile]];

    Json(BlahGame { id: 1, grid: grid, current_player_id: 1, current_state: String::from("action") })
}

pub fn draw_tile(current_game_id: String) -> Json<BlahTile> {
    use schema::tile_roads::dsl::*;
    use schema::tile_cities::dsl::*;

    let connection = establish_connection();
    let sql = format!("SELECT t.* FROM tiles t \
                    LEFT JOIN game_tiles gt ON gt.tile_id = t.id and gt.game_id = {} \
                    WHERE gt.id IS NULL \
                    ORDER BY random() LIMIT 1", current_game_id);
    let tile_result: std::result::Result<Tile, ::diesel::result::Error> = sql_query(sql).get_result(&connection);
    //println!("OK: {}", tile_result.is_ok());

    // HANDLE NO TILES MEANS GAME OVER
    let tile = tile_result.ok().unwrap();
    let cities = tile_cities.filter(::schema::tile_cities::dsl::tile_id.eq(tile.id))
                    .load::<TileCity>(&connection).expect("Error loading tile cities");

    let roads = tile_roads.filter(::schema::tile_roads::dsl::tile_id.eq(tile.id))
                    .load::<TileRoad>(&connection).expect("Error loading tile cities");

    let mut output = BlahTile { id: tile.id, cities: Vec::new(), roads: Vec::new() };
    for city in cities {
        output.cities.push(city.city_side);
    }

    for road in roads {
        output.roads.push(road.road_side);
    }

    Json(output)
}

pub fn play_tile(current_game_id: String, play: TilePlay) -> Json<BlahTile> {
    use schema::game_tiles::dsl::*;

    let int_game_id:i32 = current_game_id.parse().unwrap();
    let conn = establish_connection();
    insert_into(game_tiles)
        .values(
            (
                game_id.eq(int_game_id),
                tile_id.eq(play.tile_id.unwrap()),
                player_id.eq(play.player_id.unwrap()),
                row_offset.eq(play.row_offset.unwrap()),
                column_offset.eq(play.column_offset.unwrap()),
            )
        ).execute(&conn);

    Json(BlahTile { id: 1, cities: Vec::new(), roads: Vec::new() })
}
