#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

extern crate serde_json;
extern crate dotenv;
extern crate actix_web;
extern crate env_logger;

mod schema;
mod models;
mod lib;
mod game;
mod player;

use std::path::{Path, PathBuf};
use actix_web::{
    middleware::cors::Cors,
    middleware::Logger,
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

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[derive(Deserialize)]
struct GamePath {
    game_id: i32,
}

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

    Ok(NamedFile::open(Path::new("dist/").join(path))?)
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

// API CALL: Update Game(All it updates is game to start)
fn update(path: actix_web::Path<GamePath>) -> impl Responder {
    println!("UPDATE GAME: {}", path.game_id);
    start_game(path.game_id)
}

// API CALL: Player joining the game
fn create_player(data: (actix_web::Path<GamePath>, Json<Info>)) -> impl Responder {
    let (path, info) = data;
    println!("CREATE PLAYER: {}, {}", path.game_id, info.username);

    join_game(path.game_id, info.username.to_string())
}

 // API CALL: Draw a tile
fn draw(path: actix_web::Path<GamePath>) -> impl Responder {
     draw_tile(path.game_id)
 }

// API CALL: Play a tile
fn play(data: (actix_web::Path<GamePath>, Json<TilePlay>)) -> impl Responder {
    println!("PLAY TILE");

    let (path, play) = data;
    println!("PLAY TILE: {}, {}", path.game_id, play.tile_id.unwrap());

    play_tile(path.game_id, play.into_inner())
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new(|| App::new()
        .middleware(Logger::default())
        .configure(|app| {
            Cors::for_app(app)
                .send_wildcard()
                .resource("/game", |r| r.method(Method::GET).f(create))
                .resource("/game/{game_id}", |r| r.method(Method::GET).f(show))
                .resource("/game/{game_id}/start", |r| r.method(Method::GET).with(update))
                .resource("/game/{game_id}/tiles", |r| r.method(Method::GET).with(draw))
                .resource("/game/{game_id}/play", |r| r.method(Method::POST).with(play))
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
