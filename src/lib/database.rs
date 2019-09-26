use super::models;
use super::schema;
use models::kill::Kill;
use diesel::prelude::*;

use diesel::sqlite::SqliteConnection as Connection;
use models::Integer;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;

pub fn establish_connection() -> Connection {
    use crate::diesel::Connection;
    let url = std::env::var("DB_URL").expect("DB_URL environment variable required");
    Connection::establish(&url).expect(&format!("Error connection to {}", url))
}

pub fn insert_kills(conn: &Connection, kills: &Vec<Kill>) -> QueryResult<usize> {
    diesel::insert_into(schema::kills::table)
            .values(kills)
            // .on_conflict_do_nothing() on diesel 2.0
            .execute(conn)
}

pub fn get_kills(conn: &Connection, date_id: Integer) -> QueryResult<Vec<Kill>> {
    use schema::kills::dsl as table;
    table::kills
//        .filter(table::date_id.eq(&date_id))
        .load(conn)
}