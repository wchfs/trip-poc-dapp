use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use json::{JsonValue};

pub const WEI_TO_GWEI_FACTOR: u128 = 1000000000;

#[derive(Debug)]
pub enum TicketStatus {
    Paid = 0,
    Completed = 1,
    Refunded = 2,
    PaidOut = 3,
}



#[derive(Debug)]
pub enum ResponseType {
    Notice,
    Report,
    Voucher,
}

impl ResponseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseType::Notice => "notice",
            ResponseType::Report => "report",
            ResponseType::Voucher => "voucher",
        }
    }
}

impl Into<JsonValue> for ResponseType {
    fn into(self) -> JsonValue {
        self.as_str().into()
    }
}

#[derive(Debug)]
pub enum ResponseStatus {
    Accept,
    Reject,
}

impl ResponseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseStatus::Accept => "accept",
            ResponseStatus::Reject => "reject",
        }
    }
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Route {
    pub endpoint: String,
    pub payload: Option<RoutePayload>,
}

#[derive(Debug)]
pub struct StandardInput {
    //bytes32: Option<ethabi::Token>,
    pub address: Option<String>,
    pub uint256: Option<ethabi::Token>,
    pub bytes: Vec<u8>,
    pub request: JsonValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorOutput {
    pub error: String,
}

impl fmt::Display for ErrorOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for ErrorOutput {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessOutput {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum RoutePayload {
    Ticket(TicketActions),
    Point(GeoPoint),
    Balance(BalanceActions),
    Seed(SeederActions),
    Remove(Remover),
}

#[derive(Deserialize, Debug, Clone)]
pub enum TicketActions {
    Buy(BuyTicket),
    Get(GetTicket),
    Validate(ValidateTicket),
}

#[derive(Deserialize, Debug, Clone)]
pub enum BalanceActions {
    Withdraw(WithdrawFunds),
    Get(GetBalance),
}

#[derive(Deserialize, Debug, Clone)]
pub enum SeederActions {
    Zone(ZoneSeeder),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Remover {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GeoPoint {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuyTicket {
    pub license: String,
    pub longitude: f32,
    pub latitude: f32,
    pub started_at: String,
    pub duration: i32,
    pub zone_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetTicket {
    pub license: Option<String>,
    pub owner_address: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ValidateTicket {
    pub license: String,
    pub date: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WithdrawFunds {
    pub amount: String,
    pub zone_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetBalance {
    pub zone_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ZoneSeeder {
    pub name: String,
    pub price: String,
    pub geo_json: String,
    pub zone_owner_address: String,
}

//What are those numbers? Let's explain with code: keccak_hash::keccak("etherWithdrawal(bytes)".as_bytes()).as_bytes().get(0..4).unwrap();
pub const ETHER_WITHDRAWAL_HEADER: [u8; 4] = [116, 149, 107, 148];

#[derive(Serialize, Debug)]
pub struct VoucherResponse {
    pub address: String,
    pub payload: String,
}
