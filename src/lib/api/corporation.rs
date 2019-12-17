use crate::api::*;
use crate::provider;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Corporation {
    pub name: StrRequired,
    pub ticker: StrRequired,
    pub member_count: IntRequired,
    pub ceo_id: IntRequired,
    pub creator_id: IntRequired,
    pub tax_rate: FloatRequired,
    pub date_founded: TimeOptional,
    pub alliance_id: IntOptional,
    pub faction_id: IntOptional,
    pub home_station_id: IntOptional,
    pub shares: LongOptional,
    pub url: StrOptional,
    pub war_eligible: BoolOptional,
    pub description: StrOptional,
}
impl Corporation {
    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("corporations/{}", id)).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_corporation(id, &Self::load)
    }
}

impl TryFrom<String> for Corporation {
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
        let response = Corporation::new(&661107786);
        assert!(response.is_some());
        let corporation = response.unwrap();
        assert_eq!(&corporation.name, "CCP Engineering Corp");
        assert_eq!(&corporation.ticker, "CCPES");
    }
}
