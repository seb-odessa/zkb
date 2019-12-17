use crate::api::*;
use crate::provider;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Alliance {
    pub name: StrRequired,
    pub creator_id: IntRequired,
    pub creator_corporation_id: IntRequired,
    pub ticker: StrRequired,
    pub date_founded: TimeRequired,
    pub executor_corporation_id: IntOptional,
    pub faction_id: IntOptional,
}
impl Alliance {
    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("alliances/{}", id)).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_alliance(id, &Self::load)
    }
}

impl TryFrom<String> for Alliance {
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
        let response = Alliance::new(&503818424);
        assert!(response.is_some());
        let alliance = response.unwrap();
        assert_eq!(&alliance.name, "CCP Engineering Alliance");
        assert_eq!(&alliance.ticker, "CCP-E");
        assert_eq!(alliance.executor_corporation_id, Some(661107786));
    }
}
