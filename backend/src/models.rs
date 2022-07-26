use std::fmt;
use crate::schema::{tickets, zones, balances, super_wallets, proposals, env_vars, vouchers};
use json::{JsonValue, object};
use serde::{Serialize, Deserialize};

#[derive(Identifiable, Queryable, Insertable, Deserialize, Serialize, Debug, Clone)]
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

#[derive(Identifiable, Associations, Queryable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(belongs_to(Zone))]
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

#[derive(Identifiable, Associations, Queryable, Insertable, Deserialize, Serialize, Debug)]
#[diesel(belongs_to(Zone))]
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

#[derive(Identifiable, Queryable, Insertable, Deserialize, Serialize, Debug)]
pub struct SuperWallet {
    pub id: i32,
    pub address: String,
}

#[derive(Identifiable, Queryable, Insertable, Deserialize, Serialize, Debug)]
pub struct Proposal {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub proposal_type: i32,
    pub created_at: String,
}

#[derive(Identifiable, Queryable, Insertable, Deserialize, Serialize, Debug)]
pub struct EnvVar {
    pub id: i32,
    pub var_name: String,
    pub var_value: String,
}

impl Into<JsonValue> for EnvVar {
    fn into(self) -> JsonValue {
        self.var_value.as_str().into()
    }
}

impl fmt::Display for EnvVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.var_value.as_str())
    }
}

#[derive(Identifiable, Queryable, Insertable, Deserialize, Serialize, Debug)]
pub struct Voucher {
    pub id: i32,
    pub epoch_index: i32,
    pub input_index: i32,
    pub voucher_index: Option<i32>,
    pub requested_by: String
}

impl From<Voucher> for JsonValue {
    fn from(item: Voucher) -> Self {
        object! { 
            id: item.id,
            epoch_index: item.epoch_index,
            input_index: item.input_index,
            voucher_index: item.voucher_index,
            owner_address: item.requested_by
        }
    }
}