use crate::api;
use super::{zkb_href, link_system};
use crate::services::Context;
use crate::reports::lazy;
use crate::reports::{div, href};
use std::fmt;


#[derive(Debug, PartialEq)]
pub struct System {
    pub id: i32,
    system: api::system::System,
    neighbors: Vec<api::system::System>,
}
impl System {

    pub fn report(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::system::System::new(id) {
            div(&mut output, "System", &href(object.zkb(), object.name.clone()));
            lazy(&mut output, format!("api/constellation/{}", object.constellation_id), &ctx);
        } else {
            div(&mut output, "System", &format!("{} not found", id));
        }
        return output;
    }

    pub fn new(id: &i32) -> Option<Self> {
        if let Some(system) = api::system::System::new(id) {
            let mut neighbors = Vec::new();
            if let Some(ref gates) = &system.stargates {
                for gate_id in gates {
                    if let Some(gate) = api::stargate::Stargate::new(gate_id) {
                        if let Some(neighbor) = api::system::System::new(&gate.destination.system_id) {
                            neighbors.push(neighbor);
                        }
                    }
                }
            }

            Some( Self {
                id: *id,
                system: system,
                neighbors: neighbors
            })
        } else {
            None
        }
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<div>System: {}</div>", zkb_href("system", &Some(self.id), &Some(self.system.get_name())))?;
        write!(f, "<div>Region: {}</div>", zkb_href("region", &self.system.get_region_id(), &self.system.get_region_name()))?;
        for system in &self.neighbors {
            write!(f, "<div>=&gt {}</div>", link_system(&system.system_id, &system.get_name()))?;
        }
        write!(f, "")
    }
}