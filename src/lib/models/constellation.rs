use std::convert::From;
use crate::api;
use crate::schema::constellations;
use super::{Integer};

#[derive(Insertable)]
#[table_name = "constellations"]
pub struct Constellation {
    pub constellation_id: Integer,
    pub region_id: Integer,
}

impl From<&api::constellation::Constellation> for Constellation{
    fn from(src: &api::constellation::Constellation) -> Self {
        Self {
            constellation_id: src.constellation_id,
            region_id: src.region_id,
        }
    }
}


