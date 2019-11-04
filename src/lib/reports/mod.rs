use crate::models::*;

use killmail::KillmailNamed;
use attacker::AttackerNamed;
use victim::VictimNamed;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Killmail {
    pub killmail_id: Integer,
    pub killmail_time: DateTime,
    pub system_id: Integer,
    pub system_name: OptString,
    pub moon_id: OptInteger,
    pub moon_name: OptString,
    pub war_id: OptInteger,
    pub victim: VictimNamed,
    pub attackers: Vec<AttackerNamed>,
}
impl Killmail {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        let killmail = KillmailNamed::load(conn, id)?;
        Ok(Self {
            killmail_id: killmail.killmail_id,
            killmail_time: killmail.killmail_time,
            system_id: killmail.system_id,
            system_name: killmail.system_name,
            moon_id: killmail.moon_id,
            moon_name: killmail.moon_name,
            war_id: killmail.war_id,
            victim: VictimNamed::load(conn, id)?,
            attackers: AttackerNamed::load(conn, id)?,
        })
    }
}

impl fmt::Display for Killmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.killmail_id, self.killmail_time.to_string())
    }
}