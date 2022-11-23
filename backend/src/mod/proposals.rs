use crate::diesel::prelude::*;
use crate::establish_connection;
use crate::models::{Ticket, Zone};
use crate::structures::*;
use chrono::prelude::*;
use diesel::insert_into;
use geo::{Contains, Coordinate, Point};
use geo_types::GeometryCollection;
use geojson::{quick_collection, GeoJson};
use json::{object, JsonValue};
use std::error::Error;
use std::str::FromStr;


