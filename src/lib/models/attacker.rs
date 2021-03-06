use std::convert::From;
use crate::api;
use crate::schema::attackers;
use crate::schema::named_attackers;
use super::{Integer, OptInteger, OptString, Float, Bool, Connection, QueryResult};

#[derive(Insertable)]
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
impl From<&api::Attacker> for Attacker{
    fn from(src: &api::Attacker) -> Self {
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

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "named_attackers"]
pub struct AttackerNamed {
    pub attacker_id: Integer,
    pub killmail_id: Integer,
    pub security_status: Float,
    pub final_blow: Bool,
    pub damage_done: Integer,
    pub ship_id: OptInteger,
    pub ship_name: OptString,
    pub character_id: OptInteger,
    pub character_name: OptString,
    pub corporation_id: OptInteger,
    pub corporation_name: OptString,
    pub alliance_id: OptInteger,
    pub alliance_name: OptString,
    pub faction_id: OptInteger,
    pub faction_name: OptString,
    pub weapon_id: OptInteger,
    pub weapon_name: OptString,
}
impl AttackerNamed {

    pub fn get_id(&self, name: &str) -> i32 {
        match name {
            "ship" => self.ship_id.clone(),
            "character" => self.character_id.clone(),
            "corporation" => self.corporation_id.clone(),
            "alliance" => self.alliance_id.clone(),
            "faction" => self.faction_id.clone(),
            "weapon" => self.weapon_id.clone(),
            any => { warn!("Unknown pattern {}", any); Some(0) },
        }.unwrap_or_default()
    }


    pub fn get_name(&self, name: &str) -> String {
        match name {
            "ship" => self.ship_name.clone(),
            "character" => self.character_name.clone(),
            "corporation" => self.corporation_name.clone(),
            "alliance" => self.alliance_name.clone(),
            "faction" => self.faction_name.clone(),
            "weapon" => self.weapon_name.clone(),
            any => Some(format!("Unknown pattern {}", any))
        }.unwrap_or_default()
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        named_attackers::table.filter(named_attackers::killmail_id.eq(id)).load(conn)
    }
}