use crate::models::*;
use crate::reports;
use crate::services::{Context, Message, Report, Area, Category};
use crate::services::server::root;
use chrono::{Duration, Utc};

#[derive(Debug, PartialEq)]
pub struct History;
impl History {
    fn report_impl(area: Area, minutes: &Integer, ctx: &Context) -> String {
        let mut output = String::new();
        let root = root(&ctx);
        let msg_id = crate::create_id().to_simple();
        let start = DateTime::from((Utc::now() - Duration::minutes(*minutes as i64)).naive_utc());
        super::div(&mut output, format!("History since {} ", start.time().format("%H:%M:%S").to_string()));

        ctx.database.push(Message::Find((msg_id, Category::History((area, *minutes)))));
        if let Report::History(history) = reports::wait_for(msg_id, &ctx) {
            for killmail in history {
                reports::Killmail::write(&mut output, &killmail, &root);
            }
        }
        return output;
    }

    pub fn system(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report_impl(Area::System(*id), minutes, ctx)
    }

    pub fn region(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report_impl(Area::Region(*id), minutes, ctx)
    }

    pub fn constellation(id: &Integer, minutes: &Integer, ctx: &Context) -> String {
        Self::report_impl(Area::Constellation(*id), minutes, ctx)
    }

}


