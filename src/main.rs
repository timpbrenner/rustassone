#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;

mod schema;
mod models;
mod lib;

use diesel::prelude::*;
use diesel::insert_into;
use diesel::sql_query;
use rocket_contrib::Json;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

pub use models::*;
pub use lib::*;

#[derive(Serialize)]
struct BlahTile {
    id: i32,
    cities: Vec<i32>,
    roads: Vec<i32>,
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/<current_game_id>/draw")]
fn draw(current_game_id: String) -> Json<BlahTile> {
    use schema::tiles::dsl::*;
    use schema::tile_roads::dsl::*;
    use schema::tile_cities::dsl::*;
    use schema::game_tiles::dsl::*;

    let connection = establish_connection();
    let sql = format!("SELECT t.* FROM tiles t \
                    LEFT JOIN game_tiles gt ON gt.tile_id = t.id and gt.game_id = {} \
                    WHERE gt.id IS NULL \
                    ORDER BY random() LIMIT 1", current_game_id);
    let tile_result: std::result::Result<Tile, diesel::result::Error> = sql_query(sql).get_result(&connection);
    //println!("OK: {}", tile_result.is_ok());

    // HANDLE NO TILES MEANS GAME OVER
    let tile = tile_result.ok().unwrap();
    let cities = tile_cities.filter(schema::tile_cities::dsl::tile_id.eq(tile.id))
                    .load::<TileCity>(&connection).expect("Error loading tile cities");

    let roads = tile_roads.filter(schema::tile_roads::dsl::tile_id.eq(tile.id))
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

#[derive(FromForm)]
struct TilePlay {
    tile_id: Option<i32>,
    player_id: Option<i32>,
    row_offset: Option<i32>,
    column_offset: Option<i32>
}

#[get("/<current_game_id>/play?<play>")]
fn play(current_game_id: String, play: TilePlay) -> Json<BlahTile> {
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

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/game", routes![draw, play])
        .mount("assets/", routes![files])
        .launch();
}
