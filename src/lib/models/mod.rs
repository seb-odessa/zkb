use crate::api;
use std::collections::HashSet;


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
    pub fn save_kills(conn: &Connection, kills: &Vec<kill::Kill>) -> QueryResult<usize> {
        use diesel::prelude::*;
        use super::schema;
        diesel::insert_into(schema::kills::table).values(kills).execute(conn)
    }

    pub fn get_saved_killmails(conn: &Connection, date: &Date) -> HashSet<Integer> {
        let start =date.and_hms(0, 0, 0);
        let end =date.and_hms(23, 59, 59);

        use diesel::prelude::*;
        use std::iter::FromIterator;
        use super::schema::killmails::dsl as table;
        let vector = table::killmails
                    .filter(table::killmail_time.between(&start, &end))
                    .select(table::killmail_id).load(conn).unwrap();
        HashSet::from_iter(vector.iter().cloned())
    }

    pub fn exists(conn: &Connection, killmail_id: Integer) -> bool {
        use diesel::prelude::*;
        use super::schema::killmails::dsl as table;
        table::killmails.find(killmail_id).select(table::killmail_id).first(conn) == Ok(killmail_id)
    }

    /** Saves killmail into DB */
    pub fn save(conn: &Connection, killmail: &api::killmail::KillMail) -> QueryResult<()> {
        use super::schema;
        use diesel::connection::Connection;
        use diesel::RunQueryDsl;

        conn.transaction::<_, _, _>(|| {
            diesel::insert_into(schema::killmails::table)
                   .values(&killmail::KillMail::from(killmail))
                   .execute(conn)?;
            diesel::insert_into(schema::victims::table)
                   .values(&victim::Victim::from(killmail))
                   .execute(conn)?;
            diesel::insert_into(schema::attackers::table)
                   .values(&get_attackers(killmail))
                   .execute(conn)?;
            diesel::insert_into(schema::items::table)
                   .values(&get_items(killmail))
                   .execute(conn)?;
            Ok(())
        })
    }

    /** Saves vector of killmail into DB */
    pub fn save_all(conn: &Connection, killmail: &Vec<api::killmail::KillMail>) -> QueryResult<()> {
        use super::schema;
        use diesel::connection::Connection;
        use diesel::RunQueryDsl;

        let mut killmails = Vec::new();
        let mut victims = Vec::new();
        let mut attackers = Vec::new();
        let mut items = Vec::new();

        for killmail in killmail.iter() {
            victims.push(victim::Victim::from(killmail));
            killmails.push(killmail::KillMail::from(killmail));
            attackers.append(&mut get_attackers(killmail));
            items.append(&mut get_items(killmail));
        }

        conn.transaction::<_, _, _>(|| {
            diesel::insert_into(schema::killmails::table).values(&killmails).execute(conn)?;
            diesel::insert_into(schema::attackers::table).values(&attackers).execute(conn)?;
            diesel::insert_into(schema::victims::table).values(&victims).execute(conn)?;
            diesel::insert_into(schema::items::table).values(&items).execute(conn)?;
            Ok(())
        })
    }

}

fn get_attackers(killmail: &api::killmail::KillMail) -> Vec<attacker::Attacker> {
    let mut result = Vec::new();
    for attacker in &killmail.attackers {
        let mut obj = attacker::Attacker::from(attacker);
        obj.killmail_id = killmail.killmail_id;
        result.push(obj);
    }
    return result;
}

fn get_items(killmail: &api::killmail::KillMail) -> Vec<item::Item> {
    let mut result = Vec::new();
    if let Some(ref items) = &killmail.victim.items {
        for item in items {
            let mut obj = item::Item::from(item);
            obj.killmail_id = killmail.killmail_id;
            result.push(obj);
            if let Some(ref subitems) = item.items {
                for item in subitems {
                    let mut obj = item::Item::from(item);
                    obj.killmail_id = killmail.killmail_id;
                    result.push(obj);
                }
            }
        }
    }
    return result;
}
