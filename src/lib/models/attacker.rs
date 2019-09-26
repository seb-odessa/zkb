use std::convert::From;
use crate::api;
use crate::schema::attackers;
use super::{Integer, OptInteger, Float, Bool};

// CREATE TABLE IF NOT EXISTS attackers(
//     attacker_id INTEGER NOT NULL PRIMARY KEY,
//     killmail_id INTEGER NOT NULL,
//     security_status REAL NOT NULL,
//     final_blow BOOLEAN NOT NULL,
//     damage_done INTEGER NOT NULL,
//     ship_type_id INTEGER,
//     alliance_id INTEGER,
//     character_id INTEGER,
//     corporation_id INTEGER,
//     faction_id INTEGER,
//     weapon_type_id INTEGER,
//     FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
// );

#[derive(Queryable, Insertable, Default)]
#[table_name = "attackers"]
pub struct Attacker {
    pub killmail_id: Integer,
    pub security_status: Float,
    pub final_blow: Bool,
    pub damage_done: Integer,
    pub ship_type_id: OptInteger,
    pub alliance_id: OptInteger,
    pub character_id: OptInteger,
    pub corporation_id: OptInteger,
    pub faction_id: OptInteger,
    pub weapon_type_id: OptInteger,
}

impl From<&api::killmail::Attacker> for Attacker{
    fn from(src: &api::killmail::Attacker) -> Self {
        Self {
            killmail_id: 0,
            security_status: src.security_status,
            final_blow: src.final_blow,
            damage_done: src.damage_done,
            ship_type_id: src.ship_type_id,
            alliance_id: src.alliance_id,
            character_id: src.character_id,
            corporation_id: src.corporation_id,
            faction_id: src.faction_id,
            weapon_type_id: src.weapon_type_id,
        }
    }
}

impl Attacker{
    pub fn load(src: &api::killmail::KillMail) -> Vec<Self> {
        let mut res = Vec::new();
        for i in 0..src.attackers.len()
        {
            let mut attacker = Self::from(&src.attackers[i]);
            attacker.killmail_id = src.killmail_id;
            res.push(attacker);
        }
        return res;
    }
}




