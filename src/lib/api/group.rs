use crate::api::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Group {
    pub group_id: IntRequired,
    pub category_id: IntRequired,
    pub name: StrRequired,
    pub published: BoolRequired,
    pub types: IdsRequired,
}
impl Group {
    fn load(id: &i32) -> Option<Self> {
        let response = gw::eve_api(&format!("universe/groups/{}", id)).unwrap_or_default();
        serde_json::from_str(&response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        Self::load(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_api() {
        let response = Group::new(&420);
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.group_id, 420);
        assert_eq!(object.category_id, 6);
        assert_eq!(object.name, "Destroyer");
        assert_eq!(object.types.len(), 20);
    }
}
