use crate::schema::tickets;
use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Zone {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub start_hour: String,
    pub end_hour: String,
    pub geo_json: String,
    pub owner_address: String,
}

#[derive(Insertable, Queryable, Serialize)]
pub struct Ticket {
    pub id: i32,
    pub license: String,
    pub longitude: f32,
    pub latitude: f32,
    pub owner_address: String,
    pub purchased_at: String,
    pub duration: i32,
    pub zone_id: i32,
    pub status: i32,
}
