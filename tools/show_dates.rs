extern crate lib;
extern crate diesel;

use lib::models::*;
use lib::database::*;
use lib::schema::dates::dsl::*;
use diesel::prelude::*;


fn main() {
    let conn = establish_connection();
    let res = dates.limit(100).load::<Date>(&conn).expect("Error loading date");
    for date in res {
        println!("{:?}", date);
    }
}
