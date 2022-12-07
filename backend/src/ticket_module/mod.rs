use crate::diesel::prelude::*;
use crate::{establish_connection, helper, balance_module};
use crate::models::{Ticket, Zone};
use crate::structures::*;
use chrono::prelude::*;
use json::{object, JsonValue};
use std::error::Error;

pub fn buy_ticket(
    data: BuyTicket,
    additional_data: &StandardInput,
) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::tickets::{self, *};
    let mut connection = establish_connection();

    let received_uint256_token = match additional_data.uint256.clone() {
        Some(value) => value,
        None => {
            return Err(Box::new(ErrorOutput {
                error: "Empty amount field".into(),
            }))
        }
    };

    let received_uint = match received_uint256_token.into_uint() {
        Some(value) => value,
        None => {
            return Err(Box::new(ErrorOutput {
                error: "Failed parsing amount".into(),
            }))
        }
    };

    let amount = (received_uint / WEI_TO_GWEI_FACTOR).to_string();

    let calculated_amount = calculate_amount(data.zone_id, data.duration)?.to_string();
    
    if amount == calculated_amount {
        let received_address = match additional_data.address.clone() {
            Some(value) => value.to_lowercase(),
            None => {
                return Err(Box::new(ErrorOutput {
                    error: "Empty address field".into(),
                }))
            }
        };

        let wallet = received_address.to_string();

        let is_inserted = diesel::insert_into(tickets::table)
            .values((
                license.eq(data.license),
                longitude.eq(data.longitude),
                latitude.eq(data.latitude),
                owner_address.eq(format!("0x{}", wallet)),
                purchased_at.eq(helper::request_timestamp(&additional_data)?),
                started_at.eq(data.started_at),
                duration.eq(data.duration),
                zone_id.eq(data.zone_id),
                paid.eq(&amount),
                to_pay.eq(&calculated_amount), // In the future, we want to add the possibility to pay extra later.
                status.eq(TicketStatus::Paid as i32),
            ))
            .execute(&mut connection)?;

        if is_inserted > 0 {
            let ticket = tickets::table
                .filter(owner_address.eq(format!("0x{}", wallet)))
                .order(id.desc())
                .first::<Ticket>(&mut connection)?;

            balance_module::add_funds_to_balance(amount, &data.zone_id)?;

            return Ok(object! {
                "data": ticket
            });
        }
    }

    return Err(Box::new(ErrorOutput {
        error: "Ticket validation error!".into(),
    }));
}

fn calculate_amount(zone_id: i32, duration: i32) -> Result<f64, Box<dyn Error>> {
    use crate::schema::zones::{self, *};
    let mut connection = establish_connection();
    
    let zone_price_per_hour = zones::table
        .filter(id.eq(zone_id))
        .select(price)
        .first::<String>(&mut connection)?;
    
    let pricing: f64 = zone_price_per_hour.parse::<f64>()?;
    
    let hours: f64 = ((duration as f64) / (60 as f64)) as f64;
    
    return Ok(pricing * hours);
}

pub fn get_tickets(data: GetTicket) -> Result<JsonValue, Box<dyn Error>> {
    if data.license.is_none() && data.owner_address.is_none() {
        return Err(Box::new(ErrorOutput {
            error: "Missing license and owner address!".into(),
        }));
    }

    use crate::schema::tickets::{self, *};
    let mut connection = establish_connection();

    let mut tickets_query = tickets::table.order(id.desc()).into_boxed();

    if !data.license.is_none() {
        let received_license = match data.license {
            Some(value) => value.to_string(),
            None => {
                return Err(Box::new(ErrorOutput {
                    error: "No license provided".into(),
                }))
            }
        };

        tickets_query = tickets_query.filter(license.eq(received_license));
    }

    if !data.owner_address.is_none() {
        let received_owner_address = match data.owner_address {
            Some(value) => value.to_lowercase(),
            None => {
                return Err(Box::new(ErrorOutput {
                    error: "No Owner address provided".into(),
                }))
            }
        };

        tickets_query = tickets_query.filter(owner_address.eq(received_owner_address));
    }

    let results = tickets_query.load::<Ticket>(&mut connection)?;

    return Ok(object! {
        "data": results
    });
}

pub fn validate_ticket(data: ValidateTicket) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::{*};
    let mut connection = establish_connection();
    
    return match tickets::table
        .filter(tickets::license.eq(&data.license.to_string()))
        .inner_join(zones::table)                                             
        .select((
            (tickets::all_columns),
            (zones::all_columns)
        ))
        .order(tickets::id.desc())
        .load::<(Ticket, Zone)>(&mut connection)
    {
        Ok(filtered_tickets) => {
            for t in filtered_tickets {
                
                let ticket_date = t.0.started_at.parse::<DateTime<Utc>>()?;
                let date_to_check = data.date.parse::<DateTime<Utc>>()?;
                let diff = (date_to_check - ticket_date).num_minutes();

                if diff < t.0.duration.into() && diff > 0 {
                    let mut object = object! {
                        "data": t.0
                    };
                    
                    object["data"].insert("zone", t.1)?;
                    
                    return Ok(object);
                }
            }

            return Err(Box::new(ErrorOutput {
                error: "There is no valid ticket available".into(),
            }));
        }
        Err(val) => Err(Box::new(val)),
    };
}