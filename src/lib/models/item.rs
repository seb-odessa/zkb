use std::convert::From;
use crate::api;
use crate::schema::items;
use super::{Integer, OptInteger, Connection, QueryResult};


#[derive(Insertable, Default)]
#[table_name = "items"]
pub struct Item {
    pub killmail_id: Integer,
    pub item_type_id: Integer,
    pub singleton: Integer,
    pub flag: Integer,
    pub quantity_destroyed: OptInteger,
    pub quantity_dropped: OptInteger,
}

impl From<&api::killmail::Item> for Item{
    fn from(src: &api::killmail::Item) -> Self {
        Self {
            killmail_id: 0,
            item_type_id: src.item_type_id,
            singleton: src.singleton,
            flag: src.flag,
            quantity_destroyed: src.quantity_destroyed,
            quantity_dropped: src.quantity_dropped,
        }
    }
}
impl From<&Loadable> for Item{
    fn from(src: &Loadable) -> Self {
        Self {
            killmail_id: src.killmail_id,
            item_type_id: src.item_type_id,
            singleton: src.singleton,
            flag: src.flag,
            quantity_destroyed: src.quantity_destroyed,
            quantity_dropped: src.quantity_dropped,
        }
    }
}


#[derive(Queryable)]
struct Loadable {
    pub item_id: Integer,
    pub killmail_id: Integer,
    pub item_type_id: Integer,
    pub singleton: Integer,
    pub flag: Integer,
    pub quantity_destroyed: OptInteger,
    pub quantity_dropped: OptInteger,
}

impl Loadable {
    pub fn load(conn: &Connection, killmail_id: Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        use crate::schema::items::dsl as table;
        table::items.filter(table::killmail_id.eq(&killmail_id)).load(conn)
    }
}




