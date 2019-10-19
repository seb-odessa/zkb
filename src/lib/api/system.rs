use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use crate::api::*;

pub const AMARR_ID: IntRequired = 30002187;
pub const HEK_ID: IntRequired = 30002053;
pub const DODIXIE_ID: IntRequired = 30002659;
pub const JITA_ID: IntRequired = 30000142;
pub const RENS_ID: IntRequired = 30002510;

pub type PlanetOptional = Option<Vec<Planet>>;

pub fn route(departue: IntRequired, destination: IntRequired) -> IdsRequired {
    let uri = format!("route/{}/{}", departue, destination);
    let response = gw::evetech(&uri).unwrap_or_default();
    serde_json::from_str(&response).unwrap_or_default()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct System {
    pub system_id: IntRequired,
    pub star_id: IntOptional,
    pub name: StrRequired,
    pub position: Position,
    pub security_class: StrOptional,
    pub security_status: FloatRequired,
    pub constellation_id: IntRequired,
    pub planets: PlanetOptional,
    pub stargates: IdsOptional,
    pub stations: IdsOptional,
}
impl TryFrom<String> for System {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}
impl TryFrom<i32> for System {
    type Error = serde_json::Error;
    fn try_from(id: i32) -> Result<Self, Self::Error> {
        let response = gw::evetech(&format!("universe/systems/{}", id)).unwrap_or_default();
        System::try_from(response)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(default)]
pub struct Planet {
    pub planet_id: IntRequired,
    pub asteroid_belts: IdsOptional,
    pub moons: IdsOptional,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_api() {
        let maybe = System::try_from(30002659);
        assert!(maybe.is_ok());
        let system = maybe.unwrap();
        assert_eq!(30002659, system.system_id);
        assert_eq!("Dodixie", &system.name);
        assert_eq!(Some(String::from("B")), system.security_class);
    }

    #[test]
    fn test_route() {
        let route = route(JITA_ID, HEK_ID);
        assert_eq!(10, route.len());
    }
}
