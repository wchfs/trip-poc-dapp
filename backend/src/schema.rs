table! {
    tickets (id) {
        id -> Integer,
        license -> Text,
        longitude -> Float,
        latitude -> Float,
        started_at -> Text,
        owner_address -> Text,
        purchased_at -> Text,
        duration -> Integer,
        zone_id -> Integer,
        status -> Integer,
    }
}

table! {
    zones (id) {
        id -> Integer,
        name -> Text,
        price -> Float,
        start_hour -> Text,
        end_hour -> Text,
        geo_json -> Text,
        owner_address -> Text,
    }
}

joinable!(tickets -> zones (zone_id));

allow_tables_to_appear_in_same_query!(
    tickets,
    zones,
);
