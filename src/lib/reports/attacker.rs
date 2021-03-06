use crate::models;
use crate::services;
use crate::services::Context;
use crate::reports;

use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Attacker;
impl Attacker {
    pub fn write(output: &mut dyn Write, attacker: &models::attacker::AttackerNamed, _ctx: &Context) {
        reports::div(output, format!("{}", attacker.get_name("character")));
    }

    pub fn brief(arg: &String, ctx: &Context) -> String {
        if let Ok(ref id) = arg.parse::<i32>() {
            Self::brief_impl(id, ctx)
        } else {
            format!("Can't parse {}", arg)
        }
    }

    pub fn load(id: &i32, ctx: &Context) -> Option<Vec<models::attacker::AttackerNamed>> {
        use services::{Category, Report};
        match reports::load(Category::Attackers(*id), &ctx) {
            Report::Attackers(attackers) => return Some(attackers),
            Report::NotFoundId(id) => warn!("{} was not found", id),
            report => warn!("Unexpected report {:?}", report)
        }
        return None;
    }

    pub fn brief_impl(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        if let Some(attackers) = Self::load(id, ctx) {
            for attacker in attackers {
                Self::write(&mut output, &attacker, ctx);
            }
        }
        return output;
    }
}
