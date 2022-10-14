use crate::structures::{*};
use crate::core::{*};

pub fn router(route: Route, data: StandardInput) -> String
{
    return match route.endpoint.as_str() {
        "get_zones" => get_zones(),
        "check_point_in_zones" => if let Some(RoutePayload::Point(value)) = route.payload { return check_point_in_zone(value) } else { panic!("") }
        "buy_ticket" => if let Some(RoutePayload::Ticket(value)) = route.payload {
            if let TicketActions::Buy(value) = value {
                return buy_ticket(value, data)
            } else { panic!("Validation failed! Buy Ticket does not meet requirements") }
        } else { panic!("Validation failed! Ticket does not meet requirements") }
        "get_tickets" => if let Some(RoutePayload::Ticket(value)) = route.payload {
            if let TicketActions::Get(value) = value {
                return get_tickets(value)
            } else { panic!("Validation failed! Get Ticket does not meet requirements") }
        } else { panic!("Validation failed! Ticket does not meet requirements") }
        "validate_ticket" => if let Some(RoutePayload::Ticket(value)) = route.payload {
            if let TicketActions::Validate(value) = value {
                return validate_ticket(value)
            } else { panic!("Validation failed! Validate Ticket does not meet requirements") }
        } else { panic!("Validation failed! Ticket does not meet requirements") }
        "get_app_balance" => get_app_balance(),
        &_ => todo!(),
    };
}

pub fn response_type_handler(route: &Route) -> &'static str
{
    return match route.endpoint.as_str() {
        "get_zones" => ResponseType::Report.as_str(),
        "check_point_in_zones" => ResponseType::Report.as_str(),
        "buy_ticket" => ResponseType::Notice.as_str(),
        "get_tickets" => ResponseType::Report.as_str(),
        "validate_ticket" => ResponseType::Report.as_str(),
        "get_app_balance" => ResponseType::Report.as_str(),
        &_ => todo!(),
    }
}
