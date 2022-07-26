## BACKEND

### Running the backend in host mode (step by step)

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
make host
```

Docker Host environment down
```shell
make host-down
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

run the project
```shell
cargo run
```

Cargo-watch is helpful in the dev environment
```shell
cargo watch -c -w src -x run
```

After the back-end successfully starts, it should print an output like the following:
```log
Sending finish
Received finish status 200 OK
Captured rollup address: 0xRollupAddress
```

> #### Those steps are not relevant because we move migration to the main function (I'm leaving it here because it might be helpful)
> project is using SQLite Database, install Diesel CLI
> ```shell
> cargo install diesel_cli --no-default-features --features sqlite --version ^1.4
> ```
> all setup should be handled by
> ```shell
> diesel setup
> ```
> but as project will grow with new migrations, run
> ```shell
> diesel migration run
> ```

## Interacting with the application

We can use the frontend-console application to interact with the DApp for testing purposes.
```log
https://github.com/cartesi/rollups-examples/tree/main/frontend-console
```
this console should interact with the rollup by the advance state and inspect state

in the app is present Router that acting as proxy to properly maintain routing.

#### Main payload structure:
```json
{
  "endpoint": "example_endpoint_name",
  "payload": {
    "key": "value"
  }
}
```
inner payload is defined by structs and depends on the endpoint</br>
#### Main response structure:
```json
{
  "status": 200,
  "data": {},
  "error": null
}
```
app response is present in the data field as an object

as for now, available functionalities are:

### advance state
#### Buy Ticket (ABI encoded by Ether Portal):
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
Response (notice) - Ticket:
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
  "to_pay": "1000000000000000",
  "status": 1
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
        "amount": "1000000",
        "zone_id": 1
      }
    }
  }
}
```
Response (voucher):
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
        "geo_json": "Stringify GeoJSON",
        "address": "0xAddress"
      }
    }
  }
}
```
Response - Zone:
```json
{
  "id": 1,
  "name": "String",
  "price": "1000000",
  "geo_json": "Stringify GeoJSON",
  "owner_address": "0xString"
}
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
Response - Success:
```json
{
  "message": "Zone deleted!"
}
```

### inspect state (only hex encoding) - reports
#### Get Zones:
Request:
```json
{
  "endpoint": "get_zones",
  "payload": {
    "Zone": {
      "Get": {
        "zone_id": 1,
        "owner_address": "0xAddress",
        "paginate": {
          "per_page": 15,
          "page": 1
        }
      }
    }
  }
}
```
Response - Zone:
```json
[
  {
    "id": 1,
    "name": "String",
    "price": "1000000",
    "geo_json": "Stringify GeoJSON",
    "owner_address": "0xAddress"
  },
  {},
  {}
]
```

#### Check point in zones
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
Response - Zone:
```json
{
  "id": 1,
  "name": "String",
  "price": "1000000",
  "geo_json": "Stringify GeoJSON",
  "owner_address": "0xAddress"
}
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

Response - Ticket:
```json
[
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
    "to_pay": "1000000000000000",
    "status": 1
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
Response - Ticket, Zone:
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
  "to_pay": "1000000000000000",
  "status": 1,
  "zone": {
    "id": 1,
    "name": "String",
    "price": "1000000",
    "geo_json": "Stringify GeoJSON",
    "owner_address": "0xAddress"
  }
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
