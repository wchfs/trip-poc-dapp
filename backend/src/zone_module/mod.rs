use crate::diesel::prelude::*;
use crate::{establish_connection, helper, balance_module};
use crate::models::{Zone};
use crate::structures::*;
use geo::{Contains, Coordinate, Point};
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use json::{object, JsonValue};
use std::error::Error;

pub fn get_zones(filters: GetZone) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::zones::{self, *};
    let mut connection = establish_connection();

    let mut query = zones::table.into_boxed();

    if let Some(received_value) = filters.zone_id {
        query = query.filter(id.eq(received_value));
    }

    if let Some(received_value) = filters.owner_address {
        query = query.filter(owner_address.eq(received_value));
    }

    let pagination: Pagination = match filters.paginate {
        Some(value) => value,
        None => Default::default(),
    };

    let offset = pagination.page - 1;
    query = query
        .limit(pagination.per_page)
        .offset(offset * pagination.per_page);

    return Ok(object! {
        "data": query.load::<Zone>(&mut connection)?,
    });
}

pub fn check_point_in_zone(point: GeoPoint) -> Result<JsonValue, Box<dyn Error>> {
    let point: Point = point_mapper(point);

    return Ok(object! {
        "data": is_in_the_toll_zone(point)?,
    });
}

fn point_mapper(point: GeoPoint) -> Point {
    return Point(Coordinate {
        x: point.longitude,
        y: point.latitude,
    });
}

fn is_in_the_toll_zone(gps_data: Point) -> Result<Option<Zone>, Box<dyn Error>> {
    use crate::schema::zones::{self};
    let mut connection = establish_connection();

    let zones = zones::table.load::<Zone>(&mut connection)?;

    for zone in zones {
        let polygon = get_polygon(&zone.geo_json)?;

        if polygon.contains(&gps_data) {
            return Ok(Some(zone));
        }
    }

    return Ok(None);
}

fn get_polygon(geo_json_string: &String) -> Result<GeometryCollection, Box<dyn Error>> {
    //Example point in the zone: 19.943540573120117,50.0565299987793
    let geo_json: GeoJson = geo_json_string.parse::<GeoJson>()?;

    let collection: GeometryCollection<f64> = quick_collection(&geo_json)?;

    return Ok(collection);
}

pub fn check_zone_owner(requested_zone_id: &i32, requested_by: &String) -> bool {
    use crate::schema::zones::dsl::*;
    let mut connection = establish_connection();

    let result: Result<i64, diesel::result::Error> = zones
        .find(requested_zone_id)
        .filter(owner_address.eq(requested_by.to_lowercase()))
        .count()
        .get_result(&mut connection);

    return match result {
        Ok(count) => {
            if count > 0 {
                true;
            }
            false
        }
        Err(_) => false,
    };
}

pub fn seed_zone(
    data: ZoneSeeder,
    additional_data: &StandardInput,
) -> Result<JsonValue, Box<dyn Error>> {
    let wallet = helper::parse_request_addres(additional_data)?;

    if helper::super_wallet_validator(wallet)? {}

    use crate::schema::zones::{self, *};
    let mut connection = establish_connection();

    let is_inserted = diesel::insert_into(zones::table)
        .values((
            name.eq(data.name),
            price.eq(data.price),
            geo_json.eq(data.geo_json),
            owner_address.eq(&data.zone_owner_address.to_lowercase()),
        ))
        .execute(&mut connection)?;

    if is_inserted > 0 {
        let zone = zones::table
            .filter(owner_address.eq(data.zone_owner_address.to_lowercase()))
            .order(id.desc())
            .first::<Zone>(&mut connection)?;

        balance_module::create_zone_balance(&zone)?;

        return Ok(object! {
            "data": zone
        });
    }
    return Err(Box::new(ErrorOutput {
        error: "Seed zone error!".into(),
    }));
}

pub fn remove_zone(
    data: Remover,
    additional_data: &StandardInput,
) -> Result<JsonValue, Box<dyn Error>> {
    let wallet = helper::parse_request_addres(additional_data)?;

    if helper::super_wallet_validator(wallet)? {}

    balance_module::remove_zone_balance(&data)?;

    use crate::schema::zones::{self, *};
    let mut connection = establish_connection();

    let result = diesel::delete(zones::table.filter(id.eq(data.id))).execute(&mut connection);

    match result {
        Ok(value) => {
            if value.eq(&0) {
                return Err(Box::new(ErrorOutput {
                    error: "Zone not deleted!".to_string(),
                }));
            }
            return Ok(object! {
                "data": helper::success_json_string("Zone deleted!".to_string())
            });
        }
        Err(value) => {
            return Err(Box::new(ErrorOutput {
                error: value.to_string(),
            }));
        }
    }
}