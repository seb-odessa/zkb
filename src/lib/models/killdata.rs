use std::convert::{From, Into};
use diesel::prelude::*;
use crate::killmail::KillMail;
use crate::schema::killmails;
use super::{Integer, Connection, QueryResult};

#[derive(Queryable, Insertable)]
#[table_name = "killmails"]
pub struct KillMailHeader {
    pub killmail_id: Integer,
    pub killmail_time: String,
    pub solar_system_id: Integer,
    pub moon_id: Option<Integer>,
    pub war_id: Option<Integer>,
    pub victim_id: Integer,
    pub attackers_id: Integer,
}
impl From<&KillMail> for KillMailHeader{
    fn from(src: &KillMail) -> Self {
        Self {
            killmail_id: src.killmail_id,
            killmail_time: src.killmail_time.clone(),
            solar_system_id: src.solar_system_id,
            moon_id: src.moon_id,
            war_id: src.war_id,
            victim_id: 0,
            attackers_id: 0,
        }
    }
}
impl Into<KillMail> for KillMailHeader {
    fn into(self)-> KillMail {
        KillMail {
            killmail_id: self.killmail_id,
            killmail_time: self.killmail_time.clone(),
            solar_system_id: self.solar_system_id,
            moon_id: self.moon_id,
            war_id: self.war_id,
            victim: None,
            attackers: Vec::new(),
        }
    }
}

struct Gate;
impl Gate {    
    /** Saves killmail into DB */
    pub fn save(&self, conn: &Connection, src: &KillMail) -> QueryResult<usize> {
        diesel::insert_into(killmails::table)
            .values(&KillMailHeader::from(src))
            // .on_conflict_do_nothing() on diesel 2.0
            .execute(conn)
    }
    /** Loads killmail from DB */
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<KillMail> {
        use killmails::dsl as table;

        table::killmails.find(*id)
                        .first::<KillMailHeader>(conn)
                        .and_then(|header| Ok(header.into()))
    }
}

