use super::models::*;
use super::schema::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;

pub fn establish_connection() -> SqliteConnection {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable required");
    SqliteConnection::establish(&url).expect(&format!("Error connection to {}", url))
}

pub fn insert_date(conn: &SqliteConnection, year: i32, month: i32, day: i32) -> QueryResult<usize> {
    let new = NewDate {
        year: year,
        month: month,
        day: day,
    };

    diesel::insert_into(dates::table).values(&new).execute(conn)
}

pub fn insert_kill(
    conn: &SqliteConnection,
    kill_id: i32,
    kill_hash: String,
    date_id: i32,
) -> QueryResult<usize> {
    let new = NewKill {
        id: kill_id,
        hash: kill_hash,
        date_id: date_id,
    };

    diesel::insert_into(kills::table).values(&new).execute(conn)
}
