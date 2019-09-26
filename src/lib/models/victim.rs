use std::convert::From;
use crate::api;
use crate::schema::victims;
use super::{Integer, OptInteger};

// CREATE TABLE IF NOT EXISTS victims(
//     victim_id INTEGER NOT NULL PRIMARY KEY,
//     killmail_id INTEGER NOT NULL,
//     ship_type_id INTEGER NOT NULL,
//     damage_taken INTEGER NOT NULL,
//     alliance_id	INTEGER,
//     character_id	INTEGER,
//     corporation_id	INTEGER,
//     faction_id	INTEGER,
//     FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
// );

#[derive(Queryable, Insertable)]
#[table_name = "victims"]
pub struct Victim {
    pub killmail_id: Integer,
    pub ship_type_id: Integer,
    pub damage_taken: Integer,
    pub alliance_id: OptInteger,
    pub character_id: OptInteger,
    pub corporation_id: OptInteger,
    pub faction_id: OptInteger,
}
impl From<&api::killmail::KillMail> for Victim{
    fn from(src: &api::killmail::KillMail) -> Self {
        Self {
            killmail_id: src.killmail_id,
            ship_type_id: src.victim.ship_type_id,
            damage_taken: src.victim.damage_taken,
            alliance_id: src.victim.alliance_id,
            character_id: src.victim.character_id,
            corporation_id: src.victim.corporation_id,
            faction_id: src.victim.faction_id,
        }
    }
}



