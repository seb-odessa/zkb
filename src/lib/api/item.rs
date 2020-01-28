use serde::{Deserialize, Serialize};
use crate::api::*;


#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(default)]
pub struct Item {
    pub item_type_id: IntRequired,
    pub singleton: IntRequired,
    pub flag: IntRequired,
    pub quantity_destroyed: IntOptional,
    pub quantity_dropped: IntOptional,
    pub items: ItemsOptional,
}
impl Item {
    pub fn get_name(&self) -> String {
        get_name(&self.item_type_id)
    }
}
