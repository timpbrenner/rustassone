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
mod game;
mod player;

use rocket_contrib::Json;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

pub use models::*;
pub use lib::*;
pub use game::*;
pub use player::*;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("login.html")).ok()
}

#[get("/<current_game_id>")]
fn game(current_game_id: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("index.html")).ok()
}

#[derive(FromForm)]
pub struct UserData {
    pub username: String
}

#[get("/<id>/join?<user>")]
fn join(id: String, user: UserData) -> Json<JsGame> {
    let game_id = id.parse().unwrap();

    join_game(game_id, user.username);
    Json(get_game(game_id))
}

#[get("/<id>/current")]
fn current(id: String) -> Json<JsGame> {
    let game_id = id.parse().unwrap();

    Json(get_game(game_id))
}

#[get("/<current_game_id>/draw")]
fn draw(current_game_id: String) -> Json<JsTile> {
    Json(draw_tile(current_game_id))
}

#[get("/<current_game_id>/play?<play>")]
fn play(current_game_id: String, play: TilePlay) -> Json<JsTile> {
    Json(play_tile(current_game_id, play))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/game", routes![game, current, join, draw, play])
        .mount("assets/", routes![files])
        .launch();
}
