#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};

use rocket_contrib::Json;
use rocket::response::NamedFile;

#[derive(Serialize)]
struct Tile {
    cities: Vec<u8>,
    roads: Vec<u8>,
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[get("/game/<id>/draw", rank = 3)]
fn draw(id: String) -> Json<Tile> {
    Json(Tile { cities: vec![0], roads: vec![1,3] })
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![draw, index]).mount("assets/", routes![files]).launch();
}
