use crate::diesel::prelude::*;
use crate::establish_connection;
use crate::models::{Ticket, Zone};
use crate::structures::*;
use chrono::prelude::*;
use diesel::insert_into;
use geo::{Contains, Coordinate, Point};
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use json::{object, JsonValue};
use std::error::Error;
use std::str::FromStr;

pub fn get_zones() -> Result<JsonValue, Box<dyn Error>> {
    let zones = get_all_zones()?;

    return Ok(object! {
        "data": zones,
    });
}

pub fn get_all_zones() -> Result<Vec<Zone>, Box<dyn Error>> {
    use crate::schema::zones::dsl::*;
    let connection = establish_connection();

    let results = zones
        .order(id.desc())
        .limit(100)
        .load::<Zone>(&connection)?;

    return Ok(results);
}

pub fn check_point_in_zone(point: GeoPoint) -> Result<JsonValue, Box<dyn Error>> {
    let point: Point = point_mapper(point);

    return Ok(object! {
        "data": is_in_the_toll_zone(point)?.to_string(),
    });
}

pub fn point_mapper(point: GeoPoint) -> Point {
    return Point(Coordinate {
        x: point.longitude,
        y: point.latitude,
    });
}

pub fn is_in_the_toll_zone(gps_data: Point) -> Result<i32, Box<dyn Error>> {
    let zones: Vec<Zone> = get_all_zones()?;

    for zone in zones {
        let polygon = get_polygon(zone.geo_json)?;

        if polygon.contains(&gps_data) {
            return Ok(zone.id);
        }
    }

    return Ok(0);
}

pub fn get_polygon(geo_json_string: String) -> Result<GeometryCollection, Box<dyn Error>> {
    //Example point in the zone: 19.943540573120117,50.0565299987793
    let geo_json: GeoJson = geo_json_string.parse::<GeoJson>()?;

    let collection: GeometryCollection<f64> = quick_collection(&geo_json)?;

    return Ok(collection);
}

pub fn buy_ticket(data: BuyTicket, additional_data: &StandardInput) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    let received_uint256_token = match additional_data.uint256.clone() {
        Some(value) => value,
        None => return Err(Box::new(ErrorOutput {
            error: "Empty amount field".into(),
        })),
    };

    let received_uint = match received_uint256_token.into_uint() {
        Some(value) => value,
        None => return Err(Box::new(ErrorOutput {
            error: "Failed parsing amount".into(),
        })),
    };

    let amount = (received_uint / WEI_TO_GWEI_FACTOR).to_string();
    
    let calculated_amount = calculate_amount(data.zone_id, data.duration)?.to_string();

    if amount == calculated_amount {
        let received_address = match additional_data.address.clone() {
            Some(value) => value,
            None => return Err(Box::new(ErrorOutput {
                error: "Empty address field".into(),
            })),
        };

        let wallet = received_address.to_string();

        let is_inserted = insert_into(tickets::table)
            .values((
                license.eq(data.license),
                longitude.eq(data.longitude),
                latitude.eq(data.latitude),
                owner_address.eq(format!("0x{}", wallet)),
                purchased_at.eq(request_timestamp(&additional_data)?),
                started_at.eq(data.started_at),
                duration.eq(data.duration),
                zone_id.eq(data.zone_id),
                paid.eq(&amount),
                to_pay.eq(&calculated_amount), // In the future, we want to add the possibility to pay extra later.
                status.eq(TicketStatus::Paid as i32),
            ))
            .execute(&connection)?;

        if is_inserted > 0 {
            let ticket = tickets::table
                .filter(owner_address.eq(format!("0x{}", wallet)))
                .order(id.desc())
                .first::<Ticket>(&connection)?;

            add_funds_to_balance(amount, &data.zone_id)?;

            return Ok(object! {
                "data": ticket
            });
        }
    }

    return Err(Box::new(ErrorOutput {
        error: "Ticket validation error!".into(),
    }));
}

pub fn calculate_amount(zone_id: i32, duration: i32) -> Result<f64, Box<dyn Error>> {
    use crate::schema::zones::{self, *};
    let connection = establish_connection();

    let zone_price_per_hour = zones::table
        .filter(id.eq(zone_id))
        .select(price)
        .first::<String>(&connection)?;

    let pricing: f64 = zone_price_per_hour.parse::<f64>()?;

    let hours: f64 = (duration / 60) as f64;

    return Ok(pricing * hours);
}

pub fn get_tickets(data: GetTicket) -> Result<JsonValue, Box<dyn Error>> {
    if data.license.is_none() && data.owner_address.is_none() {
        return Err(Box::new(ErrorOutput {
            error: "Missing license and owner address!".into(),
        }));
    }

    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    let mut tickets_query = tickets::table.order(id.desc()).into_boxed();

    if !data.license.is_none() {
        let received_license = match data.license {
            Some(value) => value.to_string(),
            None => return Err(Box::new(ErrorOutput {
                error: "No license provided".into(),
            }))
        };
        
        tickets_query = tickets_query.filter(license.eq(received_license));
    }

    if !data.owner_address.is_none() {
        let received_owner_address = match data.owner_address {
            Some(value) => value.to_string(),
            None => return Err(Box::new(ErrorOutput {
                error: "No Owner address provided".into(),
            }))
        };

        tickets_query = tickets_query.filter(owner_address.eq(received_owner_address));
    }

    let results = tickets_query.load::<Ticket>(&connection)?;

    return Ok(object! {
        "data": results
    });
}

pub fn validate_ticket(data: ValidateTicket) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    return match tickets::table
        .filter(license.eq(&data.license.to_string()))
        .order(id.desc())
        .load::<Ticket>(&connection)
    {
        Ok(filtered_tickets) => {
            for t in filtered_tickets {
                let ticket_date = t.started_at.parse::<DateTime<Utc>>()?;
                let date_to_check = data.date.parse::<DateTime<Utc>>()?;
                let diff = (date_to_check - ticket_date).num_minutes();

                if diff < t.duration.into() && diff > 0 {
                    return Ok(object! {
                        "data": t
                    });
                }
            }

            return Err(Box::new(ErrorOutput {
                error: "There is no valid ticket available".into(),
            }));
        }
        Err(val) => Err(Box::new(val)),
    };
}

pub fn get_app_balance(data: &GetBalance) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::balances::{self, *};
    let connection = establish_connection();
    
    return Ok(object! {
        "data": balances::table
            .select(amount)
            .filter(zone_id.eq(data.zone_id))
            .first::<String>(&connection)?
    });
}

pub fn get_current_amount(zone_id: &i32) -> Result<i128, Box<dyn Error>> {

    return Ok(get_app_balance(&GetBalance { zone_id: (*zone_id) })?["data"].to_string().parse::<i128>()?)
}

pub fn add_funds_to_balance(value: String, requested_zone_id: &i32) -> Result<(), Box<dyn Error>> {
    let current_amount = get_current_amount(requested_zone_id)?;

    let parsed_value = value.parse::<i128>()?;

    let new_amount = (current_amount + parsed_value).to_string();

    update_balance(new_amount, requested_zone_id)?;

    return Ok(());
}

pub fn remove_funds_from_balance(value: String, requested_zone_id: &i32) -> Result<(), Box<dyn Error>> {
    let current_amount = get_current_amount(requested_zone_id)?;

    let parsed_value = value.parse::<i128>()?;

    let new_amount = (current_amount - parsed_value).to_string();

    update_balance(new_amount, requested_zone_id)?;

    return Ok(());
}

pub fn update_balance(new_amount: String, requested_zone_id: &i32) -> Result<(), Box<dyn Error>> {
    use crate::schema::balances::{self, *};
    let connection = establish_connection();

    diesel::update(balances::table)
        .filter(zone_id.eq(requested_zone_id))
        .set(amount.eq(new_amount))
        .execute(&connection)?;

    return Ok(());
}

pub fn withdraw_funds(withdraw_struct: WithdrawFunds, additional_data: &StandardInput) -> Result<JsonValue, Box<dyn Error>> {
    let app_balance = &get_current_amount(&withdraw_struct.zone_id)?;
    
    let parsed_amount = &withdraw_struct
        .amount
        .parse::<i128>()?;

    if parsed_amount > app_balance {
        return Err(Box::new(ErrorOutput {
            error: "Insufficient funds to withdraw".into(),
        }));
    }

    let owner_address = match additional_data.address.as_ref() {
        Some(value) => value,
        None => return Err(Box::new(ErrorOutput {
            error: "No Owner address provided".into(),
        }))
    };
    
    if check_zone_owner(&withdraw_struct.zone_id, owner_address) {
        return Err(Box::new(ErrorOutput {
            error: "Only owner can withdraw funds".into(),
        }));
    }

    remove_funds_from_balance(withdraw_struct.amount.clone(), &withdraw_struct.zone_id)?;

    let parsed_address = ethabi::ethereum_types::H160::from_slice(
        owner_address.as_bytes(),
    );

    let address_token = ethabi::Token::Address(parsed_address);

    let uint_parsed_token =
        ethabi::ethereum_types::U256::from_str(&withdraw_struct.amount)?;

    let amount_token = ethabi::Token::Uint(uint_parsed_token);

    let withdrawal_data = ethabi::encode(&[address_token, amount_token]);

    let abi_bytes = ethabi::Token::Bytes(withdrawal_data);

    let mut data = ethabi::encode(&[abi_bytes]);

    let mut payload = ETHER_WITHDRAWAL_HEADER.as_slice().to_vec();
    payload.append(&mut data);

    let encoded_payload = hex::encode(payload);

    return Ok(object! {
        "data": encoded_payload
    });
}

fn check_zone_owner(requested_zone_id: &i32, requested_by: &String) -> bool {
    use crate::schema::zones::dsl::*;
    let connection = establish_connection();

    let result = zones
        .filter(owner_address.eq(requested_by))
        .filter(id.eq(requested_zone_id))
        .count()
        .execute(&connection);

    return match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn request_timestamp(data: &StandardInput) -> Result<String, Box<dyn Error>> {
    let parsed_request_timestamp = parse_request_timestamp(data)?;

    let rfc = parsed_request_timestamp.to_rfc3339_opts(SecondsFormat::Millis, true);

    return Ok(rfc);
}

fn parse_request_timestamp(data: &StandardInput) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let timestamp = match data.request["data"]["metadata"]["timestamp"]
            .clone()
            .as_i64() {
        Some(it) => it,
        None => return Err(Box::new(ErrorOutput {
            error: "Missin timestamp".into(),
        })),
    };

    return Ok(Utc.timestamp(timestamp, 0));
}

pub fn seed_zone(data: ZoneSeeder, additional_data: &StandardInput) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::zones::{self, *};
    let connection = establish_connection();

    let wallet = parse_request_addres(additional_data)?;

    let is_inserted = insert_into(zones::table)
        .values((
            name.eq(data.name),
            price.eq(data.price),
            geo_json.eq(data.geo_json),
            owner_address.eq(&wallet),
        ))
        .execute(&connection)?;

    if is_inserted > 0 {
        let zone = zones::table
            .filter(owner_address.eq(wallet))
            .order(id.desc())
            .first::<Zone>(&connection)?;

        create_zone_balance(&zone)?;

        return Ok(object! {
            "data": zone
        });
    }
    return Err(Box::new(ErrorOutput {
        error: "Seed zone error!".into(),
    }));
}

fn create_zone_balance(zone: &Zone) -> Result<(), Box<dyn Error>> {
    use crate::schema::balances::{self, *};
    let connection = establish_connection();

    insert_into(balances::table)
        .values((
            zone_id.eq(zone.id),
            amount.eq("0"),
        ))
        .execute(&connection)?;

    return Ok(());
}

pub fn remove_zone(data: Remover, additional_data: &StandardInput) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::zones::{self, *};
    let connection = establish_connection();

    let wallet = parse_request_addres(additional_data)?;
    
    let result = diesel::delete(zones::table
        .filter(id.eq(data.id))
        .filter(owner_address.eq(wallet))
    ).execute(&connection);

    match result {
        Ok(value) => {
            if value.eq(&0) {
                return Err(Box::new(ErrorOutput {
                    error: "Zone not deleted!".to_string(),
                }))
            }
            return Ok(object! {
                "data": success_json_string("Zone deleted!".to_string())
            });
        },
        Err(value) => return Err(Box::new(ErrorOutput {
            error: value.to_string(),
        }))
    }
}

fn parse_request_addres(data: &StandardInput) -> Result<String, Box<dyn Error>> {
    let wallet = match data
            .address
            .clone() {
        Some(it) => it,
        None => return Err(Box::new(ErrorOutput {
            error: "Missin address".into(),
        })),
    }.to_string();

    return Ok(format!("0x{}", wallet));
}

pub fn error_json_string(data: String) -> String {
    return serde_json::to_string(&ErrorOutput { error: data }).expect("Static typing ensures that there will not be any errors.");
}

pub fn success_json_string(data: String) -> String {
    return serde_json::to_string(&SuccessOutput { message: data }).expect("Static typing ensures that there will not be any errors.");
}