#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

extern crate serde_json;
extern crate dotenv;
extern crate actix_web;

mod schema;
mod models;
mod lib;
mod game;
mod player;

use std::path::{Path, PathBuf};
use actix_web::{
    middleware::cors::Cors,
    server,
    App,
    HttpRequest,
    Responder,
    Result,
    http::Method,
    fs::NamedFile,
    Json
};

pub use models::*;
pub use lib::*;
pub use game::*;
pub use player::*;

// // API CALL: Start Game
// #[get("/<id>/start")]
// fn start(id: String) -> Json<JsGame> {
//     let int_game_id:i32 = id.parse().unwrap();
//
//     Json(start_game(int_game_id))
// }
//
// // API CALL: Draw a tile
// #[get("/<current_game_id>/draw")]
// fn draw(current_game_id: String) -> Json<JsTile> {
//     Json(draw_tile(current_game_id))
// }
//
// // API CALL: Play a tile
// #[get("/<current_game_id>/play?<play>")]
// fn play(current_game_id: String, play: TilePlay) -> Json<JsTile> {
//     let int_game_id:i32 = current_game_id.parse().unwrap();
//
//     Json(play_tile(int_game_id, play))
// }

// ROOT
// Path for game lobbies
fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open(Path::new("static/").join("games.html"))?)
}

// GAME
// Page URL: specific game
fn game(req: HttpRequest) -> Result<NamedFile> {
    let current_game_id = req.match_info().get("game_id").unwrap();
    println!("SHOWING GAME: {}", current_game_id);
    Ok(NamedFile::open(Path::new("static/").join("index.html"))?)
}

// ASSETS
// Serves js and css assets
fn resources(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("tail")?;
    println!("{:?}", path);

    Ok(NamedFile::open(Path::new("static/").join(path))?)
}

// API CALL: Creates a game and return JSON of new game
fn create(_req: HttpRequest) -> impl Responder {
    println!("Creating Game");
    create_game()
}

// API CALL: Current state of a specific game
fn show(req: HttpRequest) -> impl Responder {
    let game_id = req.match_info().get("game_id").unwrap();
    println!("SHOW GAME: {}", game_id);

    let int_game_id:i32 = game_id.parse().unwrap();

    get_game(int_game_id)
}

#[derive(Deserialize)]
struct Info {
    username: String
}

// API CALL: Player joining the game
fn create_player(data: (actix_web::Path<i32>, Json<Info>)) -> impl Responder {
    let (params, info) = data;
    // println!("CREATE PLAYER: {}", params.0);
    println!("USERNAME: {}", info.username);

    "SOMETHING"
    // join_game(game_id, user.username)
}

fn main() {
    server::new(|| App::new()
        .configure(|app| {
            Cors::for_app(app)
                .send_wildcard()
                .resource("/game", |r| r.method(Method::GET).f(create))
                .resource("/game/{game_id}", |r| r.method(Method::GET).f(show))
                .resource("/game/{game_id}/players", |r| r.method(Method::POST).with(create_player))
                .register()
        })
        .resource(r"/a/{tail:.*}", |r| r.method(Method::GET).f(resources))
        .resource("/", |r| r.method(Method::GET).f(index))
        .resource("/play/{game_id}", |r| r.method(Method::GET).f(game))
    )
    .bind("127.0.0.1:8088")
    .unwrap()
    .run();
}
