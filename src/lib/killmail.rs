
pub mod curl;
pub mod database;
pub mod models;
pub mod schema;

use serde::{Deserialize, Serialize};

pub type BoolRequired = bool;
pub type IntRequired = i32;
pub type IntOptional = Option<i32>;
pub type LongOptional = Option<i64>;
pub type StrRequired = String;
pub type ItemsOptional = Vec<Item>;
pub type PositionOptional = Vec<Position>;


//https://esi.evetech.net/latest/swagger.json

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KillMail {
    killmail_id: IntRequired,
    killmail_time: StrRequired,
    solar_system_id: IntRequired,
    moon_id: IntOptional,
    war_id: IntOptional,
    victim: Victim,
    attackers: Vec<Attacker>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Victim {
    ship_type_id: IntRequired,
    damage_taken: IntRequired,
    character_id: IntOptional,
    corporation_id: IntOptional,
    alliance_id: IntOptional,
    faction_id: IntOptional,
    items: ItemsOptional,
    position: Position,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Attacker {
    ship_type_id: IntOptional,
    character_id: IntOptional,
    corporation_id: IntOptional,
    alliance_id: IntOptional,
    faction_id: IntOptional,
    damage_done: IntRequired,    
    final_blow: BoolRequired,
    security_status: IntRequired,
    weapon_type_id: IntOptional,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Item {
    item_type_id: IntRequired,
    singleton: IntRequired,
    flag: IntRequired,
    quantity_destroyed: LongOptional,
    quantity_dropped: LongOptional,
    items: ItemsOptional,
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::export::Result;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_id() {
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

    #[test]
    fn test_zkb() {
        let rec = json!({
        "killmail_id":78766279,
        "zkb":{
            "locationID":40152304,
            "hash":"05b689f860cd720cf3c8f71ab4c5100aff396081",
            "fittedValue":43913214.04,
            "totalValue":102163132.51,
            "points":1,
            "npc":true,
            "solo":false,
            "awox":false}
        });
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val: Result<Zkb, serde_json::Error> = serde_json::from_str(&json.unwrap());
        assert!(val.is_ok());
        let record: Zkb = val.unwrap();
        assert_eq!(78766279, record.killmail_id);
        assert_eq!("05b689f860cd720cf3c8f71ab4c5100aff396081", record.zkb.hash);
    }

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
            assert_eq!(3019582, record.character_id);
            assert_eq!(1000274, record.corporation_id);
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
            assert_eq!(500024, record.faction_id);
            assert_eq!(34495, record.ship_type_id);
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
        assert_eq!(2112827232, record.character_id);
        assert_eq!(98605751, record.corporation_id);
        assert_eq!(1431, record.damage_taken);
        assert_eq!(598, record.ship_type_id);
        assert_eq!(266, record.items[0].item_type_id);
        assert_eq!(27333, record.items[1].item_type_id);
        assert_eq!(337540581410.30054, record.position.z);
    }
    #[test]
    fn test_killmail() {
        {
            let rec = json!({
            "attackers": [
            {
                "character_id": 3019582,
                "corporation_id": 1000274,
                "damage_done": 1431,
                "faction_id": 500024,
                "final_blow": true,
                "security_status": 0,
                "ship_type_id": 34495,
                "weapon_type_id": 34580
            },
            {
                "damage_done": 0,
                "faction_id": 500024,
                "final_blow": false,
                "security_status": 0,
                "ship_type_id": 34495
            }],
            "killmail_id": 78560358,
            "killmail_time": "2019-08-22T01:26:53Z",
            "solar_system_id": 30002384,
            });

            let json = serde_json::to_string(&rec);
            assert!(json.is_ok());
            let val: Result<KillMail, serde_json::Error> = serde_json::from_str(&json.unwrap());
            assert!(val.is_ok());
            let record = val.unwrap();
            assert_eq!(2, record.attackers.len());
            assert_eq!(3019582, record.attackers[0].character_id);
            assert_eq!(500024, record.attackers[1].faction_id);
            assert_eq!(78560358, record.killmail_id);
            assert_eq!(30002384, record.solar_system_id);
        }
    }

    #[test]
    fn test_111() {
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