extern crate diesel;
extern crate lib;

use diesel::prelude::*;
use lib::database::*;
use lib::models::DateRow;
use lib::schema::dates::dsl::*;

fn main() {
    let conn = establish_connection();
    let res = dates
        .limit(100)
        .load::<DateRow>(&conn)
        .expect("Error loading date");
    for date in res {
        println!("{:?}", date);
    }
}
