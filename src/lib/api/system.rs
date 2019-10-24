use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use crate::api::*;
use crate::provider;
use crate::api::constellation::Constellation;

pub const AMARR_ID: IntRequired = 30002187;
pub const HEK_ID: IntRequired = 30002053;
pub const DODIXIE_ID: IntRequired = 30002659;
pub const JITA_ID: IntRequired = 30000142;
pub const RENS_ID: IntRequired = 30002510;

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
    pub fn new(system_id: IntRequired) -> Option<Self> {
        System::try_from(system_id).ok()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_constellation(&self) -> Option<Constellation> {
        Constellation::new(self.constellation_id)
    }

    pub fn get_full_name(&self) -> String {
        format!("{}({:0.1})::{}::{}",
            self.name,
            self.security_status,
            provider::get_name(&Some(self.constellation_id)),
            self.get_constellation().map(|x| provider::get_name(&Some(x.region_id))).unwrap_or_default())
    }

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
        let response = gw::eve_api(&format!("universe/systems/{}", id)).unwrap_or_default();
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

}
