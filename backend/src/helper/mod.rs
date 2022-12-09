use crate::diesel::prelude::*;
use crate::establish_connection;
use crate::structures::*;
use chrono::LocalResult;
use chrono::prelude::*;
use std::error::Error;

pub fn request_timestamp(data: &StandardInput) -> Result<String, Box<dyn Error>> {
    let parsed_request_timestamp = parse_request_timestamp(data)?;

    let rfc = parsed_request_timestamp.to_rfc3339_opts(SecondsFormat::Millis, true);

    return Ok(rfc);
}

fn parse_request_timestamp(data: &StandardInput) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let timestamp = match data.request["data"]["metadata"]["timestamp"]
        .clone()
        .as_i64()
    {
        Some(it) => it,
        None => {
            return Err(Box::new(ErrorOutput {
                error: "Missin timestamp".into(),
            }))
        }
    };

    let timezone: LocalResult<DateTime<Utc>> = Utc.timestamp_opt(timestamp, 0);

    match timezone.single() {
        Some(datetime) => return Ok(datetime),
        _ => Err(Box::new(ErrorOutput {
            error: "Unsuccessful timestamp conversion".into(),
        })),
    }
}

pub fn super_wallet_validator(sender_address: String) -> Result<bool, Box<dyn Error>> {
    use crate::schema::super_wallets::dsl::*;
    let mut connection = establish_connection();

    let result: i64 = super_wallets
        .filter(address.eq(&sender_address.to_lowercase()))
        .count()
        .get_result(&mut connection)?;

    return if result > 0 {
        Ok(true)
    } else {
        Err(Box::new(ErrorOutput {
            error: format!("You don't have permission to perform this action: {}", sender_address.to_lowercase()),
        }))
    };
}

pub fn parse_request_addres(data: &StandardInput) -> Result<String, Box<dyn Error>> {
    let wallet = match data.address.clone() {
        Some(it) => it.to_lowercase(),
        None => {
            return Err(Box::new(ErrorOutput {
                error: "Missing address".into(),
            }))
        }
    }
    .to_string();

    return Ok(format!("{}", wallet));
}

pub fn error_json_string(data: String) -> String {
    return serde_json::to_string(&ErrorOutput { error: data })
        .expect("Static typing ensures that there will not be any errors.");
}

pub fn success_json_string(data: String) -> String {
    return serde_json::to_string(&SuccessOutput { message: data })
        .expect("Static typing ensures that there will not be any errors.");
}
