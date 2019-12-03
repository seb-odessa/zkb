use crate::models::*;
use crate::reports;
use crate::services::{Context, Report, Area, Actor, Category};
use chrono::{Duration, Utc};

#[derive(Debug, PartialEq)]
pub struct History;
impl History {
    fn count(category: Category, ctx: &Context) -> i32 {
        match reports::load(category, &ctx) {
            Report::HistoryCount(count) => {
                return count;
            },
            report => {
                warn!("Unexpected report {:?}", report);
            }
        }
        return 0;
    }

    fn report_impl(category: Category, minutes: &Integer, ctx: &Context) -> String {
        let mut output = String::new();
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        let timestamp = start.format("%Y-%m-%d %H:%M:%S").to_string();
        reports::div(&mut output, format!("History since {} ", timestamp));
        match reports::load(category, &ctx) {
            Report::History(history) => {
                reports::table_start(&mut output, "Attackers", "border-collapse: collapse;", "");
                for killmail in history {
                    reports::Killmail::write_row(&mut output, &killmail, &ctx);
                }
                reports::table_end(&mut output);
            },
            Report::HistoryCount(count) => {
                reports::div(&mut output, format!("{:0>3}", count));
            },
            report => {
                reports::div(&mut output, format!("Unexpected report {:?}", report));
            }
        }
        return output;
    }

    pub fn system(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report_impl(Category::History((Area::System(*id), *minutes)), minutes, ctx)
    }

    pub fn region(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report_impl(Category::History((Area::Region(*id), *minutes)), minutes, ctx)
    }

    pub fn constellation(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report_impl(Category::History((Area::Constellation(*id), *minutes)), minutes, ctx)
    }

    pub fn report(category: &String, class: &String, id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        let actor = match category.as_ref() {
            "character" => Actor::Character(*id),
            "corporation" => Actor::Corporation(*id),
            "alliance" => Actor::Alliance(*id),
            "faction" => Actor::Faction(*id),
            _ => return format!("Unexpected category")
        };

        let category = match class.as_ref() {
            "wins" => Category::Wins((actor, *minutes)),
            "losses" => Category::Losses((actor, *minutes)),
            _ => return format!("Unexpected report class")
        };

        Self::report_impl(category, minutes, ctx)
    }


    pub fn system_count(id: &Integer, minutes: &Integer, ctx: &Context) -> i32 {
        Self::count(Category::HistoryCount((Area::System(*id), *minutes)), ctx)
    }

    pub fn region_count(id: &Integer, minutes: &Integer, ctx: &Context) -> i32 {
        Self::count(Category::HistoryCount((Area::Region(*id), *minutes)), ctx)
    }

    pub fn constellation_count(id: &Integer, minutes: &Integer, ctx: &Context) -> i32 {
        Self::count(Category::HistoryCount((Area::Constellation(*id), *minutes)), ctx)
    }
}


