use crate::api::*;
use crate::provider;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Destination {
    pub stargate_id: IntRequired,
    pub system_id: IntRequired,    
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Stargate {
    pub stargate_id: IntRequired,
    pub name: StrRequired,
    pub type_id: IntRequired,
    pub system_id: IntRequired,
    pub destination: Destination,
}
impl Stargate {
    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("universe/stargates/{}", id)).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_stargate(id, &Self::load)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

}
impl TryFrom<String> for Stargate {
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
        let maybe = Stargate::new(&50000342);
        assert!(maybe.is_some());
        let stargate = maybe.unwrap();
        assert_eq!(50000342, stargate.stargate_id);
        assert_eq!(29624,    stargate.type_id);
        assert_eq!(30000003, stargate.system_id);
        assert_eq!("Stargate (Tanoo)", stargate.name);
        assert_eq!(50000056, stargate.destination.stargate_id);
        assert_eq!(30000001, stargate.destination.system_id);

    }

}
