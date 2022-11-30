#[macro_use] extern crate diesel;
extern crate diesel_migrations;
extern crate dotenv;

pub mod core;
pub mod models;
pub mod router;
pub mod schema;
pub mod structures;

use diesel::prelude::*;
use dotenv::dotenv;
use crate::models::{EnvVar};
use std::{env};

use crate::diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migration() {
    let mut connection = establish_connection();

    connection.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn set_db_env_var(key: &str, value: String) -> bool {
    use crate::schema::{*};
    let mut connection = establish_connection();

    let result: Result<i64, diesel::result::Error> = env_vars::table
        .filter(env_vars::var_name.eq(key))
        .count()
        .get_result(&mut connection);

    return if let Ok(count) = result {
        if count > 0 {
            return true;
        }

        match diesel::insert_into(env_vars::table)
                    .values((env_vars::var_name.eq(key), env_vars::var_value.eq(value)))
                    .execute(&mut connection) {
            Ok(_) => return true,
            Err(_) => return false,
        };

    } else {
        false
    };
}

pub fn get_db_env_var(key: &str) -> Option<EnvVar> {
    use crate::schema::{*};
    let mut connection = establish_connection();

    return match env_vars::table
            .filter(env_vars::var_name.eq(key))
            .first::<EnvVar>(&mut connection) {
        Ok(it) => Some(it),
        Err(_) => None,
    };
}
