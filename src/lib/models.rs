use super::schema::dates;
use std::str::FromStr;
use std::num::ParseIntError;

pub type Integer = i32;

#[derive(Debug, Queryable, PartialEq)]
pub struct Hash{
    pub hash: [u8, 16];
    // 5a c1 d9 c6 f0 
    // 74 79 98 89 df 
    // 18 1b 21 b9 89
    // c6 fc 6d 6b f1
}

#[derive(Debug, Queryable)]
pub struct Date {
    pub id: Integer,
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}

#[derive(Debug, Insertable)]
#[table_name="dates"]
pub struct NewDate {
    pub year: Integer,
    pub month: Integer,
    pub day: Integer,
}

#[derive(Debug, Queryable)]
pub struct Kill {
    pub id: Integer,
    pub hash: String,
    pub date_id: Integer,
}


#[cfg(test)]
mod tests {

    #[test]
    fn test() {
    }
}