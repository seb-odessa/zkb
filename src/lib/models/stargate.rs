use std::convert::From;
use crate::api;
use crate::schema::stargates;
use super::{Integer};

#[derive(Insertable)]
#[table_name = "stargates"]
pub struct Stargate {
    pub stargate_id: Integer,
    pub type_id: Integer,
    pub system_id: Integer,
    pub dst_stargate_id: Integer,
    pub dst_system_id: Integer,
}

impl From<&api::stargate::Stargate> for Stargate{
    fn from(src: &api::stargate::Stargate) -> Self {
        Self {
            stargate_id: src.stargate_id,
            type_id: src.type_id,
            system_id: src.system_id,
            dst_stargate_id: src.destination.stargate_id,
            dst_system_id: src.destination.system_id,
        }
    }
}


