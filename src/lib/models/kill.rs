
use std::fmt;
use chrono::{DateTime, NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::kills;
use super::{Integer, Hash, Connection, QueryResult};


#[derive(Debug, Queryable, Insertable)]
#[table_name = "kills"]
pub struct Kill {
    pub killmail_id: Integer,
    pub killmail_hash: Hash,
    pub killmail_date: NaiveDate,
}
impl Kill {
    /** Constructor */
    pub fn new(id: &Integer, hash: &Hash, date: &NaiveDate) -> Self {        
        Self {
            killmail_id: *id,
            killmail_hash: hash.clone(),
            killmail_date: date.clone(),
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
// impl fmt::Debug for Kill {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ {}, {}, {} }}", 
//             self.killmail_id, 
//             hex::encode(&self.killmail_hash), 
//             self.killmail_date)
//     }
// }

