use crate::api::*;
use crate::provider;

use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Object {
    pub id: IntRequired,
    pub category: StrRequired,
    pub name: StrRequired,
}
impl Object {
    fn load(id: &i32) -> Option<Self> {
        let query = format!("[{}]", id);
        let response = gw::eve_api_post("universe/names", &query).unwrap_or_default();
        Self::try_from(response).ok()
    }

    pub fn new(id: &IntRequired) -> Option<Self> {
        provider::get_object(id, &Self::load)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_category(&self) -> String {
        self.category.clone()
    }
}
impl TryFrom<String> for Object {
    type Error = serde_json::Error;
    fn try_from(json: String) -> Result<Self, Self::Error> {
        use serde_json::Error;
        use serde_json::error::ErrorCode;
        let mut objects: Vec<Object> = serde_json::from_str(&json)?;
        objects.pop().ok_or(Error::syntax(ErrorCode::EofWhileParsingObject, 0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_api() {
        let response = Object::new(&2114350216);
        assert!(response.is_some());
        let object = response.unwrap();
        assert_eq!(object.id, 2114350216);
        assert_eq!(object.name, "Seb Odessa");
        assert_eq!(object.category, "character");
    }
}
