// @generated automatically by Diesel CLI.

diesel::table! {
    zones (id) {
        id -> Integer,
        name -> Text,
        price -> Float,
        geo_json -> Text,
        owner_address -> Text,
    }
}

table! {
    tickets (id) {
        id -> Integer,
        name -> Text,
        price -> Float,
        geo_json -> Text,
        owner_address -> Text,
    }
}
