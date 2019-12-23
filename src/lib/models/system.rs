use std::convert::From;
use crate::api;
use crate::schema::systems;
use crate::schema::named_systems;
use crate::schema::neighbors_systems;
use crate::schema::observatory_path;
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SystemFilter {
    Any,
    WithJovianObservatoryOnly,
    NullSec,
    LowSec,
    HiSec
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
        neighbors_systems::table
            .filter(neighbors_systems::own_id.eq(id))
            .order_by(neighbors_systems::neighbor_name)
            .load(conn)
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

    pub fn get_id(&self, name: &str) -> Integer {
        match name {
            "system" => self.system_id,
            "constellation" => self.constellation_id,
            "region" => self.region_id,
            _ => 0
        }
    }

    pub fn get_name(&self, name: &str) -> String {
        match name {
            "system" => self.system_name.clone(),
            "constellation" => self.constellation_name.clone(),
            "region" => self.region_name.clone(),
            any => Some(format!("Unknown pattern {}", any))
        }.unwrap_or_default()
    }

    pub fn has_observatory(&self) -> bool {
        self.observatory.is_some()
    }


    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        info!("Load system {}", &id);
        named_systems::table.find(id).first(conn)
    }

    pub fn load_from_constellation(conn: &Connection, id: &Integer, query: &SystemFilter) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        info!("Load systems from constellation {}", &id);
        match query {
            SystemFilter::Any =>
                named_systems::table
                    .filter(named_systems::constellation_id.eq(id))
                    .load(conn),
            SystemFilter::WithJovianObservatoryOnly =>
                named_systems::table
                    .filter(named_systems::constellation_id.eq(id))
                    .filter(named_systems::observatory.is_not_null())
                    .load(conn),
            SystemFilter::NullSec =>
                named_systems::table
                    .filter(named_systems::constellation_id.eq(id))
                    .filter(named_systems::security_status.lt(0.0))
                    .load(conn),
            SystemFilter::LowSec =>
                named_systems::table
                    .filter(named_systems::constellation_id.eq(id))
                    .filter(named_systems::security_status.gt(0.0).and(named_systems::security_status.lt(0.5)))
                    .load(conn),
            SystemFilter::HiSec =>
                named_systems::table
                    .filter(named_systems::constellation_id.eq(id))
                    .filter(named_systems::security_status.gt(0.5))
                    .load(conn),

        }
    }

    pub fn load_from_region(conn: &Connection, id: &Integer, query: &SystemFilter) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        info!("Load systems from region {}", &id);
        match query {
            SystemFilter::Any =>
                named_systems::table
                    .filter(named_systems::region_id.eq(id))
                    .load(conn),
            SystemFilter::WithJovianObservatoryOnly =>
                named_systems::table
                    .filter(named_systems::region_id.eq(id))
                    .filter(named_systems::observatory.is_not_null())
                    .load(conn),
            SystemFilter::NullSec =>
                named_systems::table
                    .filter(named_systems::region_id.eq(id))
                    .filter(named_systems::security_status.le(0.0))
                    .load(conn),
            SystemFilter::LowSec =>
                named_systems::table
                    .filter(named_systems::region_id.eq(id))
                    .filter(named_systems::security_status.gt(0.0).and(named_systems::security_status.lt(0.5)))
                    .load(conn),
            SystemFilter::HiSec =>
                named_systems::table
                    .filter(named_systems::region_id.eq(id))
                    .filter(named_systems::security_status.ge(0.5))
                    .load(conn),

        }
    }
}

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "observatory_path"]
pub struct ObservatoryPath {
    pub s0_id: Integer,
    pub s0_name: OptString,
    pub s1_id: Integer,
    pub s1_name: OptString,
    pub s1_jo: bool,
    pub s2_id: Integer,
    pub s2_name: OptString,
    pub s2_jo: bool,
    pub s3_id: Integer,
    pub s3_name: OptString,
    pub s3_jo: bool,
    pub s4_id: Integer,
    pub s4_name: OptString,
    pub s4_jo: bool,
    pub s5_id: Integer,
    pub s5_name: OptString,
    pub s5_jo: bool,
}
impl ObservatoryPath {
        pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        info!("Load system {}", &id);
        observatory_path::table
            .filter(observatory_path::s0_id.eq(id))
            .load(conn)
    }
}