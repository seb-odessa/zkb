use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use crate::api::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Object {
    pub id: IntRequired,
    pub category: StrRequired,
    pub name: StrRequired,
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
impl TryFrom<i32> for Object {
    type Error = serde_json::Error;
    fn try_from(id: i32) -> Result<Self, Self::Error> {
        let query = format!("[{}]", id);
        let response = gw::evetech_post("universe/names", &query).unwrap_or_default();
        Self::try_from(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_api() {
        let response = Object::try_from(2114350216);
        assert!(response.is_ok());
        let object = response.unwrap();
        assert_eq!(object.id, 2114350216);
        assert_eq!(object.name, "Seb Odessa");
        assert_eq!(object.category, "character");
    }
}
