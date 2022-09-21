use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Zone {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub geo_json: String,
    pub owner_address: String,
}

#[derive(Queryable)]
pub struct Ticket {
    pub id: i32,
    pub owner_address: String,
    pub purchased_at: String,
    pub zone_id: i32,
    pub status: i8,
}
