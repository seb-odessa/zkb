use std::convert::TryFrom;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type BoolRequired = bool;
pub type IntRequired = i32;
pub type FloatRequired = f32;
pub type IntOptional = Option<i32>;
pub type TimeRequired = DateTime<Utc>;
pub type ItemsOptional = Option<Vec<Item>>;
pub type PositionOptional = Option<Position>;

//https://esi.evetech.net/latest/swagger.json

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct KillMail {
    pub killmail_id: IntRequired,
    pub killmail_time: TimeRequired,
    pub solar_system_id: IntRequired,
    pub moon_id: IntOptional,
    pub war_id: IntOptional,
    pub victim: Victim,
    pub attackers: Vec<Attacker>,
}
impl TryFrom<String> for KillMail {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Attacker {
    pub ship_type_id: IntOptional,
    pub character_id: IntOptional,
    pub corporation_id: IntOptional,
    pub alliance_id: IntOptional,
    pub faction_id: IntOptional,
    pub damage_done: IntRequired,
    pub final_blow: BoolRequired,
    pub security_status: FloatRequired,
    pub weapon_type_id: IntOptional,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Item {
    pub item_type_id: IntRequired,
    pub singleton: IntRequired,
    pub flag: IntRequired,
    pub quantity_destroyed: IntOptional,
    pub quantity_dropped: IntOptional,
    pub items: ItemsOptional,
}


#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::export::Result;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_attackers() {
        {
            let rec = json!({
            "character_id": 3019582,
            "corporation_id": 1000274,
            "damage_done": 1431,
            "faction_id": 500024,
            "final_blow": true,
            "security_status": 0,
            "ship_type_id": 34495,
            "weapon_type_id": 34580
            });

            let json = serde_json::to_string(&rec);
            assert!(json.is_ok());
            let val: Result<Attacker, serde_json::Error> = serde_json::from_str(&json.unwrap());
            assert!(val.is_ok());
            let record = val.unwrap();
            assert_eq!(Some(3019582), record.character_id);
            assert_eq!(Some(1000274), record.corporation_id);
        }
        {
            let rec = json!({
                "damage_done": 0,
                "faction_id": 500024,
                "final_blow": false,
                "security_status": 0,
                "ship_type_id": 34495

            });

            let json = serde_json::to_string(&rec);
            assert!(json.is_ok());
            let val: Result<Attacker, serde_json::Error> = serde_json::from_str(&json.unwrap());
            assert!(val.is_ok());
            let record = val.unwrap();
            assert_eq!(Some(500024), record.faction_id);
            assert_eq!(Some(34495), record.ship_type_id);
        }
    }

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
    #[test]
    fn test_killmail() {
        {
            let rec = json!({
            "attackers": [{
                "character_id": 3019582,
                "corporation_id": 1000274,
                "damage_done": 1431,
                "faction_id": 500024,
                "final_blow": true,
                "security_status": 0,
                "ship_type_id": 34495,
                "weapon_type_id": 34580
            },{
                "damage_done": 0,
                "faction_id": 500024,
                "final_blow": false,
                "security_status": 0,
                "ship_type_id": 34495
            }],
            "victim": {
                "character_id": 94626634,
                "corporation_id": 1000107,
                "damage_taken": 2763,
                "items": [{
                    "flag": 5,
                    "item_type_id": 21898,
                    "quantity_dropped": 1640,
                    "singleton": 0
                },],
                "position": {
                    "x": 1672272956584.0535,
                    "y": -52529732329.21149,
                    "z": -775276459137.9266
                },
                "ship_type_id": 587
            },
            "killmail_id": 78560358,
            "killmail_time": "2019-08-22T01:26:53Z",
            "solar_system_id": 30002384,
            });

            let json = serde_json::to_string(&rec);
            assert!(json.is_ok());

            let val = KillMail::try_from(json.unwrap());
            assert!(val.is_ok());
            let record = val.unwrap();
            assert_eq!(2, record.attackers.len());
            assert_eq!(Some(3019582), record.attackers[0].character_id);
            assert_eq!(Some(500024), record.attackers[1].faction_id);
            assert_eq!(78560358, record.killmail_id);
            assert_eq!(30002384, record.solar_system_id);
        }
    }

    #[test]
    fn test_history() {
        // Returned by https://zkillboard.com/api/history/YYYYMMDD.json /
        let rec = json!({
            "78146996":"4ceed992204ea5cab36f9543e80b90f0417534f5",
            "78146999":"f22a5166bfc52151c029cc169d9e0c289c439233",
            "78147000":"34177ddc51664e50e2c6f7ef91f9e8a75f7addc1"
        });
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val: Result<HashMap<u64, String>, serde_json::Error> =
            serde_json::from_str(&json.unwrap());
        assert!(val.is_ok());
        let map = val.unwrap();
        assert!(map.get(&78146999).is_some());
        assert_eq!(
            "f22a5166bfc52151c029cc169d9e0c289c439233",
            map.get(&78146999).unwrap()
        );
    }
}
