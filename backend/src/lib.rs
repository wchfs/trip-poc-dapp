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
use std::env;

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