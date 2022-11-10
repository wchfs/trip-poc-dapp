use crate::core::*;
use crate::structures::*;
use std::error::Error;

pub fn router(route: Route, data: &StandardInput) -> Result<String, Box<dyn Error>> {
    return match route.endpoint.as_str() {
        "get_zones" => get_zones(),
        "check_point_in_zones" => {
            if let Some(RoutePayload::Point(value)) = route.payload {
                return check_point_in_zone(value);
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Point does not meet requirements".into(),
                }));
            }
        }
        "buy_ticket" => {
            if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Buy(value) = value {
                    return buy_ticket(value, &data);
                } else {
                    return Err(Box::new(ErrorOutput {
                        error: "Validation failed! Buy Ticket does not meet requirements".into(),
                    }));
                }
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Ticket does not meet requirements".into(),
                }));
            }
        }
        "get_tickets" => {
            if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Get(value) = value {
                    return get_tickets(value);
                } else {
                    return Err(Box::new(ErrorOutput {
                        error: "Validation failed! Get Ticket does not meet requirements".into(),
                    }));
                }
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Ticket does not meet requirements".into(),
                }));
            }
        }
        "validate_ticket" => {
            if let Some(RoutePayload::Ticket(value)) = route.payload {
                if let TicketActions::Validate(value) = value {
                    return validate_ticket(value);
                } else {
                    return Err(Box::new(ErrorOutput {
                        error: "Validation failed! Validate Ticket does not meet requirements".into(),
                    }));
                }
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Ticket does not meet requirements".into(),
                }));
            }
        }
        "get_app_balance" => {
            if let Some(RoutePayload::Balance(value)) = route.payload {
                if let BalanceActions::Get(value) = value {
                    return get_app_balance(&value);
                } else {
                    return Err(Box::new(ErrorOutput {
                        error: "Validation failed! Get Balance does not meet requirements".into(),
                    }));
                }
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Balance does not meet requirements".into(),
                }));
            }
        }
        "withdraw_funds" => {
            if let Some(RoutePayload::Balance(value)) = route.payload {
                if let BalanceActions::Withdraw(value) = value {
                    return withdraw_funds(value, data);
                } else {
                    return Err(Box::new(ErrorOutput {
                        error: "Validation failed! Withdraw funds does not meet requirements".into(),
                    }));
                }
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Withdraw funds does not meet requirements".into(),
                }));
            }
        }
        "seed_zone" => {
            if let Some(RoutePayload::Seed(value)) = route.payload {
                let SeederActions::Zone(value) = value;
                return seed_zone(value, data);
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Seed Zone does not meet requirements".into(),
                }));
            }
        }
        "remove_zone" => {
            if let Some(RoutePayload::Remove(value)) = route.payload {
                return remove_zone(value, data);
            } else {
                return Err(Box::new(ErrorOutput {
                    error: "Validation failed! Remove Zone does not meet requirements".into(),
                }));
            }
        }
        _ => todo!(),
    };
}

pub fn response_type_handler(route: &Route) -> &'static str {
    return match route.endpoint.as_str() {
        "get_zones" => ResponseType::Report.as_str(),
        "check_point_in_zones" => ResponseType::Report.as_str(),
        "buy_ticket" => ResponseType::Notice.as_str(),
        "get_tickets" => ResponseType::Report.as_str(),
        "validate_ticket" => ResponseType::Report.as_str(),
        "get_app_balance" => ResponseType::Report.as_str(),
        "withdraw_funds" => ResponseType::Voucher.as_str(),
        "seed_zone" => ResponseType::Notice.as_str(),
        "remove_zone" => ResponseType::Notice.as_str(),
        &_ => todo!(),
    };
}
