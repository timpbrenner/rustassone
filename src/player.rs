use diesel::prelude::*;
use diesel::insert_into;
use diesel::dsl::count;

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
        let _result = insert_into(players).values(username.eq(&sign_in_name)).execute(&connection);
        player_result = players.filter(username.eq(&sign_in_name)).get_result::<Player>(&connection).ok();
    }
    player = player_result.unwrap();

    let game_player_result = game_players
        .filter(game_id.eq(current_game_id))
        .filter(player_id.eq(player.id))
        .get_result::<GamePlayer>(&connection)
        .ok();

    if game_player_result.is_none() {
        let order = game_players
            .filter(game_id.eq(current_game_id))
            .filter(player_id.eq(player.id))
            .select(count(game_id))
            .first(&connection);

        let _result = insert_into(game_players)
            .values(
                (
                    game_id.eq(current_game_id),
                    player_id.eq(player.id),
                    turn_order.eq((order.unwrap_or(0) + 1) as i32),
                )
            ).execute(&connection);
    }

    JsPlayer { id: player.id }
}
