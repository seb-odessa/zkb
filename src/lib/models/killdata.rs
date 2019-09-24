use diesel::prelude::*;
use crate::killmail::KillMail;
use crate::schema::killmails as schema;
use super::{Integer, Connection, QueryResult};


#[derive(Queryable, Insertable)]
#[table_name = "schema"]
pub struct KillDetails {
    pub killmail_id: Integer,
    pub killmail_time: String,
    pub solar_system_id: Integer,
    pub moon_id: Option<Integer>,
    pub war_id: Option<Integer>,
    pub victim_id: Integer,
    pub attackers_id: Integer,
}
impl KillDetails {
    pub fn new(src: &KillMail) -> Self {
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
    
    /** Saves current killmail into DB */
    pub fn save(&self, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(schema::table)
            .values(self)
            // .on_conflict_do_nothing() on diesel 2.0
            .execute(conn)
    }
}

