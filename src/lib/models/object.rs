use crate::schema::objects;
use super::{Integer, Connection, QueryResult};

#[derive(Queryable, Insertable, Associations, Debug, PartialEq)]
#[table_name = "objects"]
pub struct Object {
    pub object_id: Integer,
    pub category_id: Integer,
    pub object_name: String,
}
impl Object {
    pub fn new(object_id: Integer, category_id: Integer, object_name: String) -> Self {
        Self {
            object_id: object_id,
            category_id: category_id,
            object_name: object_name
        }
    }

    pub fn find(conn: &Connection, category_id: &Integer, name: &String) -> QueryResult<Vec<Integer>> {
        use diesel::prelude::*;
        use crate::schema::objects::dsl as table;
        table::objects
            .filter(table::category_id.eq(category_id).and(table::object_name.like(name)))
            .select(table::object_id)
            .load(conn)
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Self> {
        use diesel::prelude::*;
        use crate::schema::objects::dsl as table;
        table::objects.find(id).first(conn)
    }
}
