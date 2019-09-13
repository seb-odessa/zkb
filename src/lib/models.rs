use super::schema::dates;

pub type Integer = i32;

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