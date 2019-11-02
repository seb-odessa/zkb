use crate::schema::objects;
use super::Integer;

#[derive(Queryable, Insertable, Associations)]
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
}
