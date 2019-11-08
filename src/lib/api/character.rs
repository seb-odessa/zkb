use crate::api::*;
use crate::provider;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Character {
    pub race_id: IntRequired,
    pub bloodline_id: IntRequired,
    pub corporation_id: IntRequired,
    pub name: StrRequired,
    pub gender: StrRequired,
    pub birthday: TimeRequired,
    pub alliance_id: IntOptional,
    pub ancestry_id: IntOptional,
    pub faction_id: IntOptional,
    pub description: StrOptional,
    pub security_status: FloatOptional,
    pub title: StrOptional,
}
impl Character {
    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("characters/{}", id)).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_character(id, &Self::load)
    }
}

impl TryFrom<String> for Character {
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
        let response = Character::new(&95465499);
        assert!(response.is_some());
        let character = response.unwrap();
        assert_eq!(character.race_id, 2);
        assert_eq!(character.ancestry_id, Some(19));
        assert_eq!(&character.name, "CCP Bartender");
        assert_eq!(&character.gender, "male");
    }
}
