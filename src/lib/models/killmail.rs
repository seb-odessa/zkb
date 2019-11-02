use std::convert::From;
use crate::api;
use crate::schema::killmails;
use crate::schema::named_killmails;
use super::{Integer, OptInteger, OptString, DateTime, Connection, QueryResult};

#[derive(Insertable, Associations)]
#[table_name = "killmails"]
pub struct Killmail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub solar_system_id: Integer,
    pub moon_id: Option<Integer>,
    pub war_id: Option<Integer>,
}
impl From<&api::killmail::Killmail> for Killmail {
    fn from(src: &api::killmail::Killmail) -> Self {
        Self {
            killmail_id: src.killmail_id,
            killmail_time: src.killmail_time.naive_utc(),
            solar_system_id: src.solar_system_id,
            moon_id: src.moon_id,
            war_id: src.war_id,
        }
    }
}

#[derive(Queryable, Associations)]
#[table_name = "named_killmails"]
pub struct KillmailNamed {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub system_id: Integer,
    pub system_name: OptString,
    pub moon_id: OptInteger,
    pub war_id: OptInteger,
}
impl KillmailNamed {
    pub fn load(conn: &Connection, id: Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        named_killmails::table.filter(named_killmails::killmail_id.eq(&id)).first(conn)
    }
}