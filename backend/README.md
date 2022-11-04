## BACK-END

### Running the back-end in host mode (step by step)

Install Rust (based on):
```shell
https://www.rust-lang.org/learn/get-started
```

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Environment is running in docker

Docker Host environment up
```shell
docker compose -f ./docker-compose.yml -f ./docker-compose.override.yml -f ./docker-compose-host.yml up
```

Docker Host environment down
```shell
docker compose -f ./docker-compose.yml -f ./docker-compose.override.yml -f ./docker-compose-host.yml down -v
```

ensure that all environment variables are filled properly</br>
rollup address need to be retrieved from docker instance</br>
check
```
.env.example
```

update might be useful
```shell
rustup update
```

build the project
```shell
cargo build
```

project is using SQLite Database, install Diesel CLI
```shell
cargo install diesel_cli --no-default-features --features sqlite --version ^1.4
```

all setup should be handled by
```shell
diesel setup
```

run the project
```shell
cargo run
```

but as project will grow with new migrations, run
```shell
diesel migration run
```

After the back-end successfully starts, it should print an output like the following:
```log
Sending finish
```

## Interacting with the application

We can use the frontend-console application to interact with the DApp for testing purposes.
```log
https://github.com/cartesi/rollups-examples/tree/main/frontend-console
```
this console should interact with the rollup by the advance state and inspect state

in the app is present Router that acting as proxy to properly maintain routing.

Main payload structure:
```json
{
  "endpoint": "example_endpoint_name",
  "payload": {
    "key": "value"
  }
}
```

inner payload is defined by structs and depends on the endpoint</br>
as for now, available functionalities are:

### advance state (ABI encoded by Ether Portal)
#### Buy Ticket:
Request:
```json
{
  "endpoint": "buy_ticket",
  "payload": {
    "Ticket": {
      "Buy": {
        "license": "String",
        "longitude": 19.943540573120117,
        "latitude": 50.0565299987793,
        "started_at": "2022-09-24T14:25:00Z",
        "duration": 60,
        "zone_id": 1
      }
    }
  }
}
```
Response (notice):
```json
{
  "id": 1,
  "license": "String",
  "longitude": 19.943540573120117,
  "latitude": 50.0565299987793,
  "started_at": "2022-09-24T14:25:00Z",
  "owner_address": "0xString",
  "purchased_at": "2022-09-24T14:04:20Z",
  "duration": 60,
  "zone_id": 1,
  "paid": "1000000000000000",
  "to_pay": "1000000000000000"
}
```
If the calculated price is different from the paid amount.
```json
{
  "error": "Ticket validation error!"
}
```

#### For withdraw funds from the app
Request:
```json
{
  "endpoint": "withdraw_funds",
  "payload": {
    "Balance": {
      "Withdraw": {
        "amount": "1000000"
      }
    }
  }
}
```
Response:
```json
{
  "address": "0xrollup_address",
  "voucher": "encoded_ABI"
}
```

#### For seed the zone in the app
Request:
```json
{
  "endpoint": "seed_zone",
  "payload": {
    "Seed": {
      "Zone": {
        "name": "Zone A",
        "price": "1000000",
        "geo_json": "Stringify GeoJSON"
      }
    }
  }
}
```
Response:
```json
[
  {
    "id": 1,
    "name": "String",
    "price": "1000000",
    "geo_json": "Stringify GeoJSON",
    "owner_address": "0xString"
  },
  {},
  {}
]
```

#### To remove the zone in the app
Request:
```json
{
  "endpoint": "remove_zone",
  "payload": {
    "Remove": {
      "id": 1
    }
  }
}
```
Response:
```json
[
  {
    "success": 1
  },
  {
    "error": 0
  }
]
```

### inspect state (only hex encoding) - reports
#### Get (5) Zones:
Request:
```json
{
  "endpoint": "get_zones",
  "payload": null
}
```
Response:
```json
[
  {
    "id": 1,
    "name": "String",
    "price": "1000000",
    "geo_json": "Stringify GeoJSON",
    "owner_address": "0xString"
  },
  {},
  {}
]
```

#### Check point in (5) zones
Request:
```json
{
  "endpoint": "check_point_in_zones",
  "payload": {
    "Point": {
      "longitude": 19.943540573120117,
      "latitude": 50.0565299987793
    }
  }
}
```
Response:
```
zone_id: 
"0" - means the point is not found in any zone
"1" - means the point is in the zone with id = 1
"2" - -||- id = 2 etc.
```

#### Get tickets by license plate or owner address
Request:
```json
{
  "endpoint": "get_tickets",
  "payload": {
    "Ticket": {
      "Get": {
        "license": "example_license_plate",
        "owner_address": "0xString"
      }
    }
  }
}
```
both fields are optional -> with both fields missing there will be an error
```json
{
  "error": "Missing license and owner address!"
}
```

Response:
```json
[
  {
    "id": 1,
    "license": "example_license_plate",
    "longitude": 19.943540573120117,
    "latitude": 50.0565299987793,
    "started_at": "2022-09-24T14:25:00Z",
    "owner_address": "0xString",
    "purchased_at": "2022-09-24T14:04:20Z",
    "duration": 60,
    "zone_id": 1,
    "paid": "1000000000000000",
    "to_pay": "1000000000000000"
  },
  {},
  {}
]
```

#### For ticket validation
Request:
```json
{
  "endpoint": "validate_ticket",
  "payload": {
    "Ticket": {
      "Validate": {
        "license": "example_license_plate",
        "date": "2022-11-01T16:33:00Z"
      }
    }
  }
}
```
Response:
```json
{
    "id": 1,
    "license": "example_license_plate",
    "longitude": 19.943540573120117,
    "latitude": 50.0565299987793,
    "started_at": "2022-09-24T14:25:00Z",
    "owner_address": "0xString",
    "purchased_at": "2022-09-24T14:04:20Z",
    "duration": 60,
    "zone_id": 1,
    "paid": "1000000000000000",
    "to_pay": "1000000000000000"
}
```
If there is no valid ticket there will be a proper error.
```json
{
  "error": "There is no valid ticket available"
}
```

#### Get app balance
Request:
```json
{
  "endpoint": "get_app_balance",
  "payload": {
    "Balance": {
      "Get": {
        "zone_id": 1
      }
    }
  }
}
```
Response:
```
3000000
```
