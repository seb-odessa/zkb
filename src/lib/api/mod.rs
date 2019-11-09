use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod gw;
pub mod killmail;
pub mod zkb;
pub mod system;
pub mod character;
pub mod object;
pub mod price;
pub mod region;
pub mod constellation;
pub mod stargate;
pub mod names;

pub use killmail::Killmail;
pub use killmail::Victim;
pub use killmail::Attacker;
pub use killmail::Item;
pub use object::Object;

pub type BoolRequired = bool;
pub type IntRequired = i32;
pub type FloatRequired = f32;
pub type FloatOptional = Option<f32>;
pub type IntOptional = Option<i32>;
pub type TimeRequired = DateTime<Utc>;
pub type StrRequired = String;
pub type StrOptional = Option<String>;
pub type PositionOptional = Option<Position>;
pub type IdsRequired = Vec<i32>;
pub type IdsOptional = Option<Vec<i32>>;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn get_name(id: &IntRequired) -> String {
    return object::Object::new(&id).map(|obj|obj.get_name()).unwrap_or_default()
}

pub fn try_get_name(id: &IntOptional) -> String {
    if let Some(id) = id {
        return object::Object::new(&id).map(|obj|obj.get_name()).unwrap_or_default()
    }
    return String::new();
}
