table! {
    balances (id) {
        id -> Integer,
        zone_id -> Integer,
        amount -> Text,
    }
}

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
        paid -> Text,
        to_pay -> Text,
        status -> Integer,
    }
}

table! {
    zones (id) {
        id -> Integer,
        name -> Text,
        price -> Text,
        geo_json -> Text,
        owner_address -> Text,
    }
}

joinable!(balances -> zones (zone_id));
joinable!(tickets -> zones (zone_id));

allow_tables_to_appear_in_same_query!(
    balances,
    tickets,
    zones,
);
