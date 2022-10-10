use serde::{Deserialize, Serialize};

pub const WEI_TO_GWEI_FACTOR: u128 = 1000000000;

pub enum TicketStatus {
    Paid = 0,
    Completed = 1,
    Refunded = 2,
    PaidOut = 3,
}

#[derive(Deserialize, Debug, Default)]
pub struct Route {
    pub endpoint: String,
    pub payload: Option<RoutePayload>
}

#[derive(Deserialize, Debug)]
pub struct StandardInput {
    //bytes32: Option<ethabi::Token>,
    pub address: Option<ethabi::Token>,
    pub uint256: Option<ethabi::Token>,
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Deserialize, Debug)]
pub enum RoutePayload {
    Ticket(TicketActions),
    Point(GeoPoint)
}

#[derive(Deserialize, Debug)]
pub enum TicketActions {
    Buy(BuyTicket),
    Get(GetTicket),
    Validate(ValidateTicket)
}

#[derive(Deserialize, Debug)]
pub struct GeoPoint {
    pub longitude: f64,
    pub latitude: f64
}

#[derive(Deserialize, Debug)]
pub struct BuyTicket {
    pub license: String,
    pub longitude: f32,
    pub latitude: f32,
    pub started_at: String,
    pub duration: i32,
    pub zone_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct GetTicket {
    pub license: Option<String>,
    pub owner_address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ValidateTicket {
    pub license: String,
}