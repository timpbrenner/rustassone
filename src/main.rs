#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

mod schema;
mod models;
mod lib;

use std::path::{Path, PathBuf};

use rocket_contrib::Json;
use rocket::response::NamedFile;
use diesel::prelude::*;

pub use models::*;
pub use lib::*;

#[derive(Serialize)]
struct BlahTile {
    cities: Vec<u8>,
    roads: Vec<u8>,
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/game/<id>/draw", rank = 3)]
fn draw(id: String) -> Json<BlahTile> {
    use schema::tiles::dsl::*;
    use schema::tile_roads::dsl::*;
    use schema::tile_cities::dsl::*;

    let connection = establish_connection();
    let tile = tiles.order(schema::tiles::dsl::id.desc())
                    .first::<Tile>(&connection)
                    .expect("Error loading tile");
    let cities = tile_cities.filter(schema::tile_cities::dsl::tile_id.eq(tile.id))
                    .load::<TileCity>(&connection).expect("Error loading tile cities");

    let roads = tile_roads.filter(schema::tile_roads::dsl::tile_id.eq(tile.id))
                    .load::<TileRoad>(&connection).expect("Error loading tile cities");

    println!("Tile id: {}", tile.id);
    println!("Cities: {}", cities.len());
    println!("Roads: {}", roads.len());

    //for post in results {
    //    println!("{}", post.title);
    //    println!("----------\n");
    //    println!("{}", post.body);
    //}

    Json(BlahTile { cities: vec![0], roads: vec![1,3] })
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![draw, index]).mount("assets/", routes![files]).launch();
}
