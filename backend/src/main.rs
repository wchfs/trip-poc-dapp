// Copyright 2022 Cartesi Pte. Ltd.
//
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use json::{object, JsonValue};
use std::env;
use dotenv::dotenv;
use geo::{Coordinate, Point, Contains};
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use serde::Deserialize;
use ethabi::{ParamType, decode};
use diesel::insert_into;

use point_in_polygon_dapp_paid_parking_assistant::establish_connection;
use point_in_polygon_dapp_paid_parking_assistant::models::{Zone, Ticket};

extern crate point_in_polygon_dapp_paid_parking_assistant;
extern crate diesel;

use self::diesel::prelude::*;
use chrono::prelude::*;

#[derive(Deserialize, Debug, Default)]
struct Route {
    endpoint: String,
    payload: Option<RoutePayload>
}

#[derive(Deserialize, Debug)]
enum RoutePayload {
    Ticket(TicketActions),
    Point(GeoPoint)
}

#[derive(Deserialize, Debug)]
enum TicketActions {
    Buy(BuyTicket),
    Get(GetTicket),
}

#[derive(Deserialize, Debug)]
struct GeoPoint {
    longitude: f64,
    latitude: f64
}

#[derive(Deserialize, Debug)]
struct BuyTicket {
    license: String,
    longitude: f32,
    latitude: f32,
    started_at: String,
    duration: i32,
    zone_id: i32,
}

#[derive(Deserialize, Debug)]
struct GetTicket {
    license: String,
}

fn router(route: Route, request: JsonValue) -> String
{
    return match route.endpoint.as_str() {
        "get_zones" => get_zones(),
        "check_point_in_zones" => if let Some(RoutePayload::Point(value)) = route.payload { return check_point_in_zone(value) } else { panic!("") }
        "buy_ticket" => if let Some(RoutePayload::Ticket(value)) = route.payload {
            if let TicketActions::Buy(value) = value {
                return buy_ticket(value, request)
            } else { panic!("Validation failed! Buy Ticket does not meet requirements") }
        } else { panic!("Validation failed! Ticket does not meet requirements") }
        "get_ticket" => if let Some(RoutePayload::Ticket(value)) = route.payload {
            if let TicketActions::Get(value) = value {
                return get_ticket(value)
            } else { panic!("Validation failed! Get Ticket does not meet requirements") }
        } else { panic!("Validation failed! Ticket does not meet requirements") }
        "my_tickets" => my_tickets(request),
        &_ => todo!(),
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = hyper::Client::new();
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL")?;

    let mut status = "accept";
    let mut _rollup_address = String::new();

    loop {
        println!("Sending finish");

        let response = object! {"status" => status.clone()};

        let request = hyper::Request::builder()
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/finish", &server_addr))
            .body(hyper::Body::from(response.dump()))?;

        let response = client.request(request).await?;

        println!("Received finish status {}", response.status());

        if response.status() == hyper::StatusCode::ACCEPTED {
            println!("No pending rollup request, trying again");
        } else {
            let body = hyper::body::to_bytes(response).await?;
            let utf = std::str::from_utf8(&body)?;
            let req = json::parse(utf)?;

            if let Some(address) = process_initial(&req["data"]["metadata"]) {
                _rollup_address = address;
                continue;
            }

            let request_type = req["request_type"]
                .as_str()
                .ok_or("request_type is not a string")?;

            status = match request_type {
                "advance_state" => handle_advance(&client, &server_addr[..], req).await?,
                "inspect_state" => handle_inspect(&client, &server_addr[..], req).await?,
                &_ => {
                    eprintln!("Unknown request type");
                    "reject"
                }
            };
        }
    }
}

pub async fn handle_advance(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);

    let input = handle_input(request.clone());

    let input_abi_payload = abi_decoder(input);

    let output = handle_output(input_abi_payload, request);

    return Ok(add_response("notice", client, server_addr, output).await?);
}

pub async fn handle_inspect(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);

    let input = handle_input(request.clone());

    let output = handle_output(input, request);

    return Ok(add_response("report", client, server_addr, output).await?);
}

pub async fn add_response(
    response_type: &str,
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    output: String,
) -> Result<&'static str, Box<dyn std::error::Error>>
{
    println!("Adding {}", response_type);

    let add = object! {"payload" => output};

    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!("{}/{}", server_addr, response_type))
        .body(hyper::Body::from(add.dump()))?;

    let response = client.request(req).await?;

    print_response(response).await?;

    Ok("accept")
}

fn handle_input(request: JsonValue) -> Vec<u8>
{
    return hex_decoder(request);
}

fn handle_output(data: Vec<u8>, request: JsonValue) -> String
{
    let route: Route = payload_parser(data);

    let output_payload: String = router(route, request);

    return format!("0x{}", hex::encode(output_payload));
}

fn abi_decoder(data: Vec<u8>) -> Vec<u8>
{
    let abi_parameters = get_abi_ether_parameters();

    let tokens = decode(&abi_parameters, &data).unwrap();

    return tokens[3].clone().into_bytes().unwrap();
}

fn payload_parser(data: Vec<u8>) -> Route
{
    return serde_json::from_slice(&data).unwrap();
}

fn hex_decoder(request: JsonValue) -> Vec<u8>
{
    let payload = prepare_payload(request);

    return hex::decode(&payload.as_str()).unwrap();
}

fn prepare_payload(request: JsonValue) -> String
{
    let mut payload = request["data"]["payload"]
        .to_string();

    //We don't need "0x" for conversion.
    payload.remove(0);
    payload.remove(0);

    return payload;
}

fn get_abi_ether_parameters() -> [ParamType; 4]
{
    return [
        ParamType::FixedBytes(32),
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::Bytes,
    ];
}

fn get_zones() -> String {
    let zones = get_all_zones();

    return serde_json::to_string(&zones).unwrap();
}

fn get_all_zones() -> Vec<Zone> {
    use point_in_polygon_dapp_paid_parking_assistant::schema::zones::dsl::*;

    let connection = establish_connection();
    let results = zones
        .limit(5)
        .load::<Zone>(&connection)
        .expect("Error loading zones");

    return results;
}

fn check_point_in_zone(point: GeoPoint) -> String
{
    let point: Point = point_mapper(point);

    return is_in_the_toll_zone(point).to_string();
}

fn point_mapper(point: GeoPoint) -> Point
{
    return Point(
        Coordinate {
            x: point.longitude,
            y: point.latitude
        }
    );
}

fn is_in_the_toll_zone(gps_data: Point) -> i32
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

fn get_polygon(geo_json_string: String) -> GeometryCollection
{
    //Example point in the zone: 19.943540573120117,50.0565299987793
    let geo_json: GeoJson = geo_json_string.parse::<GeoJson>().unwrap();

    let collection: GeometryCollection<f64> = quick_collection(&geo_json).unwrap();

    return collection;
}

fn buy_ticket(data: BuyTicket, request: JsonValue) -> String
{
    use point_in_polygon_dapp_paid_parking_assistant::schema::tickets::{self, *};
    let connection = establish_connection();

    let is_inserted = insert_into(tickets::table)
        .values((
            license.eq(data.license),
            longitude.eq(data.longitude),
            latitude.eq(data.latitude),
            owner_address.eq(&request["data"]["metadata"]["msg_sender"].to_string()),
            purchased_at.eq(Utc::now().to_string()),
            started_at.eq(data.started_at),
            duration.eq(data.duration),
            zone_id.eq(data.zone_id),
            status.eq(0)
        ))
        .execute(&connection)
        .unwrap();

    if is_inserted > 0 {
        let ticket = tickets::table
            .filter(owner_address.eq(&request["data"]["metadata"]["msg_sender"].to_string()))
            .order(id.desc())
            .first::<Ticket>(&connection)
            .expect("No ticket found");

        return serde_json::to_string(&ticket).unwrap();
    }

    return "Ticket error!".to_string();
}

fn get_ticket(data: GetTicket) -> String
{
    use point_in_polygon_dapp_paid_parking_assistant::schema::tickets::{self, *};
    let connection = establish_connection();

    let ticket = tickets::table
        .filter(license.eq(&data.license.to_string()))
        .order(id.desc())
        .first::<Ticket>(&connection);

    return match ticket {
        Ok(t) => {
            let ticket_date = t.started_at.parse::<DateTime<Utc>>().unwrap();
            let diff = (Utc::now() - ticket_date).num_minutes();

            if diff < t.duration.into() && diff > 0 {
                return serde_json::to_string(&t).unwrap();
            }

            return "There is no valid ticket available".to_string();
        },
        Err(val) => val.to_string(),
    };
}

fn my_tickets(request: JsonValue) -> String {
    use point_in_polygon_dapp_paid_parking_assistant::schema::tickets::dsl::*;
    let connection = establish_connection();

    let results = tickets
        .filter(owner_address.eq(&request["data"]["metadata"]["msg_sender"].to_string()))
        .load::<Ticket>(&connection)
        .expect("Error loading tickets");

    return serde_json::to_string(&results).unwrap();
}

async fn print_response<T: hyper::body::HttpBody>(
    response: hyper::Response<T>,
) -> Result<(), Box<dyn std::error::Error>>
    where
        <T as hyper::body::HttpBody>::Error: 'static,
        <T as hyper::body::HttpBody>::Error: std::error::Error,
{
    let response_status = response.status().as_u16();
    let response_body = hyper::body::to_bytes(response).await?;
    println!(
        "Received notice status {} body {}",
        response_status,
        std::str::from_utf8(&response_body)?
    );
    Ok(())
}

fn process_initial(metadata: &JsonValue) -> Option<String> {
    let epoch_index = metadata["epoch_index"].as_u64()?;
    let input_index = metadata["input_index"].as_u64()?;

    if epoch_index == 0 && input_index == 0 {
        let msg_sender = metadata["msg_sender"].as_str()?;
        println!("Captured rollup address: {}", msg_sender);
        return Some(msg_sender.to_string());
    }

    return None;
}
