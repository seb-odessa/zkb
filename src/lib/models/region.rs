use crate::schema::neighbors_regions;
use super::{Connection, QueryResult, Integer, OptString};

#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "neighbors_regions"]
pub struct RegionNeighbors {
    pub own_id: Integer,
    pub own_name: OptString,
	pub neighbor_id: Integer,
	pub neighbor_name: OptString,
}
impl RegionNeighbors {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        neighbors_regions::table
            .filter(neighbors_regions::own_id.eq(id))
            .order_by(neighbors_regions::neighbor_name)
            .load(conn)
    }
}
