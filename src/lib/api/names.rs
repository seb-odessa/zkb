use crate::api::*;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Item {
    pub id: IntRequired,
    pub name: StrRequired,
}

type OptItems = Option<Vec<Item>>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Names {
    pub agents: OptItems,
    pub alliances: OptItems,
    pub characters: OptItems,
    pub constellations: OptItems,
    pub corporations: OptItems,
    pub factions: OptItems,
    pub inventory_types: OptItems,
    pub regions: OptItems,
    pub stations: OptItems,
    pub systems: OptItems,
}
impl Names {
    fn load(name: &String) -> Option<Self> {
        let query = format!("[\"{}\"]", name);
        let response = gw::eve_api_post("universe/ids", &query).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(name: &String) -> Option<Self> {
        Self::load(name)
    }

}
impl TryFrom<String> for Names {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_api() {
        let response = Names::new(&String::from("Jita"));
        assert!(response.is_some());
        let names = response.unwrap();
        assert_eq!(names.alliances, Some(vec![Item{ id: 99005382, name: String::from("Jita Holding Inc.") }]));
        assert_eq!(names.corporations, Some(vec![Item{ id: 383768304, name: String::from("jion ss Corp") }]));
        assert_eq!(names.systems, Some(vec![Item{ id: 30000142, name: String::from("Jita") }]));
    }
}
