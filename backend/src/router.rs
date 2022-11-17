use json::JsonValue;
use crate::core::*;
use crate::structures::*;
use std::error::Error;

pub fn router(route: Route, data: &StandardInput) -> Result<JsonValue, Box<dyn Error>> {
    return match route.endpoint.as_str() {
        "get_zones" => get_zones(),
        "check_point_in_zones" => {
            return if let Some(RoutePayload::Point(value)) = route.payload {
                check_point_in_zone(value)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Point does not meet requirements".into(),
                }))
            }
        }
        "buy_ticket" => {
            return if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Buy(value) = value {
                    buy_ticket(value, &data)
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
            return if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Get(value) = value {
                    get_tickets(value)
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
            return if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Validate(value) = value {
                    validate_ticket(value)
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
            return if let Some(RoutePayload::Balance(value)) = route.payload {
                if let BalanceActions::Get(value) = value {
                    get_app_balance(&value)
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
            return if let Some(RoutePayload::Balance(value)) = route.payload {
                if let BalanceActions::Withdraw(value) = value {
                    withdraw_funds(value, data)
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
            return if let Some(RoutePayload::Seed(value)) = route.payload {
                let SeederActions::Zone(value) = value;
                seed_zone(value, data)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Seed Zone does not meet requirements".into(),
                }))
            }
        }
        "remove_zone" => {
            return if let Some(RoutePayload::Remove(value)) = route.payload {
                remove_zone(value, data)
            } else {
                Err(Box::new(ErrorOutput {
                    error: "Validation failed! Remove Zone does not meet requirements".into(),
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
        &_ => todo!(),
    };
}
