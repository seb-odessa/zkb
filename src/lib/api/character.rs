use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use crate::api::*;

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
impl TryFrom<String> for Character {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&json)
    }
}
impl TryFrom<i32> for Character {
    type Error = serde_json::Error;
    fn try_from(id: i32) -> Result<Self, Self::Error> {
        let response = gw::eve_api(&format!("characters/{}", id)).unwrap_or_default();
        Self::try_from(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_api() {
        let response = Character::try_from(2114350216);
        assert!(response.is_ok());
        let character = response.unwrap();
        assert_eq!(character.race_id, 1);
        assert_eq!(&character.name, "Seb Odessa");
        assert_eq!(&character.gender, "male");
    }
}
