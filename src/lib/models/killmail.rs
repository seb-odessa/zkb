use std::convert::From;
use std::convert::Into;
use crate::api;
use crate::schema::killmails;
use super::{Integer, DateTime};

#[derive(Queryable, Insertable)]
#[table_name = "killmails"]
pub struct KillMail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub solar_system_id: Integer,
    pub moon_id: Option<Integer>,
    pub war_id: Option<Integer>,
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

/*
impl Into<api::killmail::KillMail> for KillMail{
    fn into(self) -> api::killmail::KillMail {

        api::killmail::KillMail {
            killmail_id: self.killmail_id,
            killmail_time: self.killmail_time,
            solar_system_id: self.solar_system_id,
            moon_id: self.moon_id,
            war_id: self.war_id,
            victim:
        }

    pub victim: Victim,
    pub attackers: Vec<Attacker>,
    }
}
*/


