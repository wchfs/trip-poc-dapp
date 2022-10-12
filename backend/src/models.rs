use crate::schema::{tickets, zones, balances};
use serde::{Serialize};

#[derive(Identifiable, Queryable, Serialize, Debug)]
pub struct Zone {
    pub id: i32,
    pub name: String,
    pub price: String,
    pub geo_json: String,
    pub owner_address: String,
}

#[derive(Identifiable, Associations, Queryable, Insertable, Serialize, Debug)]
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
    pub paid: String,
    pub to_pay: String,
    pub status: i32,
}

#[derive(Identifiable, Queryable, Serialize, Debug)]
pub struct Balance {
    pub id: i32,
    pub amount: String,
}
