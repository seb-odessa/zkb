use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

// https://redisq.zkillboard.com/listen.php?queueID=54689e7ff0b3cebfa1356bfbc9c7682c

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Zkb {
    pub locationID: i32,
    pub hash: String,
    pub fittedValue: f32,
    pub totalValue: f32,
    pub points: i32,
    pub npc: bool,
    pub solo: bool,
    pub awox: bool,
    pub href: String,
}
impl TryFrom<String> for Zkb {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub struct PackageContent {
//     pub killID: IntRequired,
//     pub killmail: KillMail,
//     pub zkb:
// }


//



#[cfg(test)]
mod tests {
    use super::*;
    use serde::export::Result;
    use serde_json::json;

    #[test]
    fn test_zkb() {
        let rec = json!({
            "locationID":40009240,
            "hash":"d470b0a91d10d8adbf5dcd1caac52b4462afade0",
            "fittedValue":242860.83,
            "totalValue":1997431.74,
            "points":1,
            "npc":false,
            "solo":true,
            "awox":false,
            "href":"https://esi.evetech.net/v1/killmails/79417923/d470b0a91d10d8adbf5dcd1caac52b4462afade0/"
        });
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val = Zkb::try_from(json.unwrap());
        assert!(val.is_ok());
        let record = val.unwrap();
        assert_eq!(40009240, record.locationID);
    }



    #[test]
    fn test_attackers() {
        let rec = json!({
        "package": {
            "killID":79417923,
            "killmail":{
                "attackers":[
                    {"character_id":2112200442,"corporation_id":98497155,"damage_done":741,"final_blow":true,"security_status":5,"ship_type_id":17720,"weapon_type_id":2913}],
                "killmail_id":79417923,
                "killmail_time":"2019-10-17T12:28:46Z",
                "solar_system_id":30000144,
                "victim":{
                    "character_id":2113793009,
                    "corporation_id":1000167,
                    "damage_taken":741,
                    "items":[],
                    "position":{"x":-496863799854.8822,"y":14718076067.006048,"z":145658997250.70566},
                    "ship_type_id":32880
                }
            },
            "zkb":{
                "locationID":40009240,
                "hash":"d470b0a91d10d8adbf5dcd1caac52b4462afade0",
                "fittedValue":242860.83,
                "totalValue":1997431.74,
                "points":1,
                "npc":false,
                "solo":true,
                "awox":false,
                "href":"https://esi.evetech.net/v1/killmails/79417923/d470b0a91d10d8adbf5dcd1caac52b4462afade0/"
            }
        }});

        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
    }
}