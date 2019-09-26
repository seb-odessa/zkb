use std::convert::From;
use crate::api;
use crate::schema::items;
use super::{Integer, OptInteger};

// CREATE TABLE IF NOT EXISTS items(
//     item_id INTEGER NOT NULL PRIMARY KEY,
//     killmail_id INTEGER NOT NULL,
//     item_type_id INTEGER NOT NULL,
//     singleton INTEGER NOT NULL,
//     flag INTEGER NOT NULL,
//     quantity_destroyed INTEGER,
//     quantity_dropped INTEGER,
//     FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
// );


#[derive(Queryable, Insertable, Default)]
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

impl Item {
    pub fn load(src: &api::killmail::KillMail) -> Vec<Self> {
        let mut res = Vec::new();
        if let Some(items) = &src.victim.items {
            for i in 0..items.len()
            {
                let mut item = Self::from(&items[i]);
                item.killmail_id = src.killmail_id;
                res.push(item);
                if let Some(subitems) = &items[i].items {
                    for i in 0..subitems.len()
                    {
                        let mut item = Self::from(&subitems[i]);
                        item.killmail_id = src.killmail_id;
                        res.push(item);
                    }
                }
            }
        }
        return res;
    }
}




