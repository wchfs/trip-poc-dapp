use crate::schema::{tickets, zones, balances};
use json::{JsonValue, object};
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Queryable, Deserialize, Serialize, Debug)]
pub struct Zone {
    pub id: i32,
    pub name: String,
    pub price: String,
    pub geo_json: String,
    pub owner_address: String,
}

impl From<Zone> for JsonValue {
    fn from(item: Zone) -> Self {
        object! { 
            id: item.id,
            name: item.name,
            price: item.price,
            geo_json: item.geo_json,
            owner_address: item.owner_address,   
        }
    }
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

impl From<Ticket> for JsonValue {
    fn from(item: Ticket) -> Self {
        object! { 
            id: item.id,
            license: item.license,
            longitude: item.longitude,
            latitude: item.latitude,
            started_at: item.started_at,
            owner_address: item.owner_address,
            purchased_at: item.purchased_at,
            duration: item.duration,
            zone_id: item.zone_id,
            paid: item.paid,
            to_pay: item.to_pay,
            status: item.status,
        }
    }
}

#[derive(Identifiable, Associations, Queryable, Insertable, Serialize, Debug)]
#[belongs_to(Zone)]
pub struct Balance {
    pub id: i32,
    pub zone_id: i32,
    pub amount: String,
}

impl From<Balance> for JsonValue {
    fn from(item: Balance) -> Self {
        object! { 
            id: item.id,
            zone_id: item.zone_id,
            amount: item.amount,
        }
    }
}
