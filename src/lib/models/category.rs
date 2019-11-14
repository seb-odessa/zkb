use crate::schema::categories;
use super::{Integer, Connection, QueryResult};

#[derive(Queryable, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub category_id: Integer,
    pub category_name: String,
}
impl Category {
    pub fn find(conn: &Connection, name: &String) -> QueryResult<Vec<Integer>> {
        use diesel::prelude::*;
        use crate::schema::categories::dsl as table;
        table::categories
            .filter(table::category_name.like(name))
            .select(table::category_id)
            .load(conn)
    }
}