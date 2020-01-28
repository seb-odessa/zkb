use std::convert::From;
use crate::api;
use crate::schema::items;
use crate::schema::named_items;
use super::{Integer, OptInteger, OptString, Connection, QueryResult};

#[derive(Insertable)]
#[table_name = "items"]
pub struct Item {
    pub killmail_id: Integer,
    pub item_type_id: Integer,
    pub singleton: Integer,
    pub flag: Integer,
    pub quantity_destroyed: OptInteger,
    pub quantity_dropped: OptInteger,
}

impl From<&api::Item> for Item{
    fn from(src: &api::Item) -> Self {
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


#[derive(Queryable, Associations, Debug, PartialEq)]
#[table_name = "named_items"]
pub struct ItemNamed {
    pub item_id: Integer,
    pub killmail_id: Integer,
    pub item_type_id: Integer,
    pub item_type_name: OptString,
    pub singleton: Integer,
    pub flag: Integer,
    pub quantity_destroyed: OptInteger,
    pub quantity_dropped: OptInteger,
}
impl ItemNamed {

    pub fn get_id(&self) -> Integer {
        self.item_type_id
    }
    pub fn get_name(&self) -> String {
        self.item_type_name.clone().unwrap_or_default()
    }

    pub fn get_destroyed(&self) -> u64 {
        self.quantity_destroyed.clone().unwrap_or_default() as u64
    }

    pub fn get_dropped(&self) -> u64 {
        self.quantity_dropped.clone().unwrap_or_default() as u64
    }

    pub fn load(conn: &Connection, id: &Integer) -> QueryResult<Vec<Self>> {
        use diesel::prelude::*;
        named_items::table.filter(named_items::killmail_id.eq(id)).load(conn)
    }
}

