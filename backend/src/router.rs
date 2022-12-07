use json::JsonValue;
use crate::balance_module;
use crate::structures::*;
use crate::ticket_module;
use crate::voucher_module;
use crate::zone_module;
use std::error::Error;

pub fn router(route: Route, data: &StandardInput) -> Result<JsonValue, Box<dyn Error>> {
    return match route.endpoint.as_str() {
        "get_zones" => {
            if let Some(RoutePayload::Zone(value)) = route.payload {
                let ZoneActions::Get(value) = value;
                zone_module::get_zones(value)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Get Zone does not meet requirements".into(),
                }))
            }
        }
        "check_point_in_zones" => {
            if let Some(RoutePayload::Point(value)) = route.payload {
                zone_module::check_point_in_zone(value)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Point does not meet requirements".into(),
                }))
            }
        }
        "buy_ticket" => {
            if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Buy(value) = value {
                    ticket_module::buy_ticket(value, &data)
                } else {
                    Err(Box::new(ErrorOutput {
                        error: "Validation failed! Buy Ticket does not meet requirements".into(),
                    }))
                }
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Ticket does not meet requirements".into(),
                }))
            }
        }
        "get_tickets" => {
            if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Get(value) = value {
                    ticket_module::get_tickets(value)
                } else {
                    Err(Box::new(ErrorOutput {
                        error: "Validation failed! Get Ticket does not meet requirements".into(),
                    }))
                }
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Ticket does not meet requirements".into(),
                }))
            }
        }
        "validate_ticket" => {
            if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Validate(value) = value {
                    ticket_module::validate_ticket(value)
                } else {
                    Err(Box::new(ErrorOutput {
                        error: "Validation failed! Validate Ticket does not meet requirements".into(),
                    }))
                }
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Ticket does not meet requirements".into(),
                }))
            }
        }
        "get_app_balance" => {
            if let Some(RoutePayload::Balance(value)) = route.payload {
                if let BalanceActions::Get(value) = value {
                    balance_module::get_app_balance(&value)
                } else {
                    Err(Box::new(ErrorOutput {
                        error: "Validation failed! Get Balance does not meet requirements".into(),
                    }))
                }
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Balance does not meet requirements".into(),
                }))
            }
        }
        "withdraw_funds" => {
            if let Some(RoutePayload::Balance(value)) = route.payload {
                if let BalanceActions::Withdraw(value) = value {
                    balance_module::withdraw_funds(value, data)
                } else {
                    Err(Box::new(ErrorOutput {
                        error: "Validation failed! Withdraw funds does not meet requirements".into(),
                    }))
                }
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Withdraw funds does not meet requirements".into(),
                }))
            }
        }
        "seed_zone" => {
            if let Some(RoutePayload::Seed(value)) = route.payload {
                let SeederActions::Zone(value) = value;
                zone_module::seed_zone(value, data)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Seed Zone does not meet requirements".into(),
                }))
            }
        }
        "remove_zone" => {
            if let Some(RoutePayload::Remove(value)) = route.payload {
                zone_module::remove_zone(value, data)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Remove Zone does not meet requirements".into(),
                }))
            }
        }
        "get_vouchers" => {
            if let Some(RoutePayload::Voucher(value)) = route.payload {
                let VoucherActions::Get(value) = value;
                voucher_module::get_vouchers(value)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Get Voucher does not meet requirements".into(),
                }))
            }
        }
        _ => todo!(),
    };
}

pub fn response_type_handler(route: &Route) -> ResponseType {
    return match route.endpoint.as_str() {
        "get_zones" => ResponseType::Report,
        "check_point_in_zones" => ResponseType::Report,
        "buy_ticket" => ResponseType::Notice,
        "get_tickets" => ResponseType::Report,
        "validate_ticket" => ResponseType::Report,
        "get_app_balance" => ResponseType::Report,
        "withdraw_funds" => ResponseType::Voucher,
        "seed_zone" => ResponseType::Notice,
        "remove_zone" => ResponseType::Notice,
        "get_vouchers" => ResponseType::Report,
        &_ => todo!(),
    };
}
