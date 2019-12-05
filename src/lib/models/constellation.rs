use std::convert::From;
use crate::api;
use crate::schema::constellations;
use crate::schema::named_constellations;
use crate::schema::neighbors_constellations;
use super::{Connection, QueryResult, Integer, OptString};

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

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "neighbors_constellations"]
pub struct ConstellationNeighbors {
    pub own_id: Integer,
    pub own_name: OptString,
	pub neighbor_id: Integer,
	pub neighbor_name: OptString,
}
impl ConstellationNeighbors {

    pub fn get_name(&self, name: &str) -> String {
        match name {
            "own" => self.own_name.clone(),
            "neighbor" => self.neighbor_name.clone(),
            any => Some(format!("Unknown pattern {}", any))
        }.unwrap_or_default()
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        neighbors_constellations::table
            .filter(neighbors_constellations::own_id.eq(id))
            .order_by(neighbors_constellations::neighbor_name)
            .load(conn)
    }
}

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "named_constellations"]
pub struct ConstellationNamed {
    pub constellation_id: Integer,
    pub constellation_name: OptString,
	pub region_id: Integer,
	pub region_name: OptString,
}
impl ConstellationNamed {
    pub fn get_id(&self, name: &str) -> Integer {
        match name {
            "constellation" => self.constellation_id,
            "region" => self.region_id,
            _ => 0
        }
    }

    pub fn get_name(&self, name: &str) -> String {
        match name {
            "constellation" => self.constellation_name.clone(),
            "region" => self.region_name.clone(),
            any => Some(format!("Unknown pattern {}", any))
        }.unwrap_or_default()
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        named_constellations::table.find(id).first(conn)
    }
    pub fn load_from_region(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        named_constellations::table
            .filter(named_constellations::region_id.eq(id))
            .order_by(named_constellations::constellation_name)
            .load(conn)
    }
}
