use crate::api;
use crate::services::{Context, Report, Category};
use crate::reports;
use chrono::Utc;
use std::fmt::Write;

#[derive(Debug, PartialEq)]
pub struct Character;
impl Character {

    fn info(output: &mut dyn Write, id: &i32, ctx: &Context) {
        let name = if let Report::Object(obj) = reports::load(Category::Object(*id), &ctx) {
            obj.object_name
        } else {
            String::new()
        };
        reports::div(output, format!("Name: {}",ctx.get_full_desc("character", *id, name)));

        if let Some(character) = api::character::Character::new(&id) {
            let id = character.corporation_id;
            if let Report::Object(obj) = reports::load(Category::Object(id), &ctx) {
                reports::div(output, format!("Corporation: {}",ctx.get_full_desc("corporation", id, obj.object_name)));
            }
            if let Some(alliance_id) = character.alliance_id {
                let id = alliance_id;
                if let Report::Object(obj) = reports::load(Category::Object(id), &ctx) {
                    reports::div(output, format!("Alliance:  {}",ctx.get_full_desc("alliance", id, obj.object_name)));
                }
            }
            if let Some(ss) = character.security_status {
                reports::div(output, format!("Security Status: {:.2}", ss));
            }
            let diff = Utc::now().signed_duration_since(character.birthday);
            let age = diff.num_days();
            let years = age / 365;
            let months = (age - 365 * years) / 30;
            let days = age - 365 * years - 30 * months;
            let span = if 0 == months && 0 == years {
                reports::span("", "background-color: lightgreen;", format!("Character age: {} days", days))
            } else if 0 == years {
                reports::span("", "background-color: skyblue;", format!("Character age: {} months and {} days", months, days))
            } else {
                reports::span("", "background-color: LightCoral;", format!("Character age: {} years, {} months and {} days", years, months, days))
            };
            reports::div(output, span);
        }
    }
}
impl reports::Reportable for Character {
    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        Self::info(&mut output, &id, ctx);
        reports::lazy(&mut output, format!("report/character/wins/{}/{}", id, 60), &ctx);
        reports::lazy(&mut output, format!("report/character/losses/{}/{}", id, 60), &ctx);
        return output;
    }
}
