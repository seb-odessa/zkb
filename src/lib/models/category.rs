use crate::schema::categories;
use super::Integer;

#[derive(Queryable, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub category_id: Integer,
    pub category_name: String,
}
