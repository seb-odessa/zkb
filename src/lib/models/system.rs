use std::convert::From;
use crate::api;
use crate::schema::systems;
use super::{Connection, QueryResult, Integer, OptInteger, Float};

#[derive(Insertable)]
#[table_name = "systems"]
pub struct System {
    pub system_id: Integer,
    pub star_id: OptInteger,
    pub security_status: Float,
    pub constellation_id: Integer,
}

impl From<&api::system::System> for System {
    fn from(src: &api::system::System) -> Self {
        Self {
            system_id: src.system_id,
            star_id: src.star_id,
            security_status: src.security_status,
            constellation_id: src.constellation_id,
        }
    }
}
impl System {
    pub fn save(conn: &Connection, object: &api::system::System) -> QueryResult<bool>  {
        use crate::schema;
        use crate::diesel::RunQueryDsl;
        diesel::insert_into(schema::systems::table)
                   .values(Self::from(object))
                   .execute(conn).and_then(|count| Ok(1 == count))
    }

    pub fn exist(conn: &Connection, id: &Integer) -> bool {
        use diesel::prelude::*;
        use crate::schema::systems::dsl as table;
        table::systems.find(id).select(table::system_id).first(conn) == Ok(*id)
    }
}

