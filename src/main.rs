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
use geo::{Coordinate, Point, LineString, Polygon};
use geo::Contains;

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

fn point_mapper(mut _payload: String) -> Result<Point, String>
{
    //We don't need "0x" for conversion.
    _payload.remove(0);
    _payload.remove(0);

    //Handling conversion: hex string -> vec<u8> -> &str -> String
    let converted_string = match Vec::from_hex(&_payload) {
        Ok(vec) => {
            match std::str::from_utf8(&vec) {
                Ok(v) => v.to_string(),
                Err(_e) => panic!("Invalid UTF-8 sequence: {}", _e),
            }
        },
        Err(_e) => panic!("Invalid HEX sequence: {}", _e)
    };

    //On default data format is: "{latitude},{longitude}"
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

fn get_polygon() -> Polygon
{
    //Mock-up data - geojson in the future
    //Example point in the zone: 50.0565299987793,19.943540573120117
    return Polygon::new(
        LineString(
            vec![
                Coordinate {
                    x: 50.05842838065494,
                    y: 19.94080872302984
                },
                Coordinate {
                    x: 50.05642587449102,
                    y: 19.94541994324063
                },
                Coordinate {
                    x: 50.05432662964047,
                    y: 19.939593766165125
                },
            ]
        ),
        vec![]
    );
}

fn is_in_the_toll_zone(gps_data: Point) -> bool
{
    // TODO This is the mock-up. In the python app, there is geojson here it needs to be thought out.
    let polygon = get_polygon();

    return polygon.contains(&gps_data);
}

pub async fn handle_advance(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);

    //I created this function at first to convert GPGGA to a proper Point object but for now, we are using standard latitude and longitude.
    let _point: Point = match point_mapper(request["data"]["payload"].to_string()) {
        Ok(point) => point,
        Err(_e) => panic!("Invalid GPS data: {}", _e),
    };

    //I'm not sure if this representation of boolean true and false is a good idea but let's leave it here for now.
    let payload: &str = match is_in_the_toll_zone(_point) {
        true => "1",
        false => "0"
    };

    println!("Adding notice");
    //Quick string -> hex conversion with "0x" prefix
    let notice = object! {"payload" => format!("0x{}", hex::encode(payload))};
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
    let payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;
    println!("Adding report");
    let report = object! {"payload" => format!("{}", payload)};
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
