use serde::{Deserialize, Serialize};
use crate::api::*;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(default)]
pub struct Victim {
    pub ship_type_id: IntRequired,
    pub damage_taken: IntRequired,
    pub character_id: IntOptional,
    pub corporation_id: IntOptional,
    pub alliance_id: IntOptional,
    pub faction_id: IntOptional,
    pub items: ItemsOptional,
    pub position: PositionOptional,
}
impl Victim {
    pub fn get_ship(&self) -> String {
        Object::new(&self.ship_type_id).map(|obj|obj.get_name()).unwrap_or_default()
    }
    pub fn get_character(&self) -> String {
        try_get_name(&self.character_id)
    }
    pub fn get_corporation(&self) -> String {
        try_get_name(&self.corporation_id)
    }
    pub fn get_alliance(&self) -> String {
        try_get_name(&self.alliance_id)
    }
    pub fn get_faction(&self) -> String {
        try_get_name(&self.faction_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::export::Result;
    use serde_json::json;

    #[test]
    fn test_victim() {
        let rec = json!({
            "character_id": 2112827232,
            "corporation_id": 98605751,
            "damage_taken": 1431,
            "items": [
                {
                    "flag": 5,
                    "item_type_id": 266,
                    "quantity_dropped": 2800,
                    "singleton": 0
                },
                {
                    "flag": 29,
                    "item_type_id": 27333,
                    "quantity_dropped": 50,
                    "singleton": 0
                },
            ],
            "position": {
                "x": -361424408960.0218,
                "y": 123646758982.49516,
                "z": 337540581410.30054
            },
            "ship_type_id": 598
        });
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val: Result<Victim, serde_json::Error> = serde_json::from_str(&json.unwrap());
        assert!(val.is_ok());
        let record = val.unwrap();
        assert_eq!(Some(2112827232), record.character_id);
        assert_eq!(Some(98605751), record.corporation_id);
        assert_eq!(1431, record.damage_taken);
        assert_eq!(598, record.ship_type_id);
        assert!(record.items.is_some());
        let items = record.items.unwrap();
        assert_eq!(266, items[0].item_type_id);
        assert_eq!(27333, items[1].item_type_id);
        assert!(record.position.is_some());
        assert_eq!(337540581410.30054, record.position.unwrap().z);
    }
}
