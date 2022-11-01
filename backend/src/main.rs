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

use dotenv::dotenv;
use ethabi::{decode, ParamType};
use json::{object, JsonValue};
use parking_dapp::router::{response_type_handler, router};
use parking_dapp::structures::*;
use std::env;

extern crate diesel;
extern crate parking_dapp;

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

    let request_input = handle_input(request);

    let input = match abi_decoder(&request_input) {
        Ok(deposit) => deposit,
        Err(_) => request_input,
    };

    let route: Route = payload_parser(&input.bytes);

    let response_type = response_type_handler(&route);

    let output = handle_output(route, input);

    return Ok(add_response(response_type, client, server_addr, output).await?);
}

pub async fn handle_inspect(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);

    let input = handle_input(request);

    let route: Route = payload_parser(&input.bytes);

    let response_type = response_type_handler(&route);

    let output = handle_output(route, input);
    
    return Ok(add_response(response_type, client, server_addr, output).await?);
}

pub async fn add_response(
    response_type: &str,
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    output: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Adding {}", response_type);

    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!("{}/{}", server_addr, response_type))
        .body(hyper::Body::from(output.dump()))?;

    let response = client.request(req).await?;

    print_response(response).await?;

    Ok("accept")
}

fn handle_input(request: JsonValue) -> StandardInput {
    return StandardInput {
        /*bytes32: None,*/
        address: Some(request["data"]["metadata"]["msg_sender"].to_string()),
        uint256: None,
        bytes: hex_decoder(&request),
        request: request,
    };
}

fn handle_output(route: Route, data: StandardInput) -> JsonValue {
    let address = env::var("ROLLUP_ADDRESS").unwrap();

    let output_payload: String = router(route, &data);

    let is_error = is_error(&output_payload);
    
    let formatted_output = format!("0x{}", hex::encode(output_payload));

    return object! {"address" => address, "payload" => formatted_output, "error" => is_error};
}

fn abi_decoder(data: &StandardInput) -> Result<StandardInput, String> {
    let abi_parameters = get_abi_ether_parameters();

    return match decode(&abi_parameters, &data.bytes) {
        Ok(tokens) => {
            Ok(StandardInput {
                //bytes32: Some(tokens[0].clone()),
                address: Some(tokens[1].clone().to_string()),
                uint256: Some(tokens[2].clone()),
                bytes: tokens[3].clone().into_bytes().unwrap(),
                request: data.request.clone(),
            })
        }
        Err(msg) => Err(msg.to_string()),
    };
}

fn payload_parser(data: &Vec<u8>) -> Route {
    return serde_json::from_slice(&data).unwrap();
}

fn hex_decoder(request: &JsonValue) -> Vec<u8> {
    let payload = prepare_payload(request);

    return hex::decode(&payload.as_str()).unwrap();
}

fn prepare_payload(request: &JsonValue) -> String {
    let mut payload = request["data"]["payload"].to_string();

    //We don't need "0x" for conversion.
    payload.remove(0);
    payload.remove(0);

    return payload;
}

fn get_abi_ether_parameters() -> [ParamType; 4] {
    return [
        ParamType::FixedBytes(32),
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::Bytes,
    ];
}

fn is_error(response_string: &String) -> bool {
    if let Ok(ErrorOutput {..}) = serde_json::from_str(response_string) {
        return true;
    }
    return false;
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
