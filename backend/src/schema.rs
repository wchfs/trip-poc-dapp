// @generated automatically by Diesel CLI.

diesel::table! {
    tickets (id) {
        id -> Integer,
        license -> Text,
        longitude -> Float,
        latitude -> Float,
        owner_address -> Text,
        purchased_at -> Text,
        duration -> Integer,
        zone_id -> Integer,
        status -> Integer,
    }
}

diesel::table! {
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

diesel::allow_tables_to_appear_in_same_query!(
    tickets,
    zones,
);
