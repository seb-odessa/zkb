extern crate serde;
extern crate serde_json;

use super::schema::dates;
use super::schema::kills;
use serde::{Deserialize, Serialize};

pub type Integer = i32;
pub type Hash = String;

#[derive(Debug, Queryable)]
pub struct Date {
    pub id: Integer,
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}

#[derive(Debug, Insertable)]
#[table_name = "dates"]
pub struct NewDate {
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}

#[derive(Debug, Queryable)]
pub struct Kill {
    pub id: Integer,
    pub hash: Hash,
    pub date_id: Integer,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "kills"]
pub struct NewKill {
    pub id: Integer,
    pub hash: Hash,
    pub date_id: Integer,
}
impl NewKill {
    pub fn new(id: &Integer, hash: &Hash, date_id: &Integer) -> Self {
        Self {
            id: *id,
            hash: hash.clone(),
            date_id: *date_id,
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
