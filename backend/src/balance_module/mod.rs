use crate::diesel::prelude::*;
use crate::{establish_connection, helper, zone_module};
use crate::models::{Zone};
use crate::structures::*;
use json::{object, JsonValue};
use std::error::Error;
use std::str::FromStr;

pub fn get_app_balance(data: &GetBalance) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::balances::{self, *};
    let mut connection = establish_connection();

    return Ok(object! {
        "data": balances::table
            .select(amount)
            .filter(zone_id.eq(data.zone_id))
            .first::<String>(&mut connection)?
    });
}

pub fn get_current_amount(zone_id: &i32) -> Result<i128, Box<dyn Error>> {
    return Ok(get_app_balance(&GetBalance {
        zone_id: (*zone_id),
    })?["data"]
        .to_string()
        .parse::<i128>()?);
}

pub fn add_funds_to_balance(value: String, requested_zone_id: &i32) -> Result<(), Box<dyn Error>> {
    let current_amount = get_current_amount(requested_zone_id)?;

    let parsed_value = value.parse::<i128>()?;

    let new_amount = (current_amount + parsed_value).to_string();

    update_balance(new_amount, requested_zone_id)?;

    return Ok(());
}

pub fn remove_funds_from_balance(
    value: String,
    requested_zone_id: &i32,
) -> Result<(), Box<dyn Error>> {
    let current_amount = get_current_amount(requested_zone_id)?;

    let parsed_value = value.parse::<i128>()?;

    let new_amount = (current_amount - parsed_value).to_string();

    update_balance(new_amount, requested_zone_id)?;

    return Ok(());
}

pub fn update_balance(new_amount: String, requested_zone_id: &i32) -> Result<(), Box<dyn Error>> {
    use crate::schema::balances::{self, *};
    let mut connection = establish_connection();

    diesel::update(balances::table)
        .filter(zone_id.eq(requested_zone_id))
        .set(amount.eq(new_amount))
        .execute(&mut connection)?;

    return Ok(());
}

pub fn create_zone_balance(zone: &Zone) -> Result<(), Box<dyn Error>> {
    use crate::schema::balances::{self, *};
    let mut connection = establish_connection();

    diesel::insert_into(balances::table)
        .values((zone_id.eq(zone.id), amount.eq("0")))
        .execute(&mut connection)?;

    return Ok(());
}

pub fn remove_zone_balance(zone: &Remover) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::balances::{self, *};
    let mut connection = establish_connection();

    let result =
        diesel::delete(balances::table.filter(zone_id.eq(zone.id))).execute(&mut connection);

    match result {
        Ok(value) => {
            if value.eq(&0) {
                return Err(Box::new(ErrorOutput {
                    error: "Balance not deleted!".to_string(),
                }));
            }
            return Ok(object! {
                "data": helper::success_json_string("Balance deleted!".to_string())
            });
        }
        Err(value) => {
            println!("{:?}", value);
            return Err(Box::new(ErrorOutput {
                error: value.to_string(),
            }));
        }
    }
}

pub fn withdraw_funds(
    withdraw_struct: WithdrawFunds,
    additional_data: &StandardInput,
) -> Result<JsonValue, Box<dyn Error>> {
    let app_balance = &get_current_amount(&withdraw_struct.zone_id)?;

    let parsed_amount = &withdraw_struct.amount.parse::<i128>()?;

    if parsed_amount > app_balance {
        return Err(Box::new(ErrorOutput {
            error: "Insufficient funds to withdraw".into(),
        }));
    }

    let owner_address = match additional_data.address.as_ref() {
        Some(value) => value,
        None => {
            return Err(Box::new(ErrorOutput {
                error: "No Owner address provided".into(),
            }))
        }
    };

    if zone_module::check_zone_owner(&withdraw_struct.zone_id, owner_address) {
        return Err(Box::new(ErrorOutput {
            error: "Only owner can withdraw funds".into(),
        }));
    }
    
    let parsed_address = ethabi::ethereum_types::Address::from_str(owner_address.as_str())?;

    let address_token = ethabi::Token::Address(parsed_address);

    let uint_parsed_token = ethabi::ethereum_types::U256::from_str(&withdraw_struct.amount)?;

    let amount_token = ethabi::Token::Uint(uint_parsed_token);

    let withdrawal_data = ethabi::encode(&[address_token, amount_token]);

    let abi_bytes = ethabi::Token::Bytes(withdrawal_data);

    let mut data = ethabi::encode(&[abi_bytes]);

    let mut payload = ETHER_WITHDRAWAL_HEADER.as_slice().to_vec();
    payload.append(&mut data);

    let encoded_payload = hex::encode(payload);

    remove_funds_from_balance(withdraw_struct.amount, &withdraw_struct.zone_id)?;

    return Ok(object! {
        "data": format!("0x{}", encoded_payload)
    });
}