use super::models::*;
use super::schema::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;


type DBResult<T> = std::result::Result<T, diesel::result::Error>;

pub fn establish_connection() -> SqliteConnection {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable required");
    SqliteConnection::establish(&url).expect(&format!("Error connection to {}", url))
}

pub fn insert_date(conn: &SqliteConnection, year: i32, month: i32, day: i32) -> DBResult<usize> {
    let new = NewDate{
        year: year,
        month: month,
        day: day
    };

    diesel::insert_into(dates::table)
        .values(&new)
        .execute(conn)
        //.expect(&format!("Failed to save date {}-{}-{}", year, month, day))

}

