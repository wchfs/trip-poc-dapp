use serde::{Deserialize, Serialize};
use json::JsonValue;

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

#[derive(Deserialize, Debug, Default)]
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

#[derive(Deserialize, Debug)]
pub enum RoutePayload {
    Ticket(TicketActions),
    Point(GeoPoint),
    Balance(BalanceActions),
}

#[derive(Deserialize, Debug)]
pub enum TicketActions {
    Buy(BuyTicket),
    Get(GetTicket),
    Validate(ValidateTicket),
}

#[derive(Deserialize, Debug)]
pub enum BalanceActions {
    Withdraw(WithdrawFunds),
}

#[derive(Deserialize, Debug)]
pub struct GeoPoint {
    pub longitude: f64,
    pub latitude: f64,
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
    pub date: String,
}

#[derive(Deserialize, Debug)]
pub struct WithdrawFunds {
    pub amount: String,
}

//What are those numbers? Let's explain with code: keccak_hash::keccak("etherWithdrawal(bytes)".as_bytes()).as_bytes().get(0..4).unwrap();
pub const ETHER_WITHDRAWAL_HEADER: [u8; 4] = [116, 149, 107, 148];

#[derive(Serialize, Debug)]
pub struct VoucherResponse {
    pub address: String,
    pub payload: String,
}
