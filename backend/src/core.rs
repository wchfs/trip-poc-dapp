use geo::{Coordinate, Point, Contains};
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use diesel::insert_into;
use crate::establish_connection;
use crate::models::{Zone, Ticket};
use crate::diesel::prelude::*;
use chrono::prelude::*;
use crate::structures::{*};

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

pub fn check_point_in_zone(point: GeoPoint) -> String
{
    let point: Point = point_mapper(point);

    return is_in_the_toll_zone(point).to_string();
}

pub fn point_mapper(point: GeoPoint) -> Point
{
    return Point(
        Coordinate {
            x: point.longitude,
            y: point.latitude
        }
    );
}

pub fn is_in_the_toll_zone(gps_data: Point) -> i32
{
    let zones: Vec<Zone> = get_all_zones();
    for zone in zones {
        let polygon = get_polygon(zone.geo_json);

        if polygon.contains(&gps_data) {
            return zone.id
        }
    }

    return 0;
}

pub fn get_polygon(geo_json_string: String) -> GeometryCollection
{
    //Example point in the zone: 19.943540573120117,50.0565299987793
    let geo_json: GeoJson = geo_json_string.parse::<GeoJson>().unwrap();

    let collection: GeometryCollection<f64> = quick_collection(&geo_json).unwrap();

    return collection;
}

pub fn buy_ticket(data: BuyTicket, additional_data: StandardInput) -> String
{
    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    let amount = (additional_data.uint256.clone().expect("Empty amount field").into_uint().expect("Failed parsing amount") / WEI_TO_GWEI_FACTOR).to_string();
    let calculated_amount = calculate_amount(data.zone_id, data.duration).to_string();

    if amount == calculated_amount
    {
        let wallet = additional_data.address.clone().expect("Empty address field").to_string();

        let is_inserted = insert_into(tickets::table)
            .values((
                license.eq(data.license),
                longitude.eq(data.longitude),
                latitude.eq(data.latitude),
                owner_address.eq(format!("0x{}", wallet)),
                purchased_at.eq(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
                started_at.eq(data.started_at),
                duration.eq(data.duration),
                zone_id.eq(data.zone_id),
                paid.eq(&amount),
                to_pay.eq(&calculated_amount) // In the future, we want to add the possibility to pay extra later.
            ))
            .execute(&connection)
            .unwrap();

        if is_inserted > 0 {
            let ticket = tickets::table
                .filter(owner_address.eq(format!("0x{}", wallet)))
                .order(id.desc())
                .first::<Ticket>(&connection)
                .expect("No ticket found");

            return serde_json::to_string(&ticket).unwrap();
        }
    }

    return error_json_string("Ticket validation error!".to_string());
}

pub fn calculate_amount(zone_id: i32, duration: i32) -> f64
{
    use crate::schema::zones::{self, *};

    let connection = establish_connection();

    let zone_price_per_hour = zones::table
        .filter(id.eq(zone_id))
        .select(price)
        .first::<String>(&connection)
        .expect("Zone not found");

    let pricing: f64 = zone_price_per_hour.parse::<f64>().expect("Pricing parse failed");

    let hours: f64 = (duration / 60) as f64;

    return pricing * hours;
}

pub fn get_tickets(data: GetTicket) -> String
{
    if data.license.is_none() && data.owner_address.is_none() {
        return error_json_string("Missing license and owner address!".to_string());
    }

    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    let mut tickets_query = tickets::table.order(id.desc()).into_boxed();

    if !data.license.is_none() {
        tickets_query = tickets_query.filter(license.eq(data.license.expect("No license provided").to_string()));
    }

    if !data.owner_address.is_none() {
        tickets_query = tickets_query.filter(owner_address.eq(data.owner_address.expect("No Owner address provided").to_string()));
    }

    return match tickets_query.load::<Ticket>(&connection) {
        Ok(t) => serde_json::to_string(&t).unwrap(),
        Err(val) => error_json_string(val.to_string()),
    };
}

pub fn validate_ticket(data: ValidateTicket) -> String
{
    use crate::schema::tickets::{self, *};
    let connection = establish_connection();

    return match tickets::table
        .filter(license.eq(&data.license.to_string()))
        .order(id.desc())
        .load::<Ticket>(&connection) {
        Ok(filtered_tickets) => {
            let mut validate_msg = error_json_string("There is no valid ticket available".to_string());

            for t in &filtered_tickets {
                let ticket_date = t.started_at.parse::<DateTime<Utc>>().unwrap();
                let diff = (Utc::now() - ticket_date).num_minutes();

                if diff < t.duration.into() && diff > 0 {
                    validate_msg = serde_json::to_string(&t).unwrap();
                    break;
                }
            };

            return validate_msg;
        }
        Err(val) => error_json_string(val.to_string())
    };
}

pub fn error_json_string(data: String) -> String
{
    return serde_json::to_string(&ErrorOutput{ error: data }).unwrap();
}
