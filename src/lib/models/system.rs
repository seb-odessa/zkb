use std::convert::From;
use crate::api;
use crate::schema::systems;
use super::{Integer, OptInteger, Float};

#[derive(Insertable)]
#[table_name = "systems"]
pub struct System {
    pub system_id: Integer,
    pub star_id: OptInteger,
    pub security_status: Float,
    pub constellation_id: Integer,
}

impl From<&api::system::System> for System {
    fn from(src: &api::system::System) -> Self {
        Self {
            system_id: src.system_id,
            star_id: src.star_id,
            security_status: src.security_status,
            constellation_id: src.constellation_id,
        }
    }
}


