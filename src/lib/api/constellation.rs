use crate::api::*;
use crate::provider;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Constellation {
    pub constellation_id: IntRequired,
    pub name: StrRequired,
    pub position: Position,
    pub region_id: IntRequired,
    pub systems: IdsRequired
}
impl Constellation {

    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("universe/constellations/{}", id)).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_constellation(id, &Self::load)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl TryFrom<String> for Constellation {
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
        let maybe = Constellation::new(&20000009);
        assert!(maybe.is_some());
        let object = maybe.unwrap();
        assert_eq!(20000009, object.constellation_id);
        assert_eq!("Mekashtad", &object.name);
        assert_eq!(10000001, object.region_id);
    }

}
