use super::models;
use super::schema;
use models::{Date, Kill};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection as Connection;
use models::Integer;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;

pub fn establish_connection() -> Connection {
    use crate::diesel::Connection;
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable required");
    Connection::establish(&url).expect(&format!("Error connection to {}", url))
}

pub fn insert_date(conn: &Connection, date: &Date) -> QueryResult<Integer> {
    diesel::insert_into(schema::dates::table)
        .values(date)
        .execute(conn)
        .and_then(|_|{ get_date_id(conn, date) })
}

pub fn get_date(conn: &Connection, id: Integer) -> QueryResult<models::DateRow> {
    schema::dates::dsl::dates.find(id).first::<models::DateRow>(conn)
}

pub fn get_date_id(conn: &Connection, date: &Date) -> QueryResult<Integer> {
    use schema::dates::dsl as table;
    table::dates
        .filter(table::year.eq(&date.year))
        .filter(table::month.eq(&date.month))
        .filter(table::day.eq(&date.day))
        .select(table::id)
        .first(conn)
}

pub fn insert_kill(conn: &Connection, kill: &Kill) -> QueryResult<usize> {
    diesel::insert_into(schema::kills::table).values(kill).execute(conn)
}

pub fn insert_kills(conn: &Connection, kills: &Vec<Kill>) -> QueryResult<usize> {
    diesel::insert_into(schema::kills::table)
            .values(kills)
//            .on_conflict(schema::kills::columns::id).do_nothing()
            .execute(conn)


}
