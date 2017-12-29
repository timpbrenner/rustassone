table! {
    tile_cities (id) {
        id -> Int4,
        tile_id -> Nullable<Int4>,
        city_side -> Int4,
        tile_group -> Int4,
    }
}

table! {
    tile_roads (id) {
        id -> Int4,
        tile_id -> Nullable<Int4>,
        road_side -> Int4,
        tile_group -> Int4,
    }
}

table! {
    tiles (id) {
        id -> Int4,
        status -> Nullable<Varchar>,
    }
}

joinable!(tile_cities -> tiles (tile_id));
joinable!(tile_roads -> tiles (tile_id));

allow_tables_to_appear_in_same_query!(
    tile_cities,
    tile_roads,
    tiles,
);
