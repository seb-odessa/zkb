use crate::models;
use crate::services;
use crate::services::Context;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Item;
impl Item {
    pub fn write(output: &mut dyn Write, item: &models::item::ItemNamed, _ctx: &Context) {
        reports::div(output, format!("{} : {} : {}", item.get_name(), item.get_dropped(), item.get_destroyed()));
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::brief_impl(id, ctx)
        } else {
            format!("Can't parse {}", arg)
        }
    }

    pub fn load(id: &i32, ctx: &Context) -> Option<Vec<models::item::ItemNamed>> {
        use services::{Category, Report};
        match reports::load(Category::Items(*id), &ctx) {
            Report::Items(items) => return Some(items),
            Report::NotFoundId(id) => warn!("{} was not found", id),
            report => warn!("Unexpected report {:?}", report)
        }
        return None;
    }

    pub fn brief_impl(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(items) = Self::load(id, ctx) {
            for item in items {
                Self::write(&mut output, &item, ctx);
            }
        }
        return output;
    }
}
