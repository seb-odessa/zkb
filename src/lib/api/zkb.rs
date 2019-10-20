use super::killmail::KillMail;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

// https://redisq.zkillboard.com/listen.php?queueID=54689e7ff0b3cebfa1356bfbc9c7682c

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Zkb {
    #[serde(alias = "locationID")]
    pub location_id: i32,
    pub hash: String,
    #[serde(alias = "fittedValue")]
    pub fitted_value: f32,
    #[serde(alias = "totalValue")]
    pub total_value: f32,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PackageContent {
    #[serde(alias = "killID")]
    pub id: i32,
    pub killmail: KillMail,
    pub zkb: Zkb,
}
impl PackageContent {
    pub fn zkb_url(&self)->String {
        format!("https://zkillboard.com/kill/{}/", self.id)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Package {
    #[serde(alias = "package")]
    pub content: Option<PackageContent>,
}
impl TryFrom<String> for Package {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
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
        assert_eq!(40009240, record.location_id);
    }

    #[test]
    fn test_package_content() {
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

        println!("{:?}", rec);
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val = Package::try_from(json.unwrap());
        assert!(val.is_ok());
        let record = val.unwrap();
        assert!(record.content.is_some());
        let content = record.content.unwrap();
        assert_eq!(79417923, content.id);
        assert_eq!(79417923, content.killmail.killmail_id);
        assert_eq!(40009240, content.zkb.location_id);
    }

    #[test]
    fn test_package_without_content() {
        let none: Option<PackageContent> = None;
        let rec = json!({"package": none});
        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());
        let val = Package::try_from(json.unwrap());
        assert!(val.is_ok());
        let record = val.unwrap();
        assert!(record.content.is_none());
    }
}