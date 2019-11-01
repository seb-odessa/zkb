use std::convert::From;
use std::convert::Into;
use chrono::Utc;

use crate::api;
use crate::schema::killmails;
use super::{Integer, DateTime, Connection, QueryResult};
use super::victim::Victim;
use super::attacker::Attacker;

#[derive(Queryable, Insertable)]
#[table_name = "killmails"]
pub struct KillMail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub solar_system_id: Integer,
    pub moon_id: Option<Integer>,
    pub war_id: Option<Integer>,
}
impl KillMail {
    pub fn load(conn: &Connection, id: Integer) -> QueryResult<api::killmail::KillMail> {
        use diesel::prelude::*;
        use crate::schema::killmails::dsl as table;
        let killmail: KillMail = table::killmails.find(&id).first::<Self>(conn)?;

        Ok(api::killmail::KillMail {
            killmail_id: killmail.killmail_id,
            killmail_time: chrono::DateTime::from_utc(killmail.killmail_time, Utc),
            solar_system_id: killmail.solar_system_id,
            moon_id: killmail.moon_id,
            war_id: killmail.war_id,
            victim: Victim::load(conn, id)?,
            attackers: Attacker::load(conn, id)?,
        })
    }
}
impl From<&api::killmail::KillMail> for KillMail{
    fn from(src: &api::killmail::KillMail) -> Self {
        Self {
            killmail_id: src.killmail_id,
            killmail_time: src.killmail_time.naive_utc(),
            solar_system_id: src.solar_system_id,
            moon_id: src.moon_id,
            war_id: src.war_id,
        }
    }
}


