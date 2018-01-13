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

// ROOT
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("games.html")).ok()
}

// ASSETS
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

// GAME
#[get("/")]
fn create() -> Json<JsGame> {
    Json(create_game())
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
fn join(id: String, user: UserData) -> Json<JsPlayer> {
    let game_id = id.parse().unwrap();

    println!("USERNAME: {}", user.username);

    Json(join_game(game_id, user.username))
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
    let int_game_id:i32 = current_game_id.parse().unwrap();

    Json(play_tile(int_game_id, play))
}

#[get("/<id>/start")]
fn start(id: String) -> Json<JsGame> {
    let int_game_id:i32 = id.parse().unwrap();

    Json(start_game(int_game_id))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/game", routes![create, game, current, start, join, draw, play])
        .mount("assets/", routes![files])
        .launch();
}
