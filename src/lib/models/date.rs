use crate::schema::dates;
use super::{Integer, Connection, QueryResult};
use diesel::prelude::*;

//=========================================
#[derive(Debug, Queryable)]
pub struct DateRow {
    pub id: Integer,
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}
impl DateRow {
    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<DateRow> {
        use crate::schema::dates::dsl as table;
        table::dates.find(id).first::<DateRow>(conn)
    }
}

//=========================================

#[derive(Debug, Insertable)]
#[table_name = "dates"]
pub struct Date {
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}
impl Date {
    /** Constructor */    
    pub fn new(year: &Integer, month: &Integer, day: &Integer) -> Self {
        Self {
            year: *year,
            month: *month,
            day: *day,
        }
    }

    /** Loads date Id from the DB */
    pub fn load_id(&self, conn: &Connection) -> QueryResult<Integer> {
        use crate::schema::dates::dsl as table;
        table::dates
            .filter(table::year.eq(&self.year))
            .filter(table::month.eq(&self.month))
            .filter(table::day.eq(&self.day))
            .select(table::id)
            .first(conn)
    }

    /** Saves date to the DB */
    pub fn save(&self, conn: &Connection) -> QueryResult<Integer> {
        diesel::insert_into(dates::table)
            .values(self)
            .execute(conn)
            .and_then(|_|{ self.load_id(conn) })
    }
}
