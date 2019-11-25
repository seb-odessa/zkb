use std::convert::From;
use crate::api;
use crate::schema::victims;
use crate::schema::named_victims;
use super::{Integer, OptInteger, OptString, Connection, QueryResult};

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
impl From<&api::Killmail> for Victim{
    fn from(src: &api::Killmail) -> Self {
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

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "named_victims"]
pub struct VictimNamed {
    pub victim_id: Integer,
    pub killmail_id: Integer,
    pub damage_taken: Integer,
    pub ship_id: Integer,
    pub ship_name: OptString,
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
    pub fn get_id(&self, name: &str) -> Integer {
        match name {
            "victim" => Some(self.victim_id),
            "killmail" => Some(self.killmail_id),
            "ship" => Some(self.ship_id),
            "character" => self.character_id,
            "corporation" => self.corporation_id,
            "alliance" => self.alliance_id,
            "faction" => self.faction_id,
            any => { warn!("Unknown pattern {}", any); Some(0)}
        }.unwrap_or_default()
    }

    pub fn get_name(&self, name: &str) -> String {
        match name {
            "ship" => self.ship_name.clone(),
            "character" => self.character_name.clone(),
            "corporation" => self.corporation_name.clone(),
            "alliance" => self.alliance_name.clone(),
            "faction" => self.faction_name.clone(),
            any => Some(format!("Unknown pattern {}", any))
        }.unwrap_or_default()
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        named_victims::table.filter(named_victims::killmail_id.eq(id)).first(conn)
    }
}

