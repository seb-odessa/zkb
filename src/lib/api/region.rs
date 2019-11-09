use crate::api::*;
use crate::provider;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Region {
    pub region_id: IntRequired,
    pub name: StrRequired,
    pub constellations: IdsRequired,
    pub description: StrOptional,
}
impl Region {

    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("universe/regions/{}", id)).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn zkb(&self) -> String {
        format!("https://zkillboard.com/region/{}/", self.region_id)
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_region(id, &Self::load)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl TryFrom<String> for Region {
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
        let maybe = Region::new(&10000001);
        assert!(maybe.is_some());
        let object = maybe.unwrap();
        assert_eq!(10000001, object.region_id);
        assert_eq!("Derelik", &object.name);
    }

}
