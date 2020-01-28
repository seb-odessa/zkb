use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use crate::api::*;
use crate::api::system::System;
use crate::provider;

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

    pub fn get_system_name(&self) -> String {
        get_name(&self.solar_system_id)
    }

    pub fn get_system_full_name(&self) -> String {
        System::new(&self.solar_system_id).map(|s| s.get_full_name()).unwrap_or_default()
    }

    fn get_sum<P>(id: &IntOptional, quantity: &IntOptional, get_price: &P) -> FloatRequired
    where P: Fn(&IntOptional)->FloatOptional {
        let quantity = quantity.unwrap_or(0);
        let price = get_price(id).unwrap_or(0.0);
        return quantity as f32 * price;
    }

    fn items_sum<Q, P>(items: &ItemsOptional, get_quantity: &Q, get_price: &P) -> FloatRequired
    where
        Q: Fn(&Item)->IntOptional,
        P: Fn(&IntOptional)->FloatOptional
        {
            items.as_ref().map_or(0.0, |items|{
                items.iter().map(|item| {
                    Killmail::get_sum(&Some(item.item_type_id), &get_quantity(item), get_price)
                    +
                    Killmail::items_sum(&item.items, get_quantity, get_price)
            }).fold(0.0, |acc, x| acc + x)
        })
    }

    pub fn get_dropped_sum(&self) -> u64 {
        Killmail::items_sum(
            &self.victim.items,
            &|item: &Item| {item.quantity_dropped},
            &provider::get_avg_price) as u64
    }

    pub fn get_destroyed_sum(&self) -> u64 {
        (
            Killmail::items_sum(
                &self.victim.items,
                &|item: &Item| {item.quantity_destroyed},
                &provider::get_avg_price)
            +
            Killmail::get_sum(&Some(self.victim.ship_type_id), &Some(1), &provider::get_avg_price)
        )
        as u64
    }

    pub fn get_total_sum(&self) -> u64 {
        self.get_destroyed_sum() + self.get_dropped_sum()
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
