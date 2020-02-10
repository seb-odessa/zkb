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
        reports::div(output, format!("Name: {}",ctx.get_actor_desc("character", *id, name)));

        if let Some(character) = api::character::Character::new(&id) {
            let id = character.corporation_id;
            if let Report::Object(obj) = reports::load(Category::Object(id), &ctx) {
                reports::div(output, format!("Corporation: {}",ctx.get_actor_desc("corporation", id, obj.object_name)));
            }
            if let Some(alliance_id) = character.alliance_id {
                let id = alliance_id;
                if let Report::Object(obj) = reports::load(Category::Object(id), &ctx) {
                    reports::div(output, format!("Alliance:  {}",ctx.get_actor_desc("alliance", id, obj.object_name)));
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
    pub fn stat(id: &i32, _ctx: &Context) -> String {
        use api::stats::Stats;
        use api::stats::Entity;

        let mut output = String::new();
        if let Some(stats) = Stats::new(Entity::Character(*id)) {
            let table_style = "border-collapse: collapse;";
            let head_style = "border: 1px solid black; padding: 2px 5px; text-align: center;";
            let color = "White";
            let text_style = &format!("border: 1px solid black; padding: 2px 5px; background-color: {};", color);
            let num_style = &format!("border: 1px solid black; padding: 2px 5px; text-align: right;  background-color: {};", color);

            reports::table_start(&mut output, "", table_style, "");
            reports::table_row_start(&mut output, head_style);
            reports::table_cell_head(&mut output, "", head_style, "");
            reports::table_cell_head(&mut output, "", head_style, "Destroyed");
            reports::table_cell_head(&mut output, "", head_style, "Lost");
            reports::table_row_end(&mut output);
            reports::table_row_start(&mut output, text_style);
            reports::table_cell(&mut output, "", text_style, "Ships");
            reports::table_cell(&mut output, "", num_style, format!("{}", stats.ship_destroyed.unwrap_or_default()));
            reports::table_cell(&mut output, "", num_style, format!("{}", stats.ship_lost.unwrap_or_default()));
            reports::table_row_end(&mut output);
            reports::table_row_start(&mut output, text_style);
            reports::table_cell(&mut output, "", text_style, "Solo");
            reports::table_cell(&mut output, "", num_style, format!("{}", stats.solo_kills.unwrap_or_default()));
            reports::table_cell(&mut output, "", num_style, format!("{}", stats.solo_losses.unwrap_or_default()));
            reports::table_row_end(&mut output);
            reports::table_end(&mut output);
            reports::div(&mut output, format!("Danger: {}%", stats.danger_ratio));
            reports::div(&mut output, format!("Gangs: {}%", stats.gang_ratio));

        }
        return output;
    }
}
impl reports::Reportable for Character {
    fn report_by_id(id: &i32, ctx: &Context) -> String {
        let mut output = String::new();
        Self::info(&mut output, &id, ctx);
        reports::lazy(&mut output, format!("stat/character/{}", id), &ctx);
        reports::lazy(&mut output, format!("report/character/wins/{}/{}", id, 60), &ctx);
        reports::lazy(&mut output, format!("report/character/losses/{}/{}", id, 60), &ctx);
        return output;
    }
}
