use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use crate::api::*;


pub type PlanetOptional = Option<Vec<Planet>>;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(default)]
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
        println!("{}", response);
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
    fn test_from() {
        let maybe = System::try_from(30002659);
        assert!(maybe.is_ok());
        let system = maybe.unwrap();
        assert_eq!(30002659, system.system_id);
        assert_eq!("Dodixie", &system.name);
        assert_eq!(Some(String::from("B")), system.security_class);
    }
}
