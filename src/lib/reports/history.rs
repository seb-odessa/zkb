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

    fn report(category: Category, minutes: &Integer, ctx: &Context) -> String {
        let mut output = String::new();
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        let timestamp = start.format("%Y-%m-%d %H:%M:%S").to_string();
        match reports::load(category, &ctx) {
            Report::History(history) => {
                format!("History since {} ", timestamp);
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
        Self::report(Category::History((Area::System(*id), *minutes)), minutes, ctx)
    }

    pub fn region(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::History((Area::Region(*id), *minutes)), minutes, ctx)
    }

    pub fn constellation(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::History((Area::Constellation(*id), *minutes)), minutes, ctx)
    }

    pub fn character_wins(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Wins((Actor::Character(*id), *minutes)), minutes, ctx)
    }

    pub fn character_losses(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Losses((Actor::Character(*id), *minutes)), minutes, ctx)
    }

    pub fn corporation_wins(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Wins((Actor::Corporation(*id), *minutes)), minutes, ctx)
    }

    pub fn corporation_losses(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Losses((Actor::Corporation(*id), *minutes)), minutes, ctx)
    }

    pub fn alliance_wins(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Wins((Actor::Alliance(*id), *minutes)), minutes, ctx)
    }

    pub fn alliance_losses(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Losses((Actor::Alliance(*id), *minutes)), minutes, ctx)
    }

    pub fn faction_wins(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Wins((Actor::Faction(*id), *minutes)), minutes, ctx)
    }

    pub fn faction_losses(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report(Category::Losses((Actor::Faction(*id), *minutes)), minutes, ctx)
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


