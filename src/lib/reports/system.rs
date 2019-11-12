use crate::api;
use super::{zkb_href, link_system, FAIL};
use crate::services::Context;
use crate::reports::lazy;
use crate::reports::{div, href, root};
use chrono::{NaiveDateTime, Duration, Utc};

use std::fmt;
use std::fmt::Write;


#[derive(Debug, PartialEq)]
pub struct System {
    pub id: i32,
    system: api::system::System,
    neighbors: Vec<api::system::System>,
}
impl System {

    pub fn write(output: &mut dyn Write, system: &api::system::System, root: &String) {
        let url = format!("{}/api/system/{}", root, system.system_id);
        let name = format!("{} ({:.1})", system.name, system.security_status);
        std::fmt::write(
            output,
            format_args!(
                "<div>System: {url} {zkb}</div>",
                url = href(&url, &name),
                zkb = href(system.zkb(), String::from("(zkb)"))
            )
        ).expect(FAIL);
    }

    pub fn brief(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::system::System::new(id) {
            Self::write(&mut output, &object, &root(ctx));
        } else {
            div(&mut output, "System", &format!("{} not found", id));
        }
        return output;
    }

    pub fn report(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(object) = api::system::System::new(id) {
            Self::write(&mut output, &object, &root(ctx));
            lazy(&mut output, format!("api/constellation/{}", object.constellation_id), &ctx);
            lazy(&mut output, format!("api/region/{}", object.get_region_id().unwrap_or_default()), &ctx);
            if let Some(ref gates) = &object.stargates {
                for gate_id in gates {
                    if let Some(object) = api::stargate::Stargate::new(gate_id) {
                        lazy(&mut output, format!("api/system_brief/{}", object.destination.system_id), &ctx);
                    }
                }
            }
            div(&mut output, "Current ET", &NaiveDateTime::from(Utc::now().naive_utc()).time().to_string());
            div(&mut output, "Kill history 60 minutes", "");
            lazy(&mut output, format!("history/{}/{}", id, 60), &ctx);
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