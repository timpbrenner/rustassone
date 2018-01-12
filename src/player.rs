use diesel::prelude::*;
use diesel::insert_into;

use models::*;
use lib::*;

pub fn join_game(current_game_id: i32, sign_in_name: String) -> Player {
    use schema::players::dsl::*;
    use schema::game_players::dsl::*;

    let connection = establish_connection();
    let mut player_result = players.filter(username.eq(&sign_in_name)).get_result::<Player>(&connection).ok();
    let player: Player;

    if player_result.is_none() {
        insert_into(players).values((username.eq(&sign_in_name))).execute(&connection);
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

    player
}
