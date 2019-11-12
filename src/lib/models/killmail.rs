use std::convert::From;
use chrono::{Duration, Utc};

use crate::api;
use crate::schema::killmails;
use crate::schema::named_killmails;
use super::{Integer, OptInteger, OptString, DateTime, Connection, QueryResult};


#[derive(Insertable)]
#[table_name = "killmails"]
pub struct Killmail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub solar_system_id: Integer,
    pub moon_id: Option<Integer>,
    pub war_id: Option<Integer>,
}
impl From<&api::Killmail> for Killmail {
    fn from(src: &api::Killmail) -> Self {
        Self {
            killmail_id: src.killmail_id,
            killmail_time: src.killmail_time.naive_utc(),
            solar_system_id: src.solar_system_id,
            moon_id: src.moon_id,
            war_id: src.war_id,
        }
    }
}

#[derive(Queryable, Associations, Debug)]
#[table_name = "named_killmails"]
pub struct KillmailNamed {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub system_id: Integer,
    pub system_name: OptString,
    pub constellation_id: OptInteger,
    pub constellation_name: OptString,
    pub region_id: OptInteger,
    pub region_name: OptString,
}
impl KillmailNamed {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        named_killmails::table.filter(named_killmails::killmail_id.eq(id)).first(conn)
    }

    pub fn load_ids_for_last_minutes(conn: &Connection, system_id: &Integer, minutes: &Integer) -> QueryResult<Vec<Integer>> {
        use diesel::prelude::*;

        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        info!("Load killmails after {}", &start);
        named_killmails::table
            .filter(named_killmails::killmail_time.gt(start))
            .filter(named_killmails::system_id.eq(system_id))
            .select(named_killmails::killmail_id)
            .order(named_killmails::killmail_time.desc())
            .load(conn)
    }
}