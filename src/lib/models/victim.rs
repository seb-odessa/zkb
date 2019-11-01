use std::convert::From;
use std::convert::Into;
use crate::api;
use crate::schema::victims;
use super::{Integer, OptInteger, Connection, QueryResult};

#[derive(Insertable)]
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
impl Victim {
    pub fn load(conn: &Connection, id: Integer) -> QueryResult<api::killmail::Victim> {
        Loadable::load(conn, id).and_then(|row| Ok(Victim::from(row).into()))
    }
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
impl Into<api::killmail::Victim> for Victim{
    fn into(self) -> api::killmail::Victim {
        api::killmail::Victim {
            ship_type_id: self.ship_type_id,
            damage_taken: self.damage_taken,
            character_id: self.character_id,
            corporation_id: self.corporation_id,
            alliance_id: self.alliance_id,
            faction_id: self.faction_id,
            items: None,
            position: None,
        }
    }
}
impl From<Loadable> for Victim{
    fn from(src: Loadable) -> Self {
        Self {
            killmail_id: src.killmail_id,
            ship_type_id: src.ship_type_id,
            damage_taken: src.damage_taken,
            alliance_id: src.alliance_id,
            character_id: src.character_id,
            corporation_id: src.corporation_id,
            faction_id: src.faction_id,
        }
    }
}


#[derive(Queryable)]
struct Loadable {
    pub victim_id: Integer,
    pub killmail_id: Integer,
    pub ship_type_id: Integer,
    pub damage_taken: Integer,
    pub alliance_id: OptInteger,
    pub character_id: OptInteger,
    pub corporation_id: OptInteger,
    pub faction_id: OptInteger,
}

impl Loadable {
    pub fn load(conn: &Connection, id: Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        use crate::schema::victims::dsl as table;
        table::victims.filter(table::killmail_id.eq(&id)).first(conn)
    }
}

