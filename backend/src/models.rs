use crate::schema::{tickets, zones};
use serde::{Serialize};

#[derive(Identifiable, Queryable, Serialize)]
pub struct Zone {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub start_hour: String,
    pub end_hour: String,
    pub geo_json: String,
    pub owner_address: String,
}

#[derive(Identifiable, Associations, Queryable, Insertable, Serialize)]
#[belongs_to(Zone)]
pub struct Ticket {
    pub id: i32,
    pub license: String,
    pub longitude: f32,
    pub latitude: f32,
    pub started_at: String,
    pub owner_address: String,
    pub purchased_at: String,
    pub duration: i32,
    pub zone_id: i32,
    pub status: i32,
}
