use std::convert::From;
use crate::api;
use crate::schema::attackers;
use super::{Integer, OptInteger, Float, Bool, Connection, QueryResult};

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
impl Attacker {
    pub fn load(conn: &Connection, id: Integer) -> QueryResult<Vec<api::killmail::Attacker>> {
        Loadable::load(conn, id).and_then(|rows| Ok(rows.into_iter().map(|a| Attacker::from(a).into()).collect()))
    }
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
impl Into<api::killmail::Attacker> for Attacker{
    fn into(self) -> api::killmail::Attacker {
        api::killmail::Attacker {
            ship_type_id: self.ship_type_id,
            character_id: self.character_id,
            corporation_id: self.corporation_id,
            alliance_id: self.alliance_id,
            faction_id: self.faction_id,
            damage_done: self.damage_done,
            final_blow: self.final_blow,
            security_status: self.security_status,
            weapon_type_id: self.weapon_type_id,
        }
    }
}
impl From<Loadable> for Attacker{
    fn from(src: Loadable) -> Self {
        Self {
            killmail_id: src.killmail_id,
            ship_type_id: src.ship_type_id,
            character_id: src.character_id,
            corporation_id: src.corporation_id,
            alliance_id: src.alliance_id,
            faction_id: src.faction_id,
            damage_done: src.damage_done,
            final_blow: src.final_blow,
            security_status: src.security_status,
            weapon_type_id: src.weapon_type_id,
        }
    }
}
#[derive(Queryable)]
struct Loadable {
    pub attacker_id: Integer,
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

impl Loadable {
    pub fn load(conn: &Connection, id: Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        use crate::schema::attackers::dsl as table;
        table::attackers.filter(table::killmail_id.eq(&id)).load(conn)
    }
}