use serde::{Deserialize, Serialize};
use crate::api::*;


#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
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
impl Attacker {
    pub fn get_ship(&self) -> String {
        try_get_name(&self.ship_type_id)
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
    pub fn get_weapon(&self) -> String {
        try_get_name(&self.weapon_type_id)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde::export::Result;
    use serde_json::json;

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
}
