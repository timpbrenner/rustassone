#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rand;
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
use rand::Rng;
use rocket_contrib::Json;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

pub use models::*;
pub use lib::*;

#[derive(Serialize)]
struct BlahTile {
    cities: Vec<i32>,
    roads: Vec<i32>,
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/<id>/draw")]
fn draw(id: String) -> Json<BlahTile> {
    use schema::tiles::dsl::*;
    use schema::tile_roads::dsl::*;
    use schema::tile_cities::dsl::*;

    let connection = establish_connection();
    let results = tiles.load::<Tile>(&connection).expect("Error loading tile");

    // HANDLE NO TILES MEANS GAME OVER
    let tile = rand::thread_rng().choose(&results).unwrap();
    let cities = tile_cities.filter(schema::tile_cities::dsl::tile_id.eq(tile.id))
                    .load::<TileCity>(&connection).expect("Error loading tile cities");

    let roads = tile_roads.filter(schema::tile_roads::dsl::tile_id.eq(tile.id))
                    .load::<TileRoad>(&connection).expect("Error loading tile cities");

    let mut output = BlahTile { cities: Vec::new(), roads: Vec::new() };
    for city in cities {
        output.cities.push(city.city_side);
    }

    for road in roads {
        output.roads.push(road.road_side);
    }

    Json(output)
}

#[get("/<id>/play")]
fn play(id: String) -> Json<BlahTile> {
    Json(BlahTile { cities: Vec::new(), roads: Vec::new() })
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
