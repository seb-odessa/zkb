extern crate diesel;
extern crate lib;

use diesel::prelude::*;
use lib::database::*;
use lib::models::Kill;
use lib::schema::kills::dsl::*;

fn main() {
    let conn = establish_connection();
    let res = kills
        .limit(100)
        .load::<Kill>(&conn)
        .expect("Error loading date");
    for kill in res {
        println!("{:?}", kill);
    }
}
