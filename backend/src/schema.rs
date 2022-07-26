table! {
    balances (id) {
        id -> Integer,
        zone_id -> Integer,
        amount -> Text,
    }
}

table! {
    env_vars (id) {
        id -> Integer,
        var_name -> Text,
        var_value -> Text,
    }
}

table! {
    proposal_votes (id) {
        id -> Integer,
        vote -> Integer,
        proposal_id -> Integer,
    }
}

table! {
    proposals (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        status -> Integer,
        proposal_type -> Integer,
        created_at -> Text,
    }
}

table! {
    super_wallets (id) {
        id -> Integer,
        address -> Text,
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
    vouchers (id) {
        id -> Integer,
        epoch_index -> Integer,
        input_index -> Integer,
        voucher_index -> Nullable<Integer>,
        requested_by -> Text,
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
joinable!(proposal_votes -> proposals (proposal_id));
joinable!(tickets -> zones (zone_id));

allow_tables_to_appear_in_same_query!(
    balances,
    env_vars,
    proposal_votes,
    proposals,
    super_wallets,
    tickets,
    vouchers,
    zones,
);
