use diesel::prelude::*;
use diesel::sql_query;
use diesel::insert_into;

use models::*;
use lib::*;
use std;

#[derive(Serialize)]
pub struct JsTile {
    pub id: i32,
    pub playerId: i32,
    pub cities: Vec<i32>,
    pub roads: Vec<i32>,
    pub rowOffset: i32,
    pub columnOffset: i32,
}

#[derive(Serialize)]
pub struct JsGame {
    pub id: i32,
    pub grid: Vec<Vec<JsTile>>,
    pub currentPlayerId: i32,
    pub currentState: String,
}

#[derive(FromForm)]
pub struct TilePlay {
    pub tile_id: Option<i32>,
    pub player_id: Option<i32>,
    pub row_offset: Option<i32>,
    pub column_offset: Option<i32>
}

pub fn get_game(game_id: i32) -> JsGame {
    use schema::games::dsl::*;

    let connection = establish_connection();
    let game = games.filter(::schema::games::dsl::id.eq(game_id)).get_result::<Game>(&connection).expect("Error loading games");
    let grid = build_game_grid(game.id, connection);

    JsGame { id: game.id, grid: grid, currentPlayerId: game.current_player_id.unwrap_or(-1), currentState: game.current_state.unwrap_or("draw".to_owned()) }
}

pub fn draw_tile(current_game_id: String) -> JsTile {
    let connection = establish_connection();
    let sql = format!("SELECT t.* FROM tiles t \
                    LEFT JOIN game_tiles gt ON gt.tile_id = t.id and gt.game_id = {} \
                    WHERE gt.id IS NULL \
                    ORDER BY random() LIMIT 1", current_game_id);
    let tile_result: std::result::Result<Tile, ::diesel::result::Error> = sql_query(sql).get_result(&connection);

    get_tile(tile_result.ok().unwrap(), &connection)
}

pub fn play_tile(current_game_id: String, play: TilePlay) -> JsTile {
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

    JsTile { id: 1, playerId: 0, cities: Vec::new(), roads: Vec::new(), columnOffset: 0, rowOffset: 0 }
}

// Private

fn build_game_grid(current_game_id: i32, connection: PgConnection) -> Vec<Vec<JsTile>> {
    use schema::game_tiles::dsl::*;
    use schema::tiles::dsl::*;

    let played_tiles = game_tiles.filter(game_id.eq(current_game_id)).load::<GameTile>(&connection).expect("Error loading game tiles");

    let max_column_offset = played_tiles.iter().max_by_key(|x| x.column_offset).unwrap().column_offset + 1;
    let min_column_offset = played_tiles.iter().min_by_key(|x| x.column_offset).unwrap().column_offset - 1;
    let max_row_offset = played_tiles.iter().max_by_key(|x| x.row_offset).unwrap().row_offset + 1;
    let min_row_offset = played_tiles.iter().min_by_key(|x| x.row_offset).unwrap().row_offset - 1;

    let mut grid: Vec<Vec<JsTile>> = Vec::new();
    for row_offset_i in min_row_offset..(max_row_offset + 1) {
        let mut row: Vec<JsTile> = Vec::new();

        for column_offset_i in min_column_offset..(max_column_offset + 1) {
            let played = played_tiles.iter().find(|t| t.row_offset == row_offset_i && t.column_offset == column_offset_i);
            if played.is_some() {
                let tile = tiles.filter(::schema::tiles::dsl::id.eq(played.unwrap().tile_id))
                                .get_result::<Tile>(&connection).expect("Error loading tiles");

                let mut blah_tile = get_tile(tile, &connection);
                blah_tile.playerId = 1;//played.unwrap().player_id.unwrap();
                blah_tile.rowOffset = played.unwrap().row_offset;
                blah_tile.columnOffset = played.unwrap().column_offset;
                row.push(blah_tile);
            } else {
                row.push(JsTile { id: 0, playerId: 0, cities: Vec::new(), roads: Vec::new(), columnOffset: column_offset_i, rowOffset: row_offset_i });
            }
        }

        grid.push(row);
    }

    grid
}

fn get_tile(tile: Tile, connection: &PgConnection) -> JsTile {
    use schema::tile_roads::dsl::*;
    use schema::tile_cities::dsl::*;

    let cities = tile_cities.filter(::schema::tile_cities::dsl::tile_id.eq(tile.id))
                    .load::<TileCity>(connection).expect("Error loading tile cities");

    let roads = tile_roads.filter(::schema::tile_roads::dsl::tile_id.eq(tile.id))
                    .load::<TileRoad>(connection).expect("Error loading tile cities");

    let mut output = JsTile { id: tile.id, playerId: 0, cities: Vec::new(), roads: Vec::new(), columnOffset: 0, rowOffset: 0 };
    for city in cities {
        output.cities.push(city.city_side);
    }

    for road in roads {
        output.roads.push(road.road_side);
    }

    output
}
