use crate::models;
use crate::services;
use crate::services::Context;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Victim;
impl Victim {

    pub fn write(output: &mut dyn Write, victim: &models::victim::VictimNamed, _ctx: &Context) {
        reports::div(output, format!("{}", victim.get_name("character")));
    }

    pub fn report_name(id: &i32, ctx: &Context) -> String {
        let mut result = String::new();
        if let Some(victim) = reports::Victim::load(&id, ctx) {
            if let Some(id) = victim.faction_id {
                if !result.is_empty() {
                    result = result + " : ";
                }
                result = result + &reports::tip("faction", ctx.get_api_href("faction", id, victim.get_name("faction")));
            }
            if let Some(id) = victim.alliance_id {
                if !result.is_empty() {
                    result = result + " : ";
                }
                result = result + &reports::tip("alliance", ctx.get_api_href("alliance", id, victim.get_name("alliance")));
            }
            if let Some(id) = victim.corporation_id {
                if !result.is_empty() {
                    result = result + " : ";
                }
                result = result + &reports::tip("corporation", ctx.get_api_href("corporation", id, victim.get_name("corporation")));
            }
            if let Some(id) = victim.character_id {
                if !result.is_empty() {
                    result = result + " : ";
                }
                result = result + &reports::tip("character", ctx.get_api_href("character", id, victim.get_name("character")));
            }
        }
        return result;
    }

    pub fn load(id: &i32, ctx: &Context) -> Option<models::victim::VictimNamed> {
        use services::{Category, Report};
        match reports::load(Category::Victim(*id), &ctx) {
            Report::Victim(victim) => return Some(victim),
            Report::NotFoundId(id) => warn!("{} was not found", id),
            report => warn!("Unexpected report {:?}", report)
        }
        return None;
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::brief_impl(id, ctx)
        } else {
            format!("Can't parse {}", arg)
        }
    }

    pub fn brief_impl(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(victim) = Self::load(id, ctx) {
            Self::write(&mut output, &victim, ctx);
        }
        return output;
    }
}
