use crate::api;

pub mod kill;
pub mod killmail;
pub mod attacker;
pub mod victim;
pub mod item;

pub use diesel::sqlite::SqliteConnection as Connection;

pub type Bool = bool;
pub type Integer = i32;
pub type OptInteger = Option<Integer>;
pub type Float = f32;
pub type Hash = String;
pub type Date = chrono::NaiveDate;
pub type DateTime = chrono::NaiveDateTime;
pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;

pub struct DB;
impl DB {
    pub fn connection() -> Connection {
        use crate::diesel::Connection;
        let url = std::env::var("DATABASE_URL").expect("DB_URL environment variable required");
        Connection::establish(&url).expect(&format!("Error connection to {}", url))
    }

    /** Loads kills from DB by the date */
    pub fn load_kills(conn: &Connection, date: &Date) -> QueryResult<Vec<kill::Kill>> {
        use diesel::prelude::*;
        use super::schema::kills::dsl as table;
        table::kills.filter(table::killmail_date.eq(&date)).load(conn)
    }

    /** Saves kills into DB */
    pub fn insert_kills(conn: &Connection, kills: &Vec<kill::Kill>) -> QueryResult<usize> {
        use diesel::prelude::*;
        use super::schema;
        diesel::insert_into(schema::kills::table).values(kills).execute(conn)
    }

    /** Saves killmail into DB */
    pub fn save(conn: &Connection, src: &api::killmail::KillMail) -> QueryResult<()> {
        use super::schema;
        use diesel::connection::Connection;
        use diesel::RunQueryDsl;

        conn.transaction::<_, _, _>(|| {
            diesel::insert_into(schema::killmails::table)
                   .values(&killmail::KillMail::from(src))
                   .execute(conn)?;
            diesel::insert_into(schema::attackers::table)
                   .values(&attacker::Attacker::load(src))
                   .execute(conn)?;
            diesel::insert_into(schema::victims::table)
                   .values(&victim::Victim::from(src))
                   .execute(conn)?;
            let items = item::Item::load(src);
            diesel::insert_into(schema::items::table)
                   .values(&items)
                   .execute(conn)?;
            Ok(())
        })
    }



    // pub fn load(conn: &Connection, id: &Integer) -> QueryResult<api::killmail::KillMail> {
    //     let mut killmail;
    //     // use killmails::dsl as table;
    //     // table::killmails.find(*id)
    //     //                 .first::<KillMailHeader>(conn)
    //     //                 .and_then(|header| Ok(header.into()))
    //     Ok(killmail)
    // }
}