extern crate serde;
extern crate serde_json;

use super::schema::dates;
use super::schema::kills;
use serde::{Deserialize, Serialize};

pub type Integer = i32;
pub type Hash = Vec<u8>;

#[derive(Debug, Queryable)]
pub struct DateRow {
    pub id: Integer,
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}

#[derive(Debug, Insertable)]
#[table_name = "dates"]
pub struct Date {
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}
impl Date {
    pub fn new(year: &Integer, month: &Integer, day: &Integer) -> Self {
        Self {
        year: *year,
        month: *month,
        day: *day,
        }
    }
}


#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "kills"]
pub struct Kill {
    pub id: Integer,
    pub hash: Hash,
    pub date_id: Integer,
}
impl Kill {
    pub fn new(id: &Integer, hash: &Hash, date_id: &Integer) -> Self {
        Self {
            id: *id,
            hash: hash.clone(),
            date_id: *date_id,
        }
    }
}


