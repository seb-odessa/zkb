use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod gw;
pub mod killmail;
mod item;
mod victim;
mod attacker;
mod order;
pub mod zkb;
pub mod system;
pub mod character;
pub mod object;
pub mod price;
pub mod region;
pub mod constellation;
pub mod stargate;
pub mod names;
pub mod alliance;
pub mod corporation;
pub mod group;
pub mod stats;


pub use killmail::Killmail;
pub use victim::Victim;
pub use attacker::Attacker;
pub use item::Item;
pub use object::Object;
pub use order::Order;
pub use order::Orders;
pub use order::OrderType;

pub type BoolRequired = bool;
pub type BoolOptional = Option<bool>;
pub type IntRequired = i32;
pub type FloatRequired = f32;
pub type FloatOptional = Option<f32>;
pub type IntOptional = Option<i32>;
pub type LongRequired = i64;
pub type LongOptional = Option<i64>;
pub type TimeRequired = DateTime<Utc>;
pub type TimeOptional = Option<DateTime<Utc>>;
pub type StrRequired = String;
pub type StrOptional = Option<String>;
pub type PositionOptional = Option<Position>;
pub type IdsRequired = Vec<i32>;
pub type IdsOptional = Option<Vec<i32>>;
pub type ItemsOptional = Option<Vec<Item>>;

// https://esi.evetech.net/latest/swagger.json

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
