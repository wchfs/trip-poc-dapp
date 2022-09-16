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
use hex::FromHex;
use geo::{Coordinate, Point};
use geo::Contains;
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use serde::Deserialize;
use ethabi::{ParamType, decode};

use point_in_polygon_dapp_paid_parking_assistant::establish_connection;
use point_in_polygon_dapp_paid_parking_assistant::models::Zone;

extern crate point_in_polygon_dapp_paid_parking_assistant;
extern crate diesel;

use self::diesel::prelude::*;

#[derive(Deserialize, Debug)]
struct Request {
    endpoint: String,
    payload: String,
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

fn point_mapper(converted_string: String) -> Result<Point, String>
{
    //Default data format is: "{Longitude},{Latitude}"
    let split = converted_string.split(",").collect::<Vec<&str>>();

    //Check if we have an exact number of data.
    if split.len() != 2 {
        return Err(split.len().to_string());
    }

    return Ok(
        Point(
            Coordinate {
                x: split[0].parse::<f64>().unwrap(),
                y: split[1].parse::<f64>().unwrap()
            }
        )
    );
}

fn get_polygon(geo_json_string: String) -> GeometryCollection
{
    //Example point in the zone: 19.943540573120117,50.0565299987793
    let geo_json: GeoJson = geo_json_string.parse::<GeoJson>().unwrap();

    let collection: GeometryCollection<f64> = quick_collection(&geo_json).unwrap();

    return collection;
}

fn is_in_the_toll_zone(gps_data: Point) -> bool
{
    let zones = get_all_zones();
    for zone in zones {
        let polygon = get_polygon(zone.geo_json);

        if polygon.contains(&gps_data) {
            return true;
        }
    }

    return false;
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

fn get_zones() -> String {
    let zones = get_all_zones();

    return serde_json::to_string(&zones).unwrap();
}

fn check_point_in_zone(value: String) -> String
{
    //I created this function at first to convert GPGGA to a proper Point object but for now, we are using standard latitude and longitude.
    let _point: Point = match point_mapper(value.to_string()) {
        Ok(point) => point,
        Err(_e) => panic!("Invalid GPS data: {}", _e),
    };

    //I'm not sure if this representation of boolean true and false is a good idea but let's leave it here for now.
    let payload: &str = match is_in_the_toll_zone(_point) {
        true => "1",
        false => "0"
    };

    return payload.to_string();
}

fn decode_payload(request: &JsonValue) -> Request
{
    let mut payload = request["data"]["payload"]
        .to_string();

    //We don't need "0x" for conversion.
    payload.remove(0);
    payload.remove(0);

    //Handling conversion: hex string -> vec<u8> -> &str
    let converted_string = match Vec::from_hex(&payload) {
        Ok(vec) => {
            match std::str::from_utf8(&vec) {
                Ok(v) => v.to_string(),
                Err(_e) => panic!("Invalid UTF-8 sequence: {}", _e),
            }
        },
        Err(_e) => panic!("Invalid HEX sequence: {}", _e)
    };

    return serde_json::from_str(converted_string.as_str()).unwrap();
}

fn abi_decode_payload(request: &JsonValue) -> String
{
    let mut payload = request["data"]["payload"].to_string();

    payload.remove(0);
    payload.remove(0);

    let data: Vec<u8> = hex::decode(&payload.as_str()).unwrap();

    let tokens = decode(
        &[
            ParamType::FixedBytes(32),
            ParamType::Address,
            ParamType::Uint(256),
            ParamType::Bytes,
        ],
        &data).unwrap();
    // (['bytes32', 'address', 'uint256', 'bytes'])

    let deposit_payload = tokens[3].clone().to_string();

    let initially_decoded_payload = hex::decode(deposit_payload).unwrap();

    let mut stringify_payload = std::str::from_utf8(&initially_decoded_payload).unwrap().to_string();

    stringify_payload.remove(0);
    stringify_payload.remove(0);

    return stringify_payload;
}

fn router(request: Request) -> String
{
    return match request.endpoint.as_str() {
        "get_zones" => get_zones(),
        "check_point_in_zones" => check_point_in_zone(request.payload),
        &_ => todo!(),
    };
}

pub async fn handle_advance(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);

    let data = abi_decode_payload(&request);

    println!("{:#?}", data);

    println!("Adding notice");
    //Quick string -> hex conversion with "0x" prefix
    let notice = object! {"payload" => format!("0x{}", hex::encode(data))};
    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!("{}/notice", server_addr))
        .body(hyper::Body::from(notice.dump()))?;
    let response = client.request(req).await?;
    print_response(response).await?;
    Ok("accept")
}

pub async fn handle_inspect(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);

    println!("Adding report");

    let data = decode_payload(&request);

    let payload = router(data);

    let report = object! {"payload" => format!("0x{}", hex::encode(payload))};

    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!("{}/report", server_addr))
        .body(hyper::Body::from(report.dump()))?;

    let response = client.request(req).await?;

    print_response(response).await?;

    Ok("accept")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
