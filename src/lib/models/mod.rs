pub mod kill;
pub mod killdata;

pub use diesel::sqlite::SqliteConnection as Connection;

pub type Integer = i32;
pub type Hash = String;
pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
