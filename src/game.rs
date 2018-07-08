use diesel::prelude::*;
use diesel::sql_query;
use diesel::insert_into;
use diesel::update;

use models::*;
use js_models::*;
use lib::*;
use std;

#[derive(Deserialize)]
pub struct TilePlay {
    pub tile_id: Option<i32>,
    pub player_id: Option<i32>,
    pub row_offset: Option<i32>,
    pub column_offset: Option<i32>
}

#[derive(Deserialize)]
pub struct MeeplePlay {
    pub tile_id: Option<i32>,
    pub player_id: Option<i32>,
    pub side: Option<i32>,
}

pub fn create_game() -> JsGame {
    use schema::games::dsl::*;

    let conn = establish_connection();
    let game_result = insert_into(games)
                .values(current_state.eq("not_started"))
                .get_result::<Game>(&conn);

    let game = game_result.ok().unwrap();
    play_tile(game.id, TilePlay { tile_id: Some(4), player_id: None, row_offset: Some(0), column_offset: Some(0) });

    get_game(game.id)
}

pub fn start_game(game_id: i32) -> JsGame {
    use schema::games::dsl::*;

    let connection = establish_connection();
    let target = games.filter(::schema::games::dsl::id.eq(game_id));
    let _result = update(target).set(current_state.eq("draw")).execute(&connection);

    get_game(game_id)
}

pub fn get_game(game_id: i32) -> JsGame {
    use schema::games::dsl::*;

    let connection = establish_connection();
    let game = games.filter(::schema::games::dsl::id.eq(game_id)).get_result::<Game>(&connection).expect("Error loading games");

    JsGame {
        id: game.id,
        grid: build_game_grid(game.id, &connection),
        players: get_game_players(game.id, &connection),
        currentPlayerId: game.current_player_id.unwrap_or(-1),
        currentState: game.current_state.unwrap_or("draw".to_owned())
    }
}

pub fn draw_tile(current_game_id: i32) -> JsTile {
    let connection = establish_connection();
    let sql = format!("SELECT t.* FROM tiles t \
                    LEFT JOIN game_tiles gt ON gt.tile_id = t.id and gt.game_id = {} \
                    WHERE gt.id IS NULL \
                    ORDER BY random() LIMIT 1", current_game_id);
    let tile_result: std::result::Result<Tile, ::diesel::result::Error> = sql_query(sql).get_result(&connection);

    get_tile(tile_result.ok().unwrap(), &connection)
}

pub fn play_tile(play_game_id: i32, play: TilePlay) -> JsTile {
    use schema::game_tiles::dsl::*;

    let conn = establish_connection();
    insert_into(game_tiles)
        .values(
            (
                game_id.eq(play_game_id),
                tile_id.eq(play.tile_id.unwrap()),
                player_id.eq(play.player_id),
                row_offset.eq(play.row_offset.unwrap()),
                column_offset.eq(play.column_offset.unwrap()),
            )
        ).execute(&conn);

    JsTile { id: 1, playerId: 0, cities: Vec::new(), roads: Vec::new(), columnOffset: 0, rowOffset: 0, meeple: None }
}

pub fn play_meeple(play_game_id: i32, play: MeeplePlay) -> String  {
    use schema::game_pieces::dsl::*;

    let connection = establish_connection();
    insert_into(game_pieces)
        .values(
            (
                tile_id.eq(play.tile_id.unwrap()),
                game_id.eq(play_game_id),
                player_id.eq(play.player_id.unwrap()),
                side.eq(play.side.unwrap()),
            )
        ).execute(&connection);

    String::from("Return Something")
}

// Private

fn build_game_grid(current_game_id: i32, connection: &PgConnection) -> Vec<Vec<JsTile>> {
    use schema::game_tiles::dsl::*;
    use schema::tiles::dsl::*;

    let played_tiles = game_tiles.filter(game_id.eq(current_game_id)).load::<GameTile>(connection).expect("Error loading game tiles");

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
                                .get_result::<Tile>(connection).expect("Error loading tiles");

                let mut blah_tile = get_tile(tile, connection);
                blah_tile.playerId = 1;//played.unwrap().player_id.unwrap();
                blah_tile.rowOffset = played.unwrap().row_offset;
                blah_tile.columnOffset = played.unwrap().column_offset;
                row.push(blah_tile);
            } else {
                row.push(JsTile { id: 0, playerId: 0, cities: Vec::new(), roads: Vec::new(), columnOffset: column_offset_i, rowOffset: row_offset_i, meeple: None });
            }
        }

        grid.push(row);
    }

    grid
}

fn get_tile(tile: Tile, connection: &PgConnection) -> JsTile {
    use schema::tile_roads::dsl::*;
    use schema::tile_cities::dsl::*;
    use schema::game_pieces::dsl::*;

    let cities = tile_cities.filter(::schema::tile_cities::dsl::tile_id.eq(tile.id))
                    .load::<TileCity>(connection).expect("Error loading tile cities");

    let roads = tile_roads.filter(::schema::tile_roads::dsl::tile_id.eq(tile.id))
                    .load::<TileRoad>(connection).expect("Error loading tile cities");

    let meeple = game_pieces.filter(::schema::game_pieces::dsl::tile_id.eq(tile.id))
                    .get_result::<GamePiece>(connection)
                    .optional()
                    .expect("Error loading game pieces");

    let mut js_meeple = JsMeeple { playerId: 0, tileId: 0, side: 0 };
    if meeple.is_some() {
        let unwrapped_meeple = meeple.unwrap();

        js_meeple.playerId = unwrapped_meeple.player_id;
        js_meeple.tileId = unwrapped_meeple.tile_id;
        js_meeple.side = unwrapped_meeple.side;
    }

    let mut output = JsTile { id: tile.id, playerId: 0, cities: Vec::new(), roads: Vec::new(), columnOffset: 0, rowOffset: 0, meeple: Some(js_meeple) };
    for city in cities {
        output.cities.push(city.city_side);
    }

    for road in roads {
        output.roads.push(road.road_side);
    }

    output
}

fn get_game_players(play_game_id: i32, connection: &PgConnection) -> Vec<JsPlayer> {
    use schema::game_players::dsl::*;
    use schema::players::dsl::*;

    let mut ret_players = Vec::new();
    let g_players = game_players.filter(::schema::game_players::dsl::game_id.eq(play_game_id)).load::<GamePlayer>(connection).expect("Error loading game players");
    for g_p in g_players.iter() {
        let player = players.filter(::schema::players::dsl::id.eq(g_p.player_id))
            .get_result::<Player>(connection)
            .expect("Error loading game pieces");

        ret_players.push(JsPlayer { id: player.id, username: player.username.unwrap() });
    }

    ret_players
}
