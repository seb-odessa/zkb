use crate::api;
use super::{zkb_href, link_system};

use std::fmt;


#[derive(Debug, PartialEq)]
pub struct Names {
    name: String,
    names: api::names::Names,
}
impl Names {
    pub fn new(name: &String) -> Option<Self> {
        api::names::Names::new(name).map(|names| Self{ name: name.clone(), names: names })
    }
}

fn try_write(f: &mut fmt::Formatter<'_>, items: &Option<Vec<api::names::Item>>) -> fmt::Result {
    if let Some(items) = items {
        for item in items {
            write!(f, "<div>{} = &gt {} </div>", item.id, item.name)?;
        }        
    }
    write!(f, "")
}

impl fmt::Display for Names {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<div>&lt{}&gt</div>", self.name)?;
        try_write(f, &self.names.agents)?;
        try_write(f, &self.names.alliances)?;
        try_write(f, &self.names.characters)?;
        try_write(f, &self.names.constellations)?;
        try_write(f, &self.names.corporations)?;
        try_write(f, &self.names.factions)?;
        try_write(f, &self.names.inventory_types)?;
        try_write(f, &self.names.regions)?;
        try_write(f, &self.names.stations)?;
        if let Some(items) = &self.names.systems {
            for item in items {
                write!(f, "<div>{} = &gt {} ({})</div>", 
                    item.id, 
                    link_system(&item.id, &item.name),
                    zkb_href("system", &Some(item.id), &Some(String::from("zkb"))))?;
            }        
        }
        write!(f, "")
    }
}