use std::convert::From;
use std::convert::Into;
use crate::api;
use crate::schema::objects;
use super::Integer;

#[derive(Queryable, Insertable)]
#[table_name = "objects"]
pub struct Object {
    pub object_id: Integer,
    pub category_name: String,
    pub object_name: String,
}

type ApiType = api::object::Object;

impl From<&ApiType> for Object{
    fn from(src: &ApiType) -> Self {
        Self {
            object_id: src.id,
            category_name: src.category.clone(),
            object_name: src.name.clone(),
        }
    }
}
impl Into<ApiType> for Object{
    fn into(self) -> ApiType {
        ApiType {
            id: self.object_id,
            category: self.category_name,
            name: self.object_name,
        }
    }
}


