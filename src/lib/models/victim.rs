use std::convert::From;
use crate::api;
use crate::schema::victims;
use crate::schema::named_victims;
use super::{Integer, OptInteger, OptString, Connection, QueryResult};

#[derive(Insertable, Associations)]
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

#[derive(Queryable, Associations)]
#[table_name = "named_victims"]
pub struct VictimNamed {
    pub victim_id: Integer,
    pub killmail_id: Integer,
    pub damage_taken: Integer,
    pub ship_id: Integer,
    pub ship_name: String,
    pub character_id: OptInteger,
    pub character_name: OptString,
    pub corporation_id: OptInteger,
    pub corporation_name: OptString,
    pub alliance_id: OptInteger,
    pub alliance_name: OptString,
    pub faction_id: OptInteger,
    pub faction_name: OptString,
}
impl VictimNamed {
    pub fn load(conn: &Connection, id: Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        named_victims::table.filter(named_victims::killmail_id.eq(&id)).first(conn)
    }
}

