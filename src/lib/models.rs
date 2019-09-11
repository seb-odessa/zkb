use super::schema::dates;

use diesel::sqlite::SqliteConnection;


#[derive(Debug, Queryable)]
pub struct Date {
    pub id: i64,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Insertable)]
#[table_name="dates"]
pub struct NewDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}


#[derive(Debug, Queryable)]
pub struct Kill {
    pub id: i32,
    pub hash: String,
    pub date_id: i32,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::export::Result;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_id() {
    }
}