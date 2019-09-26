use std::convert::From;
use crate::api;
use crate::schema::killmails;
use super::{Integer, DateTime};

// CREATE TABLE IF NOT EXISTS killmails(
//     killmail_id INTEGER NOT NULL PRIMARY KEY,
//     killmail_time DATETIME NOT NULL,
//     solar_system_id INTEGER NOT NULL,
//     moon_id INTEGER,
//     war_id INTEGER
// );

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




