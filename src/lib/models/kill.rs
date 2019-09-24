
use std::fmt;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::kills;
use super::{Integer, Hash, Connection, QueryResult};


#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "kills"]
pub struct Kill {
    pub id: Integer,
    pub hash: Hash,
    pub date_id: Integer,
}
impl Kill {
    /** Constructor */
    pub fn new(id: &Integer, hash: &Hash, date_id: &Integer) -> Self {
        Self {
            id: *id,
            hash: hash.clone(),
            date_id: *date_id,
        }
    }

    /** Saves current kill into DB */
    pub fn save(&self, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(crate::schema::kills::table)
            .values(self)
            // .on_conflict_do_nothing() on diesel 2.0
            .execute(conn)
    }

    /** Loads kill by id */
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use crate::schema::kills::dsl as table;
        table::kills.find(id).first::<Self>(conn)
    }
}
impl fmt::Debug for Kill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {}, {} }}", self.id, hex::encode(&self.hash), self.date_id)
    }
}

