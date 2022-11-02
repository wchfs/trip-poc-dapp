use crate::diesel::prelude::*;
use crate::establish_connection;
use crate::models::{Ticket, Zone};
use crate::structures::*;
use chrono::prelude::*;
use diesel::insert_into;
use geo::{Contains, Coordinate, Point};
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use std::str::FromStr;

pub fn get_zones() -> String {
    let zones = get_all_zones();

    return serde_json::to_string(&zones).unwrap();
}

pub fn get_all_zones() -> Vec<Zone> {
    use crate::schema::zones::dsl::*;

    let connection = establish_connection();
    let results = zones
        .limit(5)
        .load::<Zone>(&connection)
        .expect("Error loading zones");

    return results;
}

pub fn check_point_in_zone(point: GeoPoint) -> String {
    let point: Point = point_mapper(point);

    return is_in_the_toll_zone(point).to_string();
}

pub fn point_mapper(point: GeoPoint) -> Point {
    return Point(Coordinate {
        x: point.longitude,
        y: point.latitude,
    });
}

pub fn is_in_the_toll_zone(gps_data: Point) -> i32 {
    let zones: Vec<Zone> = get_all_zones();
    for zone in zones {
        let polygon = get_polygon(zone.geo_json);

        if polygon.contains(&gps_data) {
            return zone.id;
        }
    }

    return 0;
}

pub fn get_polygon(geo_json_string: String) -> GeometryCollection {
    //Example point in the zone: 19.943540573120117,50.0565299987793
    let geo_json: GeoJson = geo_json_string.parse::<GeoJson>().unwrap();

    let collection: GeometryCollection<f64> = quick_collection(&geo_json).unwrap();

    return collection;
}

pub fn buy_ticket(data: BuyTicket, additional_data: &StandardInput) -> String {
    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    let amount = (additional_data
        .uint256
        .clone()
        .expect("Empty amount field")
        .into_uint()
        .expect("Failed parsing amount")
        / WEI_TO_GWEI_FACTOR)
        .to_string();
    let calculated_amount = calculate_amount(data.zone_id, data.duration).to_string();

    if amount == calculated_amount {
        let wallet = additional_data
            .address
            .clone()
            .expect("Empty address field")
            .to_string();

        let is_inserted = insert_into(tickets::table)
            .values((
                license.eq(data.license),
                longitude.eq(data.longitude),
                latitude.eq(data.latitude),
                owner_address.eq(format!("0x{}", wallet)),
                purchased_at.eq(request_timestamp(&additional_data)),
                started_at.eq(data.started_at),
                duration.eq(data.duration),
                zone_id.eq(data.zone_id),
                paid.eq(&amount),
                to_pay.eq(&calculated_amount), // In the future, we want to add the possibility to pay extra later.
                status.eq(TicketStatus::Paid as i32),
            ))
            .execute(&connection)
            .unwrap();

        if is_inserted > 0 {
            let ticket = tickets::table
                .filter(owner_address.eq(format!("0x{}", wallet)))
                .order(id.desc())
                .first::<Ticket>(&connection)
                .expect("No ticket found");

            add_funds_to_balance(amount);

            return serde_json::to_string(&ticket).unwrap();
        }
    }

    return error_json_string("Ticket validation error!".to_string());
}

pub fn calculate_amount(zone_id: i32, duration: i32) -> f64 {
    use crate::schema::zones::{self, *};

    let connection = establish_connection();

    let zone_price_per_hour = zones::table
        .filter(id.eq(zone_id))
        .select(price)
        .first::<String>(&connection)
        .expect("Zone not found");

    let pricing: f64 = zone_price_per_hour
        .parse::<f64>()
        .expect("Pricing parse failed");

    let hours: f64 = (duration / 60) as f64;

    return pricing * hours;
}

pub fn get_tickets(data: GetTicket) -> String {
    if data.license.is_none() && data.owner_address.is_none() {
        return error_json_string("Missing license and owner address!".to_string());
    }

    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    let mut tickets_query = tickets::table.order(id.desc()).into_boxed();

    if !data.license.is_none() {
        tickets_query = tickets_query
            .filter(license.eq(data.license.expect("No license provided").to_string()));
    }

    if !data.owner_address.is_none() {
        tickets_query = tickets_query.filter(
            owner_address.eq(data
                .owner_address
                .expect("No Owner address provided")
                .to_string()),
        );
    }

    return match tickets_query.load::<Ticket>(&connection) {
        Ok(t) => serde_json::to_string(&t).unwrap(),
        Err(val) => error_json_string(val.to_string()),
    };
}

pub fn validate_ticket(data: ValidateTicket) -> String {
    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    return match tickets::table
        .filter(license.eq(&data.license.to_string()))
        .order(id.desc())
        .load::<Ticket>(&connection)
    {
        Ok(filtered_tickets) => {
            let mut validate_msg =
                error_json_string("There is no valid ticket available".to_string());

            for t in &filtered_tickets {
                let ticket_date = t.started_at.parse::<DateTime<Utc>>().unwrap();
                let date_to_check = data.date.parse::<DateTime<Utc>>().unwrap();
                let diff = (date_to_check - ticket_date).num_minutes();

                if diff < t.duration.into() && diff > 0 {
                    validate_msg = serde_json::to_string(&t).unwrap();
                    break;
                }
            }

            return validate_msg;
        }
        Err(val) => error_json_string(val.to_string()),
    };
}

pub fn add_funds_to_balance(value: String) {
    let current_amount = get_current_amount();

    let parsed_value = value.parse::<i128>().expect("Paid amount parse failed");

    let new_amount = (current_amount + parsed_value).to_string();

    update_balance(new_amount);
}

pub fn remove_funds_to_balance(value: String) {
    let current_amount = get_current_amount();

    let parsed_value = value.parse::<i128>().expect("Paid amount parse failed");

    let new_amount = (current_amount - parsed_value).to_string();

    update_balance(new_amount);
}

pub fn get_current_amount() -> i128 {
    return get_app_balance()
        .parse::<i128>()
        .expect("Paid amount parse failed");
}

pub fn update_balance(new_amount: String) {
    use crate::schema::balances::{self, *};
    let connection = establish_connection();

    diesel::update(balances::table)
        .set(amount.eq(new_amount))
        .execute(&connection)
        .expect("Failed to update balance");
}

pub fn get_app_balance() -> String {
    use crate::schema::balances::{self, *};
    let connection = establish_connection();

    return balances::table
        .select(amount)
        .first::<String>(&connection)
        .expect("Balance not found");
}

pub fn withdraw_funds(withdraw_struct: WithdrawFunds, additional_data: &StandardInput) -> String {
    let app_balance = &get_current_amount();
    let parsed_amount = &withdraw_struct
        .amount
        .parse::<i128>()
        .expect("Paid amount parse failed");

    if parsed_amount > app_balance {
        return error_json_string("Insufficient funds to withdraw".to_string());
    }

    remove_funds_to_balance(withdraw_struct.amount.clone());

    let parsed_address = ethabi::ethereum_types::H160::from_slice(
        additional_data.address.as_ref().unwrap().as_bytes(),
    );

    let address_token = ethabi::Token::Address(parsed_address);

    let uint_parsed_token =
        ethabi::ethereum_types::U256::from_str(&withdraw_struct.amount).unwrap();

    let amount_token = ethabi::Token::Uint(uint_parsed_token);

    let withdrawal_data = ethabi::encode(&[address_token, amount_token]);

    let abi_bytes = ethabi::Token::Bytes(withdrawal_data);

    let mut data = ethabi::encode(&[abi_bytes]);

    let mut payload = ETHER_WITHDRAWAL_HEADER.as_slice().to_vec();
    payload.append(&mut data);

    let encoded_payload = hex::encode(payload);

    return encoded_payload;
}

fn request_timestamp(data: &StandardInput) -> String {
    return parse_request_timestamp(data).to_rfc3339_opts(SecondsFormat::Millis, true);
}

fn parse_request_timestamp(data: &StandardInput) -> DateTime<Utc> {
    let timestamp = data.request["data"]["metadata"]["timestamp"]
        .clone()
        .as_i64()
        .unwrap();
    return Utc.timestamp(timestamp, 0);
}

pub fn seed_zone(data: ZoneSeeder, additional_data: &StandardInput) -> String {
    use crate::schema::zones::{self, *};
    let connection = establish_connection();

    let wallet = parse_request_addres(additional_data);

    let is_inserted = insert_into(zones::table)
        .values((
            name.eq(data.name),
            price.eq(data.price),
            geo_json.eq(data.geo_json),
            owner_address.eq(&wallet),
        ))
        .execute(&connection)
        .unwrap();

    if is_inserted > 0 {
        let zone = zones::table
            .filter(owner_address.eq(wallet))
            .order(id.desc())
            .first::<Zone>(&connection)
            .expect("No zone found");

        return serde_json::to_string(&zone).unwrap();
    }

    return error_json_string("Seed zone error!".to_string());
}

pub fn remove_zone(data: Remover, additional_data: &StandardInput) -> String {
    use crate::schema::zones::{self, *};
    let connection = establish_connection();

    let wallet = parse_request_addres(additional_data);
    
    let result = diesel::delete(
        zones::table
        .filter(id.eq(data.id))
        .filter(owner_address.eq(wallet))
    ).execute(&connection);

    match result {
        Ok(value) => {
            if value.eq(&0) {
                return error_json_string(value.to_string())
            }
            return success_json_string(value.to_string())
        },
        Err(value) => return error_json_string(value.to_string())
    }
}

fn parse_request_addres(data: &StandardInput) -> String {
    let wallet = data
        .address
        .clone()
        .expect("Empty address field")
        .to_string();

    return format!("0x{}", wallet);
}

pub fn error_json_string(data: String) -> String {
    return serde_json::to_string(&ErrorOutput { error: data }).unwrap();
}

pub fn success_json_string(data: String) -> String {
    return serde_json::to_string(&SuccessOutput { success: data }).unwrap();
}