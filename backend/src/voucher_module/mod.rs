use crate::diesel::prelude::*;
use crate::establish_connection;
use crate::models::{Voucher};
use crate::structures::*;
use json::{object, JsonValue};
use std::error::Error;

pub fn get_vouchers(filters: GetVoucher) -> Result<JsonValue, Box<dyn Error>> {
    use crate::schema::vouchers::{self, *};
    let mut connection = establish_connection();
    
    let mut query = vouchers::table.into_boxed()
        .filter(requested_by.eq(filters.owner_address));
    
    let pagination: Pagination = match filters.paginate {
        Some(value) => value,
        None => Default::default(),
    };

    let offset = pagination.page - 1;
    query = query
        .limit(pagination.per_page)
        .offset(offset * pagination.per_page);

    return Ok(object! {
        "data": query.load::<Voucher>(&mut connection)?,
    });
}