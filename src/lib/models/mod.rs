pub mod date;
pub mod kill;

pub use diesel::sqlite::SqliteConnection as Connection;

pub type Integer = i32;
pub type Hash = Vec<u8>;
pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
