use std::convert::From;
use crate::api;
use crate::schema::constellations;
use super::{Connection, QueryResult, Integer};

#[derive(Insertable)]
#[table_name = "constellations"]
pub struct Constellation {
    pub constellation_id: Integer,
    pub region_id: Integer,
}

impl From<&api::constellation::Constellation> for Constellation{
    fn from(src: &api::constellation::Constellation) -> Self {
        Self {
            constellation_id: src.constellation_id,
            region_id: src.region_id,
        }
    }
}

impl Constellation {
    pub fn save(conn: &Connection, object: &api::constellation::Constellation) -> QueryResult<bool>  {
        use crate::schema;
        use crate::diesel::RunQueryDsl;
        diesel::insert_into(schema::constellations::table)
                   .values(Self::from(object))
                   .execute(conn).and_then(|count| Ok(1 == count))
    }

    pub fn exist(conn: &Connection, id: &Integer) -> bool {
        use diesel::prelude::*;
        use crate::schema::constellations::dsl as table;
        table::constellations.find(id).select(table::constellation_id).first(conn) == Ok(*id)
    }
}
