
use chrono::NaiveDate;
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

    /** Saves object into DB */
    pub fn save(&self, conn: &Connection) -> QueryResult<usize> {
        use diesel::prelude::*;

        diesel::insert_into(crate::schema::kills::table)
            .values(self)
            // .on_conflict_do_nothing() on diesel 2.0
            .execute(conn)
    }

    /** Loads object from DB by killmail_id */
    pub fn load(conn: &Connection, killmail_id: Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        use crate::schema::kills::dsl as table;
        table::kills.find(killmail_id).first::<Self>(conn)
    }
}

