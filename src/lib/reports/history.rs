use crate::models::*;
use super::{zkb_href, link_killmail, Killmail};
use crate::services::Context;
use crate::services::server::root;
use crate::reports::FAIL;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct History {

    kills: Vec<Killmail>,
}
impl History {
    pub fn load(conn: &Connection, system: &Integer, minutes: &Integer) -> Self {
        use killmail::KillmailNamed;

        let ids = KillmailNamed::load_ids_for_last_minutes(conn, system, minutes).unwrap_or_default();
        let mut killmails = Vec::new();
        for id in &ids {
            if let Ok(killmail) = Killmail::load(conn, &id) {
                killmails.push(killmail);
            }
        }
        Self{ kills: killmails }
    }

    pub fn report(system: &Integer, minutes: &Integer, ctx: &Context) -> String {
        use crate::services::*;

        let mut output = String::new();
        let root = root(&ctx);
        ctx.database.push(Message::Load(Category::History((*system, *minutes))));
        while let Some(msg) = ctx.responses.pop() {
            if let Message::Report(Report::History(history)) = msg {
                for killmail in &history.kills{
                    killmail.write(&mut output, &root);
                }
            }
        }
        return output;
    }
}
impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for killmail in &self.kills {
            write!(f, "<div>{} : {}</div>",
                link_killmail(&killmail.killmail_id),
                zkb_href("kill", &Some(killmail.killmail_id), &Some(String::from("zkb")))
            )?;
        }
        writeln!(f, "")
    }
}
