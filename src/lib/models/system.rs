use std::convert::From;
use crate::api;
use crate::schema::systems;
use crate::schema::named_systems;
use crate::schema::neighbors_systems;
use super::{Connection, QueryResult, Integer, OptInteger, OptString, Float};

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

#[derive(Debug, PartialEq)]
pub enum QuerySystem {
    Any,
    WithJovianObservatoryOnly
}


#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "neighbors_systems"]
pub struct SystemNeighbors {
    pub own_id: Integer,
    pub own_name: OptString,
	pub neighbor_id: Integer,
	pub neighbor_name: OptString,
}
impl SystemNeighbors {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        neighbors_systems::table
            .filter(neighbors_systems::own_id.eq(id))
            .order_by(neighbors_systems::neighbor_name)
            .load(conn)
    }

    pub fn get_own_name(&self) -> String {
        self.own_name.clone().unwrap_or_default()
    }

    pub fn get_neighbor_name(&self) -> String {
        self.neighbor_name.clone().unwrap_or_default()
    }

}

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "named_systems"]
pub struct SystemNamed {
    pub system_id: Integer,
    pub system_name: OptString,
    pub constellation_id: Integer,
    pub constellation_name: OptString,
    pub region_id: Integer,
    pub region_name: OptString,
    pub security_status: Float,
    pub observatory: OptString,
}
impl SystemNamed {

    pub fn get_system_name(&self) -> String {
        self.system_name.clone().unwrap_or_default()
    }

    pub fn get_constellation_name(&self) -> String {
        self.constellation_name.clone().unwrap_or_default()
    }

    pub fn get_region_name(&self) -> String {
        self.region_name.clone().unwrap_or_default()
    }

    pub fn has_observatory(&self) -> bool {
        self.observatory.is_some()
    }


    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        info!("Load system {}", &id);
        named_systems::table.find(id).first(conn)
    }

    pub fn load_from_constellation(conn: &Connection, id: &Integer, query: QuerySystem) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        info!("Load systems from constellation {}", &id);
        match query {
            QuerySystem::Any =>
                named_systems::table
                    .filter(named_systems::constellation_id.eq(id))
                    .load(conn),
            QuerySystem::WithJovianObservatoryOnly =>
                named_systems::table
                    .filter(named_systems::constellation_id.eq(id))
                    .filter(named_systems::observatory.is_not_null())
                    .load(conn),
        }
    }

    pub fn load_from_region(conn: &Connection, id: &Integer, query: QuerySystem) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        info!("Load systems from region {}", &id);
        match query {
            QuerySystem::Any =>
                named_systems::table
                    .filter(named_systems::region_id.eq(id))
                    .load(conn),
            QuerySystem::WithJovianObservatoryOnly =>
                named_systems::table
                    .filter(named_systems::region_id.eq(id))
                    .filter(named_systems::observatory.is_not_null())
                    .load(conn),
        }
    }
}