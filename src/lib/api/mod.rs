use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod gw;
pub mod killmail;
pub mod zkb;
pub mod system;
pub mod character;

pub type BoolRequired = bool;
pub type IntRequired = i32;
pub type FloatRequired = f32;
pub type FloatOptional = Option<f32>;
pub type IntOptional = Option<i32>;
pub type TimeRequired = DateTime<Utc>;
pub type StrRequired = String;
pub type StrOptional = Option<String>;
pub type PositionOptional = Option<Position>;
pub type IdsOptional = Option<Vec<i32>>;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[serde(default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}