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
use hyper::StatusCode;
use json::{object, JsonValue};
use parking_dapp::router::{response_type_handler, router};
use parking_dapp::{get_db_env_var, set_db_env_var, structures::*};
use std::env;
use std::error::Error;
use diesel::prelude::*;

extern crate diesel;
extern crate parking_dapp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    parking_dapp::run_migration();

    let client = hyper::Client::new();
    let server_addr = env::var("ROLLUP_HTTP_SERVER_URL")?;

    let mut status = ResponseStatus::Accept.to_string();

    loop {
        println!("Sending finish");

        let response = object! {"status" => status.to_string()};

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
                set_db_env_var(ROLLUP_ADDRESS, address.to_lowercase());

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
                    ResponseStatus::Reject.to_string()
                }
            };
        }
    }
}

pub async fn handle_advance(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);

    let request_input = handle_input(request);

    let input = match abi_decoder(&request_input) {
        Ok(deposit) => deposit,
        Err(_) => request_input,
    };

    return Ok(payload_parser_handler(client, server_addr, input).await?);
}

pub async fn handle_inspect(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);

    let input = handle_input(request);

    return Ok(payload_parser_handler(client, server_addr, input).await?);
}

fn handle_input(request: JsonValue) -> StandardInput {
    return StandardInput {
        /*bytes32: None,*/
        address: Some(
            request["data"]["metadata"]["msg_sender"]
                .to_string()
                .to_lowercase(),
        ),
        uint256: None,
        bytes: hex_decoder(&request),
        request: request,
    };
}

pub async fn payload_parser_handler(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    input: StandardInput,
) -> Result<String, Box<dyn std::error::Error>> {
    return match payload_parser(&input.bytes) {
        Ok(route) => {
            let output = handle_output(route, input)?;

            Ok(add_response(client, server_addr, output).await?)
        }
        Err(err) => {
            let output_payload = object! {
                "status" => StatusCode::BAD_REQUEST.as_u16(),
                "data" => json::Null,
                "error" => err.to_string(),
            }
            .to_string();

            let formatted_output = format!("0x{}", hex::encode(output_payload));

            let output = object! {
                "payload" => formatted_output,
                "status" => ResponseStatus::Reject.to_string(),
                "response_type" => ResponseType::Report.as_str(),
            };

            Ok(add_response(client, server_addr, output).await?)
        }
    };
}

fn handle_output(route: Route, data: StandardInput) -> Result<JsonValue, Box<dyn Error>> {
    let mut status = ResponseStatus::Accept;

    let mut response_type = response_type_handler(&route);

    let output_payload: JsonValue = match router(route, &data) {
        Ok(data) => {
            if matches!(response_type, ResponseType::Voucher) {
                data["data"].clone()
            } else {
                object! {
                    "status" => StatusCode::OK.as_u16(),
                    "data" => data["data"].clone(),
                    "error" => json::Null,
                }
            }
        }
        Err(err) => {
            status = ResponseStatus::Reject;
            response_type = ResponseType::Report;

            object! {
                "status" => StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
                "data" => json::Null,
                "error" => err.to_string(),
            }
        }
    };

    let stringify_payload = output_payload.to_string();

    let formatted_output = format!("0x{}", hex::encode(stringify_payload));

    return Ok(object! {
        "address" => get_db_env_var(ROLLUP_ADDRESS),
        "requested_by" => data.address,
        "epoch_index" => data.request["data"]["metadata"]["epoch_index"].as_i32(),
        "input_index" => data.request["data"]["metadata"]["input_index"].as_i32(),
        "payload" => formatted_output,
        "status" => status.to_string(),
        "response_type" => response_type,
    });
}

pub async fn add_response(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    output: JsonValue,
) -> Result<String, Box<dyn Error>> {
    println!("Adding {}", output["response_type"].to_string());

    let body = object! {
        "address" => output["address"].clone(),
        "payload" => output["payload"].clone()
    };
    
    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!(
            "{}/{}",
            server_addr,
            output["response_type"].to_string()
        ))
        .body(hyper::Body::from(body.dump()))?;

    let response = client.request(req).await?;

    save_response(response, &output).await;

    Ok(output["status"].to_string())
}

fn abi_decoder(data: &StandardInput) -> Result<StandardInput, String> {
    let abi_parameters = get_abi_ether_parameters();

    return match decode(&abi_parameters, &data.bytes) {
        Ok(tokens) => {
            Ok(StandardInput {
                //bytes32: Some(tokens[0].clone()),
                address: Some(tokens[1].clone().to_string()),
                uint256: Some(tokens[2].clone()),
                bytes: tokens[3]
                    .clone()
                    .into_bytes()
                    .expect("If decoding is successful there should be a bytes token."),
                request: data.request.clone(),
            })
        }
        Err(msg) => Err(msg.to_string()),
    };
}

fn payload_parser(data: &Vec<u8>) -> Result<Route, Box<dyn Error>> {
    return Ok(serde_json::from_slice(&data)?);
}

fn hex_decoder(request: &JsonValue) -> Vec<u8> {
    let payload = prepare_payload(request);

    return hex::decode(&payload.as_str())
        .expect("Every payload from the rollup server is hex encoded.");
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

async fn save_response<T: hyper::body::HttpBody>(
    response: hyper::Response<T>,
    output: &JsonValue,
) -> () {
    println!("Received status: {};", response.status().as_u16());
    
    if ResponseType::Voucher.as_str() == output["response_type"] {
        if let Ok(bytes) = hyper::body::to_bytes(response).await {
            if let Ok(body) = serde_json::from_slice::<ResponseBody>(&bytes.to_vec().as_slice()) {
                if let (
                    Some(epoch_index_value),
                    Some(input_index_value),
                    Some(address_value)
                ) = (
                    output["epoch_index"].as_i32(),
                    output["input_index"].as_i32(),
                    output["requested_by"].as_str()
                ) {
                    use parking_dapp::schema::vouchers::{self, *};
                    let mut connection = parking_dapp::establish_connection();

                    diesel::RunQueryDsl::execute(diesel::insert_into(vouchers::table)
                        .values((
                            epoch_index.eq(epoch_index_value),
                            input_index.eq(input_index_value),
                            voucher_index.eq(body.index),
                            requested_by.eq(address_value),
                        )), &mut connection)
                        .ok();
                }
            }
        }
    }

    return ();
}
