use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Zone {
    pub id: i32,
    pub price_per_minute: f32,
    pub geo_json: String,
    pub owner_address: String,
}
