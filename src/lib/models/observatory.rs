use crate::schema::observatories;
use super::{Integer, Connection, QueryResult};

#[derive(Queryable, Insertable)]
#[table_name = "observatories"]
pub struct Observatory {
    pub system_id: Integer,
}
impl Observatory {

    pub fn new(id: &Integer) -> Self {
        Self{system_id: *id}
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        observatories::table.find(id).first(conn)
    }

    pub fn save(conn: &Connection, id: &Integer) -> QueryResult<usize>  {
        use diesel::prelude::*;
        diesel::insert_into(observatories::table).values(Self::new(id)).execute(conn)
    }

    pub fn delete(conn: &Connection, id: &Integer) -> QueryResult<usize>  {
        use diesel::prelude::*;
        diesel::delete(observatories::table.find(id)).execute(conn)
    }
}