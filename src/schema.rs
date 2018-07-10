table! {
    game_pieces (id) {
        id -> Int4,
        tile_id -> Int4,
        game_id -> Int4,
        player_id -> Int4,
        side -> Int4,
    }
}

table! {
    game_players (id) {
        id -> Int4,
        game_id -> Int4,
        player_id -> Int4,
        turn_order -> Int4,
        score -> Int4,
    }
}

table! {
    game_tiles (id) {
        id -> Int4,
        game_id -> Int4,
        tile_id -> Int4,
        player_id -> Nullable<Int4>,
        row_offset -> Int4,
        column_offset -> Int4,
    }
}

table! {
    games (id) {
        id -> Int4,
        current_player_id -> Nullable<Int4>,
        current_state -> Nullable<Text>,
    }
}

table! {
    players (id) {
        id -> Int4,
        username -> Nullable<Text>,
    }
}

table! {
    tile_cities (id) {
        id -> Int4,
        tile_id -> Int4,
        city_side -> Int4,
        tile_group -> Int4,
    }
}

table! {
    tile_roads (id) {
        id -> Int4,
        tile_id -> Int4,
        road_side -> Int4,
        tile_group -> Int4,
    }
}

table! {
    tiles (id) {
        id -> Int4,
    }
}

joinable!(game_pieces -> games (game_id));
joinable!(game_pieces -> players (player_id));
joinable!(game_pieces -> tiles (tile_id));
joinable!(game_players -> games (game_id));
joinable!(game_players -> players (player_id));
joinable!(game_tiles -> games (game_id));
joinable!(game_tiles -> players (player_id));
joinable!(game_tiles -> tiles (tile_id));
joinable!(tile_cities -> tiles (tile_id));
joinable!(tile_roads -> tiles (tile_id));

allow_tables_to_appear_in_same_query!(
    game_pieces,
    game_players,
    game_tiles,
    games,
    players,
    tile_cities,
    tile_roads,
    tiles,
);
