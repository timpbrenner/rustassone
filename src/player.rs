use diesel::prelude::*;
use diesel::insert_into;

use models::*;
use lib::*;
use serde_json;
use actix_web::{Responder, HttpRequest, HttpResponse, Error};

#[derive(Serialize)]
pub struct JsPlayer {
    pub id: i32,
}

/// Responder
impl Responder for JsPlayer {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

pub fn join_game(current_game_id: i32, sign_in_name: String) -> JsPlayer {
    use schema::players::dsl::*;
    use schema::game_players::dsl::*;

    let connection = establish_connection();
    let mut player_result = players.filter(username.eq(&sign_in_name)).get_result::<Player>(&connection).ok();
    let player: Player;

    if player_result.is_none() {
        insert_into(players).values(username.eq(&sign_in_name)).execute(&connection);
        player_result = players.filter(username.eq(&sign_in_name)).get_result::<Player>(&connection).ok();
    }
    player = player_result.unwrap();

    insert_into(game_players)
        .values(
            (
                game_id.eq(current_game_id),
                player_id.eq(player.id),
            )
        ).execute(&connection);

    JsPlayer { id: player.id }
}
