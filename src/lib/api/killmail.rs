use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use crate::api::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Killmail {
    pub killmail_id: IntRequired,
    pub killmail_time: TimeRequired,
    pub solar_system_id: IntRequired,
    pub moon_id: IntOptional,
    pub war_id: IntOptional,
    pub victim: Victim,
    pub attackers: Vec<Attacker>,
}
impl TryFrom<String> for Killmail {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}
impl Killmail {
    pub fn href(&self)->String {
        format!("https://zkillboard.com/kill/{}/", self.killmail_id)
    }

    pub fn get_system_full_name(&self) -> String {
        system::System::new(&self.solar_system_id).map(|s| s.get_full_name()).unwrap_or_default()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::export::Result;
    use serde_json::json;
    use std::collections::HashMap;

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

            let val = Killmail::try_from(json.unwrap());
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
