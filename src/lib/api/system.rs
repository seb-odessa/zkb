use crate::api::*;
use crate::provider;
use crate::api::constellation::Constellation;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

pub type PlanetOptional = Option<Vec<Planet>>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
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
impl System {
    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("universe/systems/{}", id)).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_system(id, &Self::load)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_constellation(&self) -> Option<Constellation> {
        Constellation::new(&self.constellation_id)
    }

    pub fn get_full_name(&self) -> String {
        format!("{}/{}({:0.1})",
            self.get_constellation().map(|o| get_name(&o.region_id)).unwrap_or_default(),
            self.name,
            self.security_status
            )
    }
}
impl TryFrom<String> for System {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
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
    fn test_new() {
        let maybe = System::new(&30002659);
        assert!(maybe.is_some());
        let system = maybe.unwrap();
        assert_eq!(30002659, system.system_id);
        assert_eq!("Dodixie", &system.name);
        assert_eq!(Some(String::from("B")), system.security_class);
    }

}
