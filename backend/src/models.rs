use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Zone {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub geo_json: String,
    pub owner_address: String,
}
