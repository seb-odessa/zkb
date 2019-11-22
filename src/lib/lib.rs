
#[macro_use]
extern crate diesel;
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate uuid;

use uuid::Uuid;

pub mod api;
pub mod models;
pub mod provider;
pub mod services;
pub mod reports;

pub use models::schema;

pub fn create_id() -> Uuid {
    Uuid::new_v4()
}
