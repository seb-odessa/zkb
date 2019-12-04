use crate::schema::neighbors_regions;
use crate::schema::named_systems;
use super::{Connection, QueryResult, Integer, OptString};


#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "named_systems"]
pub struct RegionNamed {
    pub region_id: Integer,
    pub region_name: OptString,
}
impl RegionNamed {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        named_systems::table
            .filter(named_systems::region_id.eq(id))
            .select((named_systems::region_id, named_systems::region_name))
            .distinct()
            .first(conn)
    }

    pub fn get_id(&self) -> Integer {
        self.region_id
    }

    pub fn get_name(&self) -> String {
        self.region_name.clone().unwrap_or_default()
    }
}


#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "neighbors_regions"]
pub struct RegionNeighbors {
    pub own_id: Integer,
    pub own_name: OptString,
	pub neighbor_id: Integer,
	pub neighbor_name: OptString,
}
impl RegionNeighbors {
    pub fn get_id(&self, name: &str) -> Integer {
        match name {
            "own" => self.own_id,
            "neighbor" => self.neighbor_id,
            _ => 0
        }
    }

    pub fn get_name(&self, name: &str) -> String {
        match name {
            "own" => self.own_name.clone(),
            "neighbor" => self.neighbor_name.clone(),
            any => Some(format!("Unknown pattern {}", any))
        }.unwrap_or_default()
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        neighbors_regions::table
            .filter(neighbors_regions::own_id.eq(id))
            .order_by(neighbors_regions::neighbor_name)
            .load(conn)
    }
}
