use super::models;
use super::schema;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection as Connection;
use models::Integer;
use models::Hash;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;

pub fn establish_connection() -> Connection {
    use crate::diesel::Connection;
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable required");
    Connection::establish(&url).expect(&format!("Error connection to {}", url))
}

pub fn insert_date(conn: &Connection, year: Integer, month: Integer, day: Integer) -> QueryResult<usize> {

    let new = models::NewDate {
        year: year,
        month: month,
        day: day,
    };
    diesel::insert_into(schema::dates::table).values(&new).execute(conn)
}

pub fn get_date(conn: &Connection, date_id: Integer) -> QueryResult<models::Date> {
    schema::dates::dsl::dates.find(date_id).first::<models::Date>(conn)
}

pub fn get_date_id(conn: &Connection, year: Integer, month: Integer, day: Integer) -> QueryResult<Integer> {
    use schema::dates::dsl as table; 
    table::dates
        .filter(table::year.eq(&year))
        .filter(table::month.eq(&month))
        .filter(table::day.eq(&day))
        .select(table::id)
        .first(conn)
}


pub fn insert_kill(conn: &Connection, id: Integer, hash: Hash, date_id: Integer) -> QueryResult<usize> {
    let new = models::NewKill {
        id: id,
        hash: hash,
        date_id: date_id,
    };
    diesel::insert_into(schema::kills::table).values(&new).execute(conn)
}
